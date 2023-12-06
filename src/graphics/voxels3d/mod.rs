use std::{
    ops::{Index, IndexMut},
    time::Instant,
};

use crate::game::material::GameMaterial;

use bevy::{
    diagnostic::{
        Diagnostic, DiagnosticId, DiagnosticMeasurement, DiagnosticsStore, RegisterDiagnostic,
    },
    prelude::*,
    utils::HashMap,
};
use bevy_meshem::{prelude::*, Dimensions};

use self::voxel_mesh::generate_colored_voxel_mesh;
use uuid::uuid;

// mod meshem;
pub mod lazyworld;
mod voxel_mesh;
pub mod voxel_physics;

pub const VOXEL_BLOCK_SIZE: i32 = 32;
const CHUNK_LEN: usize = (VOXEL_BLOCK_SIZE * VOXEL_BLOCK_SIZE * VOXEL_BLOCK_SIZE) as usize;

pub const APPLIED_CHANGES: DiagnosticId =
    DiagnosticId(uuid!("a4a701b9-f1bc-4552-a9a0-7e0ec1a14bbc"));

pub const POSTPONED_CHANGES: DiagnosticId =
    DiagnosticId(uuid!("964e6a5f-ac48-4a25-a7d2-e02e3db9ac22"));

pub const CHANGED_BLOCKS: DiagnosticId =
    DiagnosticId(uuid!("f77ae6cc-032f-49e0-b089-2d8458c7f736"));

pub struct Voxels3dPlugin;
impl Plugin for Voxels3dPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup)
            .add_systems(Update, update_meshes)
            .add_plugins(voxel_physics::VoxelPhysics)
            .add_systems(Update, apply_changes)
            .insert_resource(VoxelBlockChanges::default())
            .register_diagnostic(Diagnostic::new(APPLIED_CHANGES, "applied_changes", 10))
            .register_diagnostic(Diagnostic::new(POSTPONED_CHANGES, "postponed_changes", 10))
            .register_diagnostic(Diagnostic::new(CHANGED_BLOCKS, "changed_blocks", 10));
    }
}

// #[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash, States)]
// pub enum VoxelPluginState {
//     #[default]
//     Setup,
//     Loaded,
//     Finished,
// }

#[derive(Debug, Resource, Reflect)]
pub struct VoxelResources {
    // pub mesh: Handle<Mesh>,
    // materials: [Handle<StandardMaterial>; 4],
    pub meshes: [Mesh; 5],
    pub material_handles: [Handle<StandardMaterial>; 5],
    pub voxel_material: Handle<StandardMaterial>,
    pub debug_voxel_material: Handle<StandardMaterial>,
}

#[derive(Component)]
pub struct VoxelBlock {
    pos: IVec2,
    meta: MeshMD<Option<GameMaterial>>,
    grid: [Option<GameMaterial>; CHUNK_LEN],
    mesh_id: AssetId<Mesh>,
    forbidden_columns: [[bool; VOXEL_BLOCK_SIZE as usize]; VOXEL_BLOCK_SIZE as usize],
}

impl VoxelRegistry for VoxelResources {
    type Voxel = Option<GameMaterial>;

    fn get_mesh(&self, voxel: &Self::Voxel) -> VoxelMesh<&Mesh> {
        let Some(v) = voxel else {
            return VoxelMesh::Null;
        };

        VoxelMesh::NormalCube(&self.meshes[v.as_usize()])
    }

    fn is_covering(&self, voxel: &Self::Voxel, _side: prelude::Face) -> bool {
        voxel.is_some()
    }

    fn get_center(&self) -> [f32; 3] {
        [0.0, 0.0, 0.0]
    }

    fn get_voxel_dimensions(&self) -> [f32; 3] {
        [1.0, 1.0, 1.0]
    }

    fn all_attributes(&self) -> Vec<bevy::render::mesh::MeshVertexAttribute> {
        vec![
            Mesh::ATTRIBUTE_POSITION,
            // Mesh::ATTRIBUTE_UV_0,
            Mesh::ATTRIBUTE_NORMAL,
            Mesh::ATTRIBUTE_COLOR,
        ]
    }
}

fn generate_colored_mesh(color: Color) -> Mesh {
    generate_colored_voxel_mesh(
        [1.0, 1.0, 1.0],
        [0.0, 0.0, 0.0],
        0.05,
        color.as_rgba_f32(),
        1.0,
    )
}

