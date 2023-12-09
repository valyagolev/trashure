use bevy::{
    diagnostic::{
        Diagnostic, DiagnosticId, DiagnosticMeasurement, DiagnosticsStore, RegisterDiagnostic,
    },
    prelude::*,
    utils::{HashMap, Instant},
};
use rand::Rng;
use uuid::uuid;

use crate::{game::material::GameMaterial, graphics::camera3d::MainCamera};

use super::{
    super::camera3d::CAMERA_OFFSET,
    {generate_voxel_block, VoxelBlockChanges, VoxelResources, VOXEL_BLOCK_SIZE},
};

pub struct LazyWorldPlugin;

pub const WORLD_PARTS_DIAGNOSTIC: DiagnosticId =
    DiagnosticId(uuid!("71a16bb7-7b2a-4be4-9bad-ddbc591f42f5"));
pub const UNAPPLIED_CHANGES: DiagnosticId =
    DiagnosticId(uuid!("12a019af-d250-4d23-99b1-3079ee897d8f"));

impl Plugin for LazyWorldPlugin {
    fn build(&self, app: &mut App) {
        app.register_diagnostic(Diagnostic::new(WORLD_PARTS_DIAGNOSTIC, "world_parts", 1))
            .register_diagnostic(Diagnostic::new(UNAPPLIED_CHANGES, "unapplied_changes", 10))
            .insert_resource(LazyWorld {
                known_parts: HashMap::new(),
            })
            .add_systems(Update, handle_camera)
            .add_systems(Update, diagnostics);
    }
}

#[derive(Debug, Resource, Reflect)]
pub struct LazyWorld {
    pub known_parts: HashMap<IVec2, Entity>,
}

impl LazyWorld {
    pub fn lookup_around(
        &self,
        center: IVec2,
        radius: f32,
    ) -> impl Iterator<Item = (IVec2, Entity)> + '_ {
        // to bigblock-space
        // a bit more to handle close columns of far blocks
        let radius = radius / VOXEL_BLOCK_SIZE as f32 + 1.42;
        let radius2 = radius * radius;
        let center = center.as_vec2() / VOXEL_BLOCK_SIZE as f32;

        self.known_parts.iter().filter_map(move |(part, &entity)| {
            let dist = (part.as_vec2() - center).length_squared() as f32;
            if dist <= radius2 {
                Some((*part, entity))
            } else {
                None
            }
        })
    }
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
    changes: &mut VoxelBlockChanges,
) -> Entity {
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
                // if rand.gen_range(1..50) == 1 {
                //     rand.gen_range(80..=200)
                // } else {
                //     rand.gen_range(2..=3)
                // }

                rand.gen_range(0..=2)
            };
            // let cnt = 1;
            for _z in 0..cnt {
                let global_pos = center + pos;

                changes.register_change(
                    global_pos + IVec3::new(0, VOXEL_BLOCK_SIZE, 0),
                    GameMaterial::random(rand),
                );
            }
        }
    }

    commands.spawn(bundle).id()
}

#[derive(Component)]
pub struct WorldGenTrigger(pub Vec2);

fn handle_camera(
    // q_camera: Query<&GlobalTransform, With<MainCamera>>,
    q_trigger: Query<(&WorldGenTrigger, &GlobalTransform)>,
    mut lazy_world: ResMut<LazyWorld>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    voxel_resources: Res<VoxelResources>,
    mut blockchanges: ResMut<VoxelBlockChanges>,
) {
    // let camera = q_camera.single();

    for (trigger, trans) in q_trigger.iter() {
        // let center = ((camera.translation() - CAMERA_OFFSET).xz() / VOXEL_BLOCK_SIZE as f32).as_ivec2();
        let center = ((trans.translation().xz() - trigger.0) / VOXEL_BLOCK_SIZE as f32).as_ivec2();

        let all_around = AROUND_2D
        .iter()
        .map(|&offset| center + offset)
        .flat_map(|o| AROUND_2D.iter().map(move |offset| o + *offset))
        // .flat_map(|o| AROUND_2D.iter().map(move |offset| o + *offset))
        ;

        for part in all_around {
            // println!("Checking part {:?}", part);
            if !lazy_world.known_parts.contains_key(&part) {
                // println!("Generating part {:?} around {center:?}", part);

                lazy_world.known_parts.insert(
                    part,
                    generate_part(
                        &mut commands,
                        part,
                        &mut meshes,
                        &voxel_resources,
                        &mut blockchanges,
                    ),
                );
                // break;
            }
            // break;
        }
    }
}

fn diagnostics(
    mut diagnostics: ResMut<DiagnosticsStore>,
    lazy_world: ResMut<LazyWorld>,
    blockchanges: ResMut<VoxelBlockChanges>,
) {
    diagnostics
        .get_mut(WORLD_PARTS_DIAGNOSTIC)
        .unwrap()
        .add_measurement(DiagnosticMeasurement {
            time: Instant::now(),
            value: lazy_world.known_parts.len() as f64,
        });

    diagnostics
        .get_mut(UNAPPLIED_CHANGES)
        .unwrap()
        .add_measurement(DiagnosticMeasurement {
            time: Instant::now(),
            value: blockchanges.added.values().map(|v| v.len()).sum::<usize>() as f64,
        });
}
