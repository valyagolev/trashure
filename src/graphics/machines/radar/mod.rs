use bevy::{prelude::*, time::Stopwatch};
use rand::{seq::SliceRandom, Rng};

use crate::{
    game::{material::GameMaterial, Direction2D},
    graphics::voxels3d::{lazyworld::LazyWorld, VoxelBlock, VOXEL_BLOCK_SIZE},
};

use self::consumption::RadarConsumer;

use super::MyMachine;

pub mod consumption;
mod graphics;
pub struct RadarPlugin;

impl Plugin for RadarPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            consumption::RadarConsumptionPlugin,
            graphics::RadarGraphicsPlugin,
        ))
        .add_systems(FixedUpdate, radar_search)
        .add_event::<RadarFoundVoxel>();
    }
}

#[derive(Bundle)]
pub struct RadarBundle {
    radar: Radar,
    transform_bundle: TransformBundle,
    visibility_bundle: VisibilityBundle,
    radar_consumer: RadarConsumer,
}

impl RadarBundle {
    pub fn new(
        mats: &[GameMaterial],
        direction: Option<Direction2D>,
        radar_consumer: RadarConsumer,
    ) -> Self {
        RadarBundle {
            radar: Radar::new(mats, direction),
            transform_bundle: TransformBundle::default(),
            visibility_bundle: VisibilityBundle::default(),
            radar_consumer,
        }
    }
}

#[derive(Event)]
pub struct RadarFoundVoxel {
    pub radar: Entity,
    pub material: GameMaterial,
    pub pos: IVec3,
}

#[derive(Component)]
pub struct Radar {
    material_mask: u8,
    watch: Stopwatch,
    scene: Option<Entity>,

    pub direction: Option<Direction2D>,
    pub paused: bool,
}

#[derive(Component)]
pub struct RadarScene;

impl Radar {
    pub fn new(mats: &[GameMaterial], direction: Option<Direction2D>) -> Self {
        Radar {
            material_mask: GameMaterial::any_of_mask(mats),
            watch: Stopwatch::new(),
            scene: None,
            direction,
            paused: false,
        }
    }

    fn dist(&self) -> f32 {
        // 30.0 * ((self.watch.elapsed().as_secs_f32() / 5.0).sin()).abs()
        self.watch.elapsed().as_secs_f32() * 5.0 * 6.0
    }
}

fn radar_search(
    mut found_events: EventWriter<RadarFoundVoxel>,
    time: Res<Time>,
    mut q_radars: Query<(Entity, &mut Radar, &Parent, &GlobalTransform)>,
    q_parent_machines: Query<(&Direction2D, &MyMachine), With<Children>>,
    lazyworld: Res<LazyWorld>,
    q_blocks: Query<&VoxelBlock>,
) {
    let rand = &mut rand::thread_rng();

    for (e, mut r, rpar, gt) in q_radars.iter_mut() {
        if r.paused {
            continue;
        }

        let (rpardir, machine) = q_parent_machines.get(**rpar).unwrap();

        r.watch.tick(time.delta());

        let dist = r.dist();

        // make it more interesting lmao
        let dist = dist + rand.gen_range(0.0..3.0);

        let radar_ipos = gt.translation().xz().as_ivec2();

        let mut candidates = vec![];

        for (bigblock_pos, ent) in lazyworld.lookup_around(radar_ipos, dist) {
            let Ok(voxel_block) = q_blocks.get(ent) else {
                continue;
            };

            let local_pos = radar_ipos - bigblock_pos * VOXEL_BLOCK_SIZE;

            for col in voxel_block.closest_columns(local_pos, dist) {
                for (mat, pos) in voxel_block.material_in_col(col, r.material_mask) {
                    if let Some(dir) = r.direction {
                        let radar_local_pos = (bigblock_pos * VOXEL_BLOCK_SIZE) - radar_ipos + col;

                        if !(dir * *rpardir).within_cone(radar_local_pos, machine.dims) {
                            continue;
                        }
                    }

                    let full_pos = (bigblock_pos * VOXEL_BLOCK_SIZE).extend(0).xzy() + pos;

                    candidates.push((mat, full_pos));
                }
            }
        }

        if !candidates.is_empty() {
            let winner = candidates.choose(rand).unwrap();

            found_events.send(RadarFoundVoxel {
                radar: e,
                material: winner.0,
                pos: winner.1,
            });

            r.watch.reset();
        }
    }
}