fn setup(
    mut commands: Commands,
    mut res_meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let handles = GameMaterial::all()
        .iter()
        .map(|m| {
            materials.add(StandardMaterial::from(
                // they're too bright compared to meshem i dunno why
                Into::<Color>::into(m) + Color::rgb(0.2, 0.2, 0.2),
            ))
        })
        .collect::<Vec<_>>()
        .try_into()
        .unwrap();

    commands.insert_resource(VoxelResources {
        meshes: GameMaterial::all()
            .iter()
            .map(|m| generate_colored_mesh(m.into()))
            .collect::<Vec<_>>()
            .try_into()
            .unwrap(),
        material_handles: handles,
        voxel_material: materials.add(StandardMaterial {
            // base_color: Color::LIME_GREEN,
            // alpha_mode: AlphaMode::Mask(0.5),
            // base_color_texture: Some(texture_mesh),
            ..default()
        }),
        debug_voxel_material: materials.add(StandardMaterial {
            base_color: Color::RED,
            // alpha_mode: AlphaMode::Mask(0.5),
            // base_color_texture: Some(texture_mesh),
            ..default()
        }),
    });

    // // let rand = &mut rand::thread_rng();
    // // plane
    commands.spawn(PbrBundle {
        mesh: res_meshes.add(shape::Plane::from_size(1000.0).into()),
        material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
        transform: Transform::from_xyz(0.0, -0.5, 0.0),
        ..default()
    });
    // // cubes
    // for row in 0..6 {
    //     let half_radius = 6 - row;

    //     for i in -half_radius..half_radius {
    //         for j in -half_radius..half_radius {
    //             let pos = Vec3::new(i as f32, row as f32, j as f32);

    //         }
    //     }
    // }
}

#[derive(Bundle)]
pub struct VoxelBlockBundle {
    pub voxel_block: VoxelBlock,
    pub pbr_bundle: PbrBundle,
    // pub wireframe: Wireframe,
}

pub fn generate_voxel_block(
    pos: IVec2,
    meshes: &mut ResMut<Assets<Mesh>>,
    voxel_resources: &Res<VoxelResources>,
) -> VoxelBlockBundle {
    let _rand = &mut rand::thread_rng();

    // let grid = (0..VOXEL_BLOCK_SIZE)
    //     .map(|x| {
    //         (0..VOXEL_BLOCK_SIZE)
    //             .map(|y| {
    //                 (0..VOXEL_BLOCK_SIZE)
    //                     .map(|z| {
    //                         if x <= 3 && rand.gen::<bool>() {
    //                             Some(GameMaterial::random(rand))
    //                         } else {
    //                             None
    //                         }
    //                     })
    //                     .collect_vec()
    //             })
    //             .collect_vec()
    //     })
    //     .flatten()
    //     .flatten()
    //     .collect_vec();
    let g: [_; CHUNK_LEN] = [None; CHUNK_LEN]; // grid.try_into().unwrap();
    let dims: Dimensions = (
        VOXEL_BLOCK_SIZE as usize,
        VOXEL_BLOCK_SIZE as usize,
        VOXEL_BLOCK_SIZE as usize,
    );
    // let texture_mesh = asset_server.load("array_texture.png");
    let (culled_mesh, metadata) = mesh_grid(
        dims,
        // &[Face::Bottom, Face::Back, Face::Left],
        &[],
        &g,
        voxel_resources.as_ref(), //.into_inner(),
        MeshingAlgorithm::Culling,
        None,
    )
    .unwrap();

    let culled_mesh_handle: Handle<Mesh> = meshes.add(culled_mesh.clone());

    VoxelBlockBundle {
        voxel_block: VoxelBlock {
            pos,
            meta: metadata,
            grid: g,
            mesh_id: culled_mesh_handle.id(),
            forbidden_columns: [[false; VOXEL_BLOCK_SIZE as usize]; VOXEL_BLOCK_SIZE as usize],
        },
        pbr_bundle: PbrBundle {
            mesh: culled_mesh_handle,
            material: voxel_resources.voxel_material.clone(),
            ..default()
        },
        // wireframe: Wireframe,
    }
}

impl VoxelBlock {
    const DIMENSIONS: Dimensions = (
        VOXEL_BLOCK_SIZE as usize,
        VOXEL_BLOCK_SIZE as usize,
        VOXEL_BLOCK_SIZE as usize,
    );

    fn _meshem_neighbors(&self, idx: usize) -> [Option<Option<GameMaterial>>; 6] {
        let mut r = [None; 6];
        for i in 0..6 {
            match get_neighbor(idx, Face::from(i), Self::DIMENSIONS) {
                None => {}
                Some(j) => r[i] = Some(self.grid[j]),
            }
        }
        r
    }

