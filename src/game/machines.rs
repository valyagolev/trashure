use bevy::prelude::*;
use rand::prelude::Rng;
use strum::EnumDiscriminants;

use crate::graphics::{
    flyingvoxel::FlyingVoxel,
    machines::{
        radar::{consumption::RadarConsumer, Radar, RadarBundle, RadarType},
        targets::Target,
        BuiltMachine, MyMachine,
    },
    sceneobjectfinder::{SceneFoundObject, SceneObjectsFound},
    voxels3d::{lazyworld::LazyWorld, wholeworld::WholeBlockWorld, VoxelBlock, VOXEL_BLOCK_SIZE},
};

use super::{material::GameMaterial, voxelmailbox::VoxelMailbox, Direction2D};

pub struct MachinesPlugin;

impl Plugin for MachinesPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                // consume_radars,
                consume_mailbox,
                move_machines,
            ),
        );
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Reflect, EnumDiscriminants)]
#[strum_discriminants(derive(Reflect, Hash))]
pub enum GameMachineSettings {
    Recycler { recycling_radar: Entity },
    Plower { plowing_radar: Entity },
}

impl GameMachineSettings {
    pub fn instantiate(
        ghost: Entity,
        commands: &mut Commands,
        mc: &MyMachine,
        scob: &SceneObjectsFound,
        q_found_transforms: &Query<&Transform, With<SceneFoundObject>>,
    ) {
        let fuel_radar = commands
            .spawn((
                Name::new("fuel radar"),
                RadarBundle::new(
                    &[GameMaterial::Blueish],
                    None,
                    RadarConsumer {
                        flying_target: None,
                        // target_mailbox: None,
                        target_mailbox: Some(ghost),
                        paiload_ix: 3,
                    },
                    0.5,
                    10.0,
                    RadarType::Fuel,
                ),
                VoxelMailbox(default()),
            ))
            .id();

        commands.entity(ghost).add_child(fuel_radar);

        let set = match mc.gmt {
            GameMachineSettingsDiscriminants::Recycler => {
                let recycling_radar = commands
                    .spawn((
                        Name::new("recycling radar"),
                        RadarBundle::new(
                            GameMaterial::all(),
                            Some(Direction2D::Forward),
                            RadarConsumer {
                                flying_target: scob
                                    .0
                                    .get("RecyclingTarget")
                                    .and_then(|e| q_found_transforms.get(*e).ok())
                                    .map(|t| t.translation),
                                target_mailbox: Some(ghost),
                                paiload_ix: 1,
                            },
                            2.0,
                            6.0,
                            RadarType::Work,
                        ),
                    ))
                    .id();

                commands.entity(ghost).add_child(recycling_radar);

                GameMachineSettings::Recycler { recycling_radar }
            }
            GameMachineSettingsDiscriminants::Plower => {
                let plowing_radar = commands
                    .spawn((
                        Name::new("plowing radar"),
                        RadarBundle::new(
                            GameMaterial::all(),
                            Some(Direction2D::Forward),
                            RadarConsumer {
                                flying_target: None,
                                target_mailbox: Some(ghost),
                                paiload_ix: 2,
                            },
                            1.0,
                            3.0,
                            RadarType::Work,
                        ),
                    ))
                    .id();

                commands.entity(ghost).add_child(plowing_radar);

                commands
                    .entity(ghost)
                    .insert(Target::new(mc.pos + IVec2::new(10, 15)));

                GameMachineSettings::Plower { plowing_radar }
            }
        };

        commands.entity(ghost).insert(BuiltMachine {
            settings: set,
            fuel_radar,
        });
    }
}

