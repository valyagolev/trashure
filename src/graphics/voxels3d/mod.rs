use crate::{conf::Configuration, game::material::GameMaterial};
use bevy::render::mesh::MeshVertexAttribute;
use bevy::transform::commands;
use bevy::{prelude::*, render::camera::ScalingMode, utils::HashMap};
use bevy_meshem::{prelude::*, Dimensions};
use itertools::Itertools;
use rand::{seq::SliceRandom, Rng};

use self::voxel_mesh::generate_colored_voxel_mesh;

// mod meshem;
mod voxel_mesh;

pub const VOXEL_BLOCK_SIZE: i32 = 32;
const CHUNK_LEN: usize = (VOXEL_BLOCK_SIZE * VOXEL_BLOCK_SIZE * VOXEL_BLOCK_SIZE) as usize;

pub struct Voxels3dPlugin;
impl Plugin for Voxels3dPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup)
            // .add_systems(OnEnter(VoxelPluginState::Loaded), generate_meshes)
            ;
        // .add_systems(Update, camera_setup)
        // .add_systems(Update, handle_camera_move);
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
    meta: MeshMD<Option<GameMaterial>>,
    grid: [Option<GameMaterial>; CHUNK_LEN],
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

pub fn generate_voxel_block(
    meshes: &mut ResMut<Assets<Mesh>>,
    voxel_resources: &Res<VoxelResources>,
) -> (PbrBundle, VoxelBlock) {
    let rand = &mut rand::thread_rng();

    // let grid = (0..CHUNK_LEN)
    //     .map(|_| {
    //         if rand.gen::<bool>() {
    //             Some(GameMaterial::random(rand))
    //         } else {
    //             None
    //         }
    //     })
    //     .collect_vec();

    let grid = (0..VOXEL_BLOCK_SIZE)
        .map(|x| {
            (0..VOXEL_BLOCK_SIZE)
                .map(|y| {
                    (0..VOXEL_BLOCK_SIZE)
                        .map(|z| {
                            if x <= 3 && rand.gen::<bool>() {
                                Some(GameMaterial::random(rand))
                            } else {
                                None
                            }
                        })
                        .collect_vec()
                })
                .collect_vec()
        })
        .flatten()
        .flatten()
        .collect_vec();
    let g: [_; CHUNK_LEN] = grid.try_into().unwrap();
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

    (
        PbrBundle {
            mesh: culled_mesh_handle,
            material: voxel_resources.voxel_material.clone(),
            ..default()
        },
        VoxelBlock {
            meta: metadata,
            grid: g,
        },
    )
}
