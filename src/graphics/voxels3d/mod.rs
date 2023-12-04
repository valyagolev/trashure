use std::ops::{Index, IndexMut};

use crate::{conf::Configuration, game::material::GameMaterial};
use bevy::pbr::wireframe::Wireframe;
use bevy::render::mesh::MeshVertexAttribute;
use bevy::transform::commands;
use bevy::utils::HashSet;
use bevy::{prelude::*, render::camera::ScalingMode, utils::HashMap};
use bevy_meshem::{prelude::*, Dimensions};
use itertools::Itertools;
use rand::{seq::SliceRandom, Rng};

use self::voxel_mesh::generate_colored_voxel_mesh;

// mod meshem;
mod voxel_mesh;
pub mod voxel_physics;

pub const VOXEL_BLOCK_SIZE: i32 = 32;
const CHUNK_LEN: usize = (VOXEL_BLOCK_SIZE * VOXEL_BLOCK_SIZE * VOXEL_BLOCK_SIZE) as usize;

pub struct Voxels3dPlugin;
impl Plugin for Voxels3dPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup)
            .add_systems(Update, update_meshes)
            .add_plugins(voxel_physics::VoxelPhysics)
            .add_systems(Update, apply_changes)
            .insert_resource(VoxelBlockChanges::default());
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
    meshes: [Mesh; 4],
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

        VoxelMesh::NormalCube(&self.meshes[(*v as usize) - 1])
    }

    fn is_covering(&self, voxel: &Self::Voxel, side: prelude::Face) -> bool {
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
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.insert_resource(VoxelResources {
        meshes: [
            generate_colored_mesh(Color::rgb(0.8, 0.5, 0.4)),
            generate_colored_mesh(Color::rgb(0.5, 0.8, 0.4)),
            generate_colored_mesh(Color::rgb(0.4, 0.5, 0.8)),
            generate_colored_mesh(Color::rgb(0.8, 0.7, 0.6)),
        ],
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
        mesh: meshes.add(shape::Plane::from_size(1000.0).into()),
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
    let rand = &mut rand::thread_rng();

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
        &[Face::Bottom, Face::Back, Face::Left],
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
        assert!(self.forbidden_columns[local_pos.x as usize][local_pos.z as usize] == false);

        self.grid[idx] = Some(mat);
        self.meta.log(
            bevy_meshem::prelude::VoxelChange::Added,
            idx,
            Some(mat),
            self._meshem_neighbors(idx),
        )
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
        while inner_pos.y < 0 {
            voxel_block_pos.y -= 1;
            inner_pos.y += VOXEL_BLOCK_SIZE;
        }
        while inner_pos.x >= VOXEL_BLOCK_SIZE {
            voxel_block_pos.x += 1;
            inner_pos.x -= VOXEL_BLOCK_SIZE;
        }
        while inner_pos.y >= VOXEL_BLOCK_SIZE {
            voxel_block_pos.y += 1;
            inner_pos.y -= VOXEL_BLOCK_SIZE;
        }

        (voxel_block_pos, inner_pos)
    }

    pub fn forbid_column(&mut self, local_pos: IVec2) {
        for y in 0..VOXEL_BLOCK_SIZE {
            assert!(self[local_pos.extend(y).xzy()].is_none());
        }

        self.forbidden_columns[local_pos.x as usize][local_pos.y as usize] = true;
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

fn apply_changes(mut changes: ResMut<VoxelBlockChanges>, mut blocks: Query<&mut VoxelBlock>) {
    let rand = &mut rand::thread_rng();
    let mut new_changes = VoxelBlockChanges::default();

    for mut b in blocks.iter_mut() {
        if let Some(changes) = changes.added.remove(&b.pos) {
            for (pos, mat) in changes {
                b.push_block(pos, mat, &mut new_changes, rand)
            }
        }
    }

    for (pos, ch) in new_changes.added.drain() {
        changes.added.entry(pos).or_insert_with(Vec::new).extend(ch);
    }
}