fn consume_mailbox(
    mut commands: Commands,
    lazy_world: Res<LazyWorld>,
    mut q_machines: Query<(
        Entity,
        &mut VoxelMailbox,
        &BuiltMachine,
        &mut MyMachine,
        &Direction2D,
    )>,
    targets: Query<&Target>,
    q_blocks: Query<&VoxelBlock>,
    q_scene_object_finder: Query<&SceneObjectsFound>,
    q_scene_object_transforms: Query<&GlobalTransform, (Without<Radar>, Without<VoxelBlock>)>,
) {
    let rand = &mut rand::thread_rng();
    for (e, mut mailbox, bm, mut mm, dir) in q_machines.iter_mut() {
        let Some((_, mut vc, pl)) = mailbox.0.pop_front() else {
            continue;
        };

        if vc == GameMaterial::Blueish && mm.fuel < mm.max_fuel {
            mm.fuel += 1;
            continue;
        }

        match bm.settings {
            GameMachineSettings::Plower { .. } => {
                let target = targets.get(e).unwrap();
                let target = target.global_pos;

                let (block_pos, local_p) =
                    VoxelBlock::normalize_pos(IVec2::ZERO, target.extend(0).xzy());
                let block_e = lazy_world.known_parts[&block_pos];
                let block = q_blocks.get(block_e).unwrap();

                let y = if let Some(local_p) = block.empty_at_col(local_p.xz()) {
                    local_p.y + 3
                } else {
                    VOXEL_BLOCK_SIZE
                };

                commands.spawn(FlyingVoxel {
                    origin: mm.pos.extend(3).xzy().as_vec3(),
                    target: target.extend(y).xzy().as_vec3(),
                    target_mailbox: block_e,
                    material: vc,
                    payload: (target.extend(y).xzy(), 0),
                });
            }
            GameMachineSettings::Recycler { .. } => {
                // assert!(pl == 1);

                let rec_exit = q_scene_object_finder
                    .get(e)
                    .ok()
                    .and_then(|f| f.0.get("RecycledOrigin"))
                    .and_then(|e| q_scene_object_transforms.get(*e).ok())
                    .map(|t| t.translation());

                if vc == GameMaterial::Brownish {
                    if rand.gen_range(0..3) == 0 {
                        vc = GameMaterial::random_recycle(rand);
                    } else {
                        continue;
                    }
                }

                let back_dir = -dir;

                let mut found = None;

                for i in 1.. {
                    let target = mm.pos + back_dir.random_in_cone(i / 5 + 1, mm.dims, rand);

                    let (block_p, local_p) =
                        VoxelBlock::normalize_pos(IVec2::ZERO, target.extend(0).xzy());

                    let block_e = lazy_world.known_parts[&block_p];
                    let block = q_blocks.get(block_e).unwrap();

                    if let Some(local_p) = block.empty_at_col(local_p.xz()) {
                        found = Some((block_p, local_p, block_e));
                        break;
                    }
                }

                let (block_p, local_p, block_e) = found.unwrap();

                // println!("sending recycled to: {:?}", block_e);
                let tp = VoxelBlock::real_pos(block_p, local_p).as_ivec3() + IVec3::new(0, 3, 0);

                commands.spawn(FlyingVoxel {
                    origin: rec_exit.unwrap_or_else(|| mm.pos.extend(3).xzy().as_vec3()),
                    target: tp.as_vec3(),
                    target_mailbox: block_e,
                    material: vc,
                    payload: (tp, 0),
                });

                // let rp = VoxelBlock::real_pos(block_p, local_p);

                // debug3d::draw_gizmos(2.0, move |gizmos| {
                //     gizmos.sphere(rp, Quat::IDENTITY, 3.0, Color::RED);
                // });
            }
        }
    }
}

fn move_machines(
    lazy_world: Res<LazyWorld>,
    mut q_machines: Query<(
        // Entity,
        &BuiltMachine,
        &mut MyMachine,
        &Direction2D,
    )>,
    blocks: Query<'_, '_, &mut VoxelBlock, ()>,
) {
    let mut wbw = WholeBlockWorld { lazy_world, blocks };

    for (bm, mut mm, dir) in q_machines.iter_mut() {
        if matches!(bm.settings, GameMachineSettings::Recycler { .. }) {
            continue;
        }

        if mm.fuel < mm.max_fuel {
            continue;
        }

        mm.fuel -= mm.max_fuel;

        if !dir
            .line_in_direction(mm.pos, mm.dims)
            .any(|p| wbw.get_block_value(p.extend(0).xzy()).is_full())
        {
            mm.pos += Into::<IVec2>::into(*dir);
        }
    }
}
