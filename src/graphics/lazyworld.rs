use bevy::{prelude::*, utils::HashSet};
use rand::Rng;

use crate::graphics::{
    animated::MovingToPosition,
    pieces::{Material, Voxel},
};

pub struct LazyWorldPlugin;

const CHUNK_SIZE: i32 = 100;
// const CAMERA_RADIUS: f32 = 50.0;

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
    IVec2::new(-1, -1),
    IVec2::new(-1, 0),
    IVec2::new(-1, 1),
    IVec2::new(0, -1),
    IVec2::new(0, 0),
    IVec2::new(0, 1),
    IVec2::new(1, -1),
    IVec2::new(1, 0),
    IVec2::new(1, 1),
];

fn generate_part(commands: &mut Commands, part: IVec2) {
    let center = (part * CHUNK_SIZE).extend(0).xzy();

    let half_chunk = CHUNK_SIZE / 2;
    let rand = &mut rand::thread_rng();

    for x in -half_chunk..half_chunk {
        for y in -half_chunk..half_chunk {
            let pos = center + IVec3::new(x, 0, y);
            let cnt = {
                if rand.gen_range(1..50) == 1 {
                    rand.gen_range(10..=40)
                } else {
                    rand.gen_range(1..=3)
                }
            };
            // let cnt = 1;
            for _ in 0..cnt {
                commands.spawn((
                    Voxel {
                        material: Material::random(rand),
                        // shade: false,
                        tight_at: None,
                    },
                    Transform::from_translation(pos.as_vec3()),
                    MovingToPosition::new(pos, 40.0),
                ));
            }
        }
    }
}

fn handle_camera(
    q_camera: Query<&GlobalTransform, With<Camera3d>>,
    mut lazy_world: ResMut<LazyWorld>,
    mut commands: Commands,
) {
    let camera = q_camera.single();

    let center = (camera.translation().xz() / CHUNK_SIZE as f32).as_ivec2();

    let all_around = AROUND_2D.iter().map(|&offset| center + offset);

    for part in all_around {
        // println!("Checking part {:?}", part);
        if !lazy_world.known_parts.contains(&part) {
            println!("Generating part {:?} around {center:?}", part);
            generate_part(&mut commands, part);
            lazy_world.known_parts.insert(part);
        }
    }
}
