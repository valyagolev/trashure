use bevy::{prelude::*, utils::HashSet};
use rand::Rng;

use crate::graphics::animated::MovingToPosition;

use super::{
    camera3d::CAMERA_OFFSET,
    voxels3d::{generate_voxel_block, VoxelResources, VOXEL_BLOCK_SIZE},
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
) {
    let center = (part * VOXEL_BLOCK_SIZE).extend(0).xzy();

    dbg!(&center);

    let mut bdl = generate_voxel_block(meshes, voxel_resources);
    bdl.0.transform = Transform::from_translation(center.as_vec3());

    if center == IVec3::ZERO {
        bdl.0.material = voxel_resources.debug_voxel_material.clone();
    }

    commands.spawn(bdl);

    // let half_chunk = VOXEL_BLOCK_SIZE / 2;
    // let rand = &mut rand::thread_rng();

    // for x in -half_chunk..half_chunk {
    //     for y in -half_chunk..half_chunk {
    //         let pos = center + IVec3::new(x, 0, y);
    //         let cnt = {
    //             if rand.gen_range(1..50) == 1 {
    //                 rand.gen_range(10..=40)
    //             } else {
    //                 rand.gen_range(1..=3)
    //             }
    //         };
    //         let cnt = 1;
    //         for _ in 0..cnt {
    //             // commands.spawn((
    //             //     Voxel {
    //             //         material: Material::random(rand),
    //             //         // shade: false,
    //             //         tight_at: None,
    //             //     },
    //             //     Transform::from_translation(pos.as_vec3()),
    //             //     MovingToPosition::new(pos, 40.0),
    //             // ));

    //             commands.spawn((
    //                 generate_voxel_block()
    //             ))
    //         }
    //     }
    // }
}

fn handle_camera(
    q_camera: Query<&GlobalTransform, With<Camera3d>>,
    mut lazy_world: ResMut<LazyWorld>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    voxel_resources: Res<VoxelResources>,
) {
    let camera = q_camera.single();

    let center = ((camera.translation() - CAMERA_OFFSET).xz() / VOXEL_BLOCK_SIZE as f32).as_ivec2();

    let all_around = AROUND_2D
        .iter()
        .map(|&offset| center + offset)
        .flat_map(|o| AROUND_2D.iter().map(move |offset| o + *offset));

    for part in all_around {
        // println!("Checking part {:?}", part);
        if !lazy_world.known_parts.contains(&part) {
            println!("Generating part {:?} around {center:?}", part);
            generate_part(&mut commands, part, &mut meshes, &voxel_resources);
            lazy_world.known_parts.insert(part);
            // break;
        }
        // break;
    }
}