    pub fn _add_block(&mut self, local_pos: IVec3, mat: GameMaterial) {
        assert!(
            Self::within_bounds(local_pos),
            "local_pos out of bounds: {local_pos:?}"
        );
        // no idea why...
        let local_pos_fixed = local_pos.xzy();
        // dbg!(&local_pos);
        let idx = (local_pos_fixed.x
            + local_pos_fixed.y * VOXEL_BLOCK_SIZE
            + local_pos_fixed.z * VOXEL_BLOCK_SIZE * VOXEL_BLOCK_SIZE) as usize;
        // dbg!(&idx);

        // assert!(idx >= 0);

        assert!(self.grid[idx].is_none());
        assert!(!self.forbidden_columns[local_pos.x as usize][local_pos.z as usize]);

        self.grid[idx] = Some(mat);
        self.meta.log(
            bevy_meshem::prelude::VoxelChange::Added,
            idx,
            Some(mat),
            self._meshem_neighbors(idx),
        )
    }

    pub fn _take_block(&mut self, local_pos: IVec3) -> Option<GameMaterial> {
        assert!(
            Self::within_bounds(local_pos),
            "local_pos out of bounds: {local_pos:?}"
        );
        // no idea why...
        let local_pos_fixed = local_pos.xzy();
        // dbg!(&local_pos);
        let idx = (local_pos_fixed.x
            + local_pos_fixed.y * VOXEL_BLOCK_SIZE
            + local_pos_fixed.z * VOXEL_BLOCK_SIZE * VOXEL_BLOCK_SIZE) as usize;
        // dbg!(&idx);

        // assert!(idx >= 0);

        self.meta.log(
            bevy_meshem::prelude::VoxelChange::Broken,
            idx,
            None,
            self._meshem_neighbors(idx),
        );
        self.grid[idx].take()
    }

    pub fn within_bounds(pos: IVec3) -> bool {
        pos.x >= 0
            && pos.x < VOXEL_BLOCK_SIZE
            && pos.y >= 0
            && pos.y < VOXEL_BLOCK_SIZE
            && pos.z >= 0
            && pos.z < VOXEL_BLOCK_SIZE
    }

    pub fn normalize_pos(mut voxel_block_pos: IVec2, mut inner_pos: IVec3) -> (IVec2, IVec3) {
        while inner_pos.x < 0 {
            voxel_block_pos.x -= 1;
            inner_pos.x += VOXEL_BLOCK_SIZE;
        }
        while inner_pos.z < 0 {
            voxel_block_pos.y -= 1;
            inner_pos.z += VOXEL_BLOCK_SIZE;
        }
        while inner_pos.x >= VOXEL_BLOCK_SIZE {
            voxel_block_pos.x += 1;
            inner_pos.x -= VOXEL_BLOCK_SIZE;
        }
        while inner_pos.z >= VOXEL_BLOCK_SIZE {
            voxel_block_pos.y += 1;
            inner_pos.z -= VOXEL_BLOCK_SIZE;
        }

        (voxel_block_pos, inner_pos)
    }

    pub fn forbid_column(&mut self, local_pos: IVec2) {
        for y in 0..VOXEL_BLOCK_SIZE {
            assert!(self[local_pos.extend(y).xzy()].is_none());
        }

        self.forbidden_columns[local_pos.x as usize][local_pos.y as usize] = true;
    }

    pub fn real_pos(voxel_block_pos: IVec2, inner_pos: IVec3) -> Vec3 {
        ((voxel_block_pos * VOXEL_BLOCK_SIZE).extend(0) + inner_pos).as_vec3()
    }

    pub fn inner_pos(pos: Vec3) -> (IVec2, IVec3) {
        let voxel_block_pos = (pos.xz() / VOXEL_BLOCK_SIZE as f32).as_ivec2();
        let inner_pos = (pos.xz() % VOXEL_BLOCK_SIZE as f32)
            .as_ivec2()
            .extend(0)
            .xzy();

        (voxel_block_pos, inner_pos)
    }

    pub fn is_column_empty(&self, pos: IVec2) -> bool {
        assert!(pos.x >= 0 && pos.x < VOXEL_BLOCK_SIZE);

        self[pos.extend(0).xzy()].is_none()
    }

    /// pos in *local* coordinates
    pub fn closest_columns(&self, pos: IVec2, dist: f32) -> impl Iterator<Item = IVec2> {
        let dist_sq = dist * dist;
        let pos = pos.as_vec2();

        (0..VOXEL_BLOCK_SIZE)
            .map(|x| (0..VOXEL_BLOCK_SIZE).map(move |z| IVec2::new(x, z)))
            .flatten()
            .filter(move |col| {
                let dist = (col.as_vec2() - pos).length_squared();

                dist <= dist_sq
            })
    }

