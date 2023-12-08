use std::{
    fs::Metadata,
    ops::{Index, IndexMut},
    time::Instant,
};

use crate::game::{material::GameMaterial, voxelmailbox::VoxelMailbox};

use bevy::{
    diagnostic::{
        Diagnostic, DiagnosticId, DiagnosticMeasurement, DiagnosticsStore, RegisterDiagnostic,
    },
    prelude::*,
    utils::HashMap,
};
use bevy_meshem::{prelude::*, Dimensions};

use self::{
    changes::{apply_changes, VoxelBlockChanges},
    voxel_mesh::generate_colored_voxel_mesh,
};
use uuid::uuid;

// mod meshem;
pub mod blocks;
pub mod lazyworld;
mod voxel_mesh;
pub mod voxel_physics;
pub use blocks::VoxelBlock;
pub mod changes;
pub mod wholeworld;

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
            .add_systems(Update, (apply_changes, consume_mailbox))
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
    pub meshes: [Mesh; 4],
    pub material_handles: [Handle<StandardMaterial>; 4],
    pub voxel_material: Handle<StandardMaterial>,
    pub debug_voxel_material: Handle<StandardMaterial>,
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
        // [0.5, 0.5, 0.5]
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
    pub mailbox: VoxelMailbox,
}

pub fn generate_mesh_grid(
    voxel_resources: &Res<VoxelResources>,
    grid: &[Option<GameMaterial>; 32768],
) -> (Mesh, MeshMD<Option<GameMaterial>>) {
    let dims: Dimensions = (
        VOXEL_BLOCK_SIZE as usize,
        VOXEL_BLOCK_SIZE as usize,
        VOXEL_BLOCK_SIZE as usize,
    );

    mesh_grid(
        dims,
        // &[Face::Bottom, Face::Back, Face::Left],
        &[],
        grid,
        voxel_resources.as_ref(), //.into_inner(),
        MeshingAlgorithm::Culling,
        // MeshingAlgorithm::Naive,
        None,
    )
    .unwrap()
}

pub fn generate_voxel_block(
    pos: IVec2,
    meshes: &mut ResMut<Assets<Mesh>>,
    voxel_resources: &Res<VoxelResources>,
) -> VoxelBlockBundle {
    let _rand = &mut rand::thread_rng();

    let g: [_; CHUNK_LEN] = [None; CHUNK_LEN]; // grid.try_into().unwrap();

    // let texture_mesh = asset_server.load("array_texture.png");
    let (culled_mesh, metadata) = generate_mesh_grid(voxel_resources, &g);

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
        mailbox: VoxelMailbox::default(),
        // wireframe: Wireframe,
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
    mut blocks: Query<(&mut VoxelBlock, &mut Handle<Mesh>), Changed<VoxelBlock>>,
    voxel_resources: Res<VoxelResources>,
) {
    let vr = voxel_resources.into_inner();

    for (mut block, mut mesh) in blocks.iter_mut() {
        let Some(mesh) = meshes.get_mut(block.mesh_id) else {
            continue;
        };

        update_mesh(mesh, &mut block.meta, vr);

        // let (culled_mesh, metadata) = generate_mesh_grid(&voxel_resources, &block.grid);
        // let culled_mesh_handle: Handle<Mesh> = meshes.add(culled_mesh.clone());

        // block.meta = metadata;
        // block.mesh_id = culled_mesh_handle.id();

        // *mesh = culled_mesh_handle;
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

fn consume_mailbox(
    mut q_machines: Query<&mut VoxelMailbox, With<VoxelBlock>>,
    mut changes: ResMut<VoxelBlockChanges>,
) {
    for mut mailbox in q_machines.iter_mut() {
        let Some((target, vc, _)) = mailbox.0.pop_front() else {
            continue;
        };

        changes.register_change(target, vc);
    }
}
