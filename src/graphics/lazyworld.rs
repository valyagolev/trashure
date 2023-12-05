use bevy::{prelude::*, utils::HashSet};
use rand::Rng;

use crate::{game::material::GameMaterial, graphics::animated::MovingToPosition};

use super::{
    camera3d::CAMERA_OFFSET,
    voxels3d::{generate_voxel_block, VoxelBlockChanges, VoxelResources, VOXEL_BLOCK_SIZE},
};

pub struct LazyWorldPlugin;

impl Plugin for LazyWorldPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(LazyWorld {
            known_parts: HashSet::new(),
        })
        .add_systems(Update, handle_camera);
    }
}

#[derive(Debug, Resource, Reflect)]
struct LazyWorld {
    known_parts: HashSet<IVec2>,
}

static AROUND_2D: &[IVec2] = &[
    IVec2::new(0, 0),
    IVec2::new(-1, -1),
    IVec2::new(-1, 0),
    IVec2::new(-1, 1),
    IVec2::new(0, -1),
    IVec2::new(0, 1),
    IVec2::new(1, -1),
    IVec2::new(1, 0),
    IVec2::new(1, 1),
];

fn generate_part(
    commands: &mut Commands,
    part: IVec2,
    meshes: &mut ResMut<Assets<Mesh>>,
    voxel_resources: &Res<VoxelResources>,
    mut changes: &mut VoxelBlockChanges,
) {
    let center = (part * VOXEL_BLOCK_SIZE).extend(0).xzy();

    // dbg!(&center);

    let mut bundle = generate_voxel_block(part, meshes, voxel_resources);
    bundle.pbr_bundle.transform = Transform::from_translation(center.as_vec3());

    if center == IVec3::ZERO {
        //     bdl.1.material = voxel_resources.debug_voxel_material.clone();

        for x in 0..15 {
            for z in 0..15 {
                bundle.voxel_block.forbid_column(IVec2::new(x, z));
            }
        }
    }

    let half_chunk = VOXEL_BLOCK_SIZE / 2;
    let rand = &mut rand::thread_rng();

    for x in -half_chunk..half_chunk {
        for z in -half_chunk..half_chunk {
            let pos = IVec3::new(x, 0, z) + IVec3::new(half_chunk, 0, half_chunk);

            let cnt = {
                if rand.gen_range(1..50) == 1 {
                    rand.gen_range(80..=200)
                } else {
                    rand.gen_range(2..=3)
                }
            };
            // let cnt = 1;
            for z in 0..cnt {
                // vb._add_block(pos + IVec3::new(0, z, 0), GameMaterial::random(rand))
                bundle.voxel_block.drop_block(
                    pos.xz(),
                    GameMaterial::random(rand),
                    &mut changes,
                    rand,
                );
            }
        }
    }

    commands.spawn(bundle);
}

fn handle_camera(
    q_camera: Query<&GlobalTransform, With<Camera3d>>,
    mut lazy_world: ResMut<LazyWorld>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    voxel_resources: Res<VoxelResources>,
    mut blockchanges: ResMut<VoxelBlockChanges>,
) {
    let camera = q_camera.single();

    let center = ((camera.translation() - CAMERA_OFFSET).xz() / VOXEL_BLOCK_SIZE as f32).as_ivec2();

    let all_around = AROUND_2D
        .iter()
        .map(|&offset| center + offset)
        .flat_map(|o| AROUND_2D.iter().map(move |offset| o + *offset))
        // .flat_map(|o| AROUND_2D.iter().map(move |offset| o + *offset))
        ;

    for part in all_around {
        // println!("Checking part {:?}", part);
        if !lazy_world.known_parts.contains(&part) {
            println!("Generating part {:?} around {center:?}", part);
            generate_part(
                &mut commands,
                part,
                &mut meshes,
                &voxel_resources,
                &mut blockchanges,
            );
            lazy_world.known_parts.insert(part);
            // break;
        }
        // break;
    }
}