    pub fn material_in_col(
        &self,
        pos: IVec2,
        mask: u8,
    ) -> impl Iterator<Item = (GameMaterial, IVec3)> + '_ {
        (0..VOXEL_BLOCK_SIZE)
            .map(move |y| pos.extend(y).xzy())
            .filter_map(move |p| self[p].map(|mat| (mat, p)))
            .filter(move |(mat, _)| mat.mask_contains(mask))
    }
}

impl Index<IVec3> for VoxelBlock {
    type Output = Option<GameMaterial>;

    fn index(&self, index: IVec3) -> &Self::Output {
        let local_pos_fixed = index.xzy();

        let idx = (local_pos_fixed.x
            + local_pos_fixed.y * VOXEL_BLOCK_SIZE
            + local_pos_fixed.z * VOXEL_BLOCK_SIZE * VOXEL_BLOCK_SIZE) as usize;

        &self.grid[idx]
    }
}

// impl IndexMut<IVec3> for VoxelBlock {
//     fn index_mut(&mut self, index: IVec3) -> &mut Self::Output {
//         let local_pos_fixed = index.xzy();

//         let idx = (local_pos_fixed.x
//             + local_pos_fixed.y * VOXEL_BLOCK_SIZE
//             + local_pos_fixed.z * VOXEL_BLOCK_SIZE * VOXEL_BLOCK_SIZE) as usize;

//         &mut self.grid[idx]
//     }
// }

fn update_meshes(
    mut meshes: ResMut<Assets<Mesh>>,
    mut blocks: Query<&mut VoxelBlock, Changed<VoxelBlock>>,
    voxel_resources: Res<VoxelResources>,
) {
    let vr = voxel_resources.into_inner();

    for mut block in blocks.iter_mut() {
        let mesh = meshes.get_mut(block.mesh_id).unwrap();

        update_mesh(mesh, &mut block.meta, vr);
    }
}

#[derive(Resource, Default)]
pub struct VoxelBlockChanges {
    pub added: HashMap<IVec2, Vec<(IVec3, GameMaterial)>>,
}

impl VoxelBlockChanges {
    pub fn register_change(&mut self, voxel_block_pos: IVec2, inner_pos: IVec3, mat: GameMaterial) {
        let (voxel_block_pos, inner_pos) = VoxelBlock::normalize_pos(voxel_block_pos, inner_pos);

        self.added
            .entry(voxel_block_pos)
            .or_insert_with(Vec::new)
            .push((inner_pos, mat));
    }
}

fn apply_changes(
    mut changes: ResMut<VoxelBlockChanges>,
    mut blocks: Query<&mut VoxelBlock>,
    mut diagnostics: ResMut<DiagnosticsStore>,
) {
    let mut total_changes = 0;
    let mut total_postponed = 0;
    let mut changed_blocks = 0;

    let rand = &mut rand::thread_rng();
    let mut new_changes = VoxelBlockChanges::default();

    for mut b in blocks.iter_mut() {
        if let Some(changes) = changes.added.remove(&b.pos) {
            if !changes.is_empty() {
                changed_blocks += 1;
            }

            for (pos, mat) in changes {
                total_changes += 1;
                b.push_block(pos, mat, &mut new_changes, rand)
            }
        }
    }

    for (pos, ch) in new_changes.added.drain() {
        total_postponed += ch.len();
        changes.added.entry(pos).or_insert_with(Vec::new).extend(ch);
    }

    let measurements = [
        (APPLIED_CHANGES, total_changes),
        (POSTPONED_CHANGES, total_postponed),
        (CHANGED_BLOCKS, changed_blocks),
    ];

    for (diagnostic, value) in measurements.iter() {
        diagnostics
            .get_mut(*diagnostic)
            .unwrap()
            .add_measurement(DiagnosticMeasurement {
                time: Instant::now(),
                value: *value as f64,
            });
    }
}

#[cfg(test)]
mod test {
    use bevy::math::{IVec2, IVec3, Vec3};

    use crate::graphics::voxels3d::VoxelBlock;

    #[test]
    fn hmm() {
        dbg!(VoxelBlock::real_pos(IVec2::new(0, 0), IVec3::new(0, 0, 0)));
        dbg!(VoxelBlock::inner_pos(Vec3::new(0.0, 0.0, 0.0)));
        dbg!(VoxelBlock::real_pos(IVec2::new(0, 1), IVec3::new(0, 0, 0)));

        dbg!(VoxelBlock::real_pos(IVec2::new(0, 1), IVec3::new(1, 0, 1)));
    }
}
