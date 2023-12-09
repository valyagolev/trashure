use bevy::{core_pipeline::blit, prelude::*};
use rand::prelude::Rng;
use strum::EnumDiscriminants;

use crate::graphics::{
    debug3d,
    flyingvoxel::FlyingVoxel,
    machines::{
        radar::{Radar, RadarBundle},
        targets::Target,
        BuiltMachine, MyMachine,
    },
    voxels3d::{
        changes::VoxelBlockChanges, lazyworld::LazyWorld, wholeworld::WholeBlockWorld, VoxelBlock,
        VOXEL_BLOCK_SIZE,
    },
};

use super::{material::GameMaterial, voxelmailbox::VoxelMailbox, Direction2D};

pub struct MachinesPlugin;

impl Plugin for MachinesPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (consume_radars, consume_mailbox, move_machines));
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Reflect, EnumDiscriminants)]
#[strum_discriminants(derive(Reflect))]
pub enum GameMachineSettings {
    Recycler { recycling_radar: Entity },
    Plower { plowing_radar: Entity },
}

impl GameMachineSettings {
    pub fn instantiate(ghost: Entity, commands: &mut Commands, mc: &MyMachine) {
        let set = match mc.gmt {
            GameMachineSettingsDiscriminants::Recycler => {
                let recycling_radar = commands
                    .spawn((
                        Name::new("recycling radar"),
                        RadarBundle::new(GameMaterial::all(), Some(Direction2D::Forward)),
                    ))
                    .id();

                commands.entity(ghost).add_child(recycling_radar);

                GameMachineSettings::Recycler { recycling_radar }
            }
            GameMachineSettingsDiscriminants::Plower => {
                let plowing_radar = commands
                    .spawn((
                        Name::new("plowing radar"),
                        RadarBundle::new(GameMaterial::all(), Some(Direction2D::Forward)),
                    ))
                    .id();

                commands.entity(ghost).add_child(plowing_radar);

                commands
                    .entity(ghost)
                    .insert(Target::new(mc.pos + IVec2::new(10, 15)));

                GameMachineSettings::Plower { plowing_radar }
            }
        };

        commands.entity(ghost).insert(BuiltMachine(set));
    }
}

fn consume_radars(
    mut commands: Commands,
    lazy_world: Res<LazyWorld>,
    q_machines: Query<(Entity, &BuiltMachine, &MyMachine)>,
    mut q_radars: Query<&mut Radar>,
    q_blocks: Query<&mut VoxelBlock>,
    mut blockchanges: ResMut<VoxelBlockChanges>,
) {
    let mut whole_world = WholeBlockWorld {
        lazy_world: lazy_world,
        blocks: q_blocks,
    };

    let rand = &mut rand::thread_rng();
    for (e, m, mach) in q_machines.iter() {
        match m.0 {
            GameMachineSettings::Recycler { recycling_radar } => {
                consume_voxel(
                    &mut q_radars,
                    recycling_radar,
                    &mut whole_world,
                    &mut blockchanges,
                    rand,
                    &mut commands,
                    mach,
                    e,
                );
            }
            GameMachineSettings::Plower { plowing_radar } => {
                consume_voxel(
                    &mut q_radars,
                    plowing_radar,
                    &mut whole_world,
                    &mut blockchanges,
                    rand,
                    &mut commands,
                    mach,
                    e,
                );
            }
        }
    }
}

fn consume_voxel(
    q_radars: &mut Query<'_, '_, &mut Radar>,
    radar: Entity,
    whole_world: &mut WholeBlockWorld<'_, '_, '_, '_>,
    blockchanges: &mut ResMut<'_, VoxelBlockChanges>,
    rand: &mut rand::prelude::ThreadRng,
    commands: &mut Commands<'_, '_>,
    mach: &MyMachine,
    e: Entity,
) {
    let Ok(mut r) = q_radars.get_mut(radar) else {
        return;
    };

    let Some((_mat, vc)) = r.take_voxel() else {
        return;
    };

    let Some(mat) = whole_world.steal_block(vc, blockchanges, rand) else {
        return;
    };

    // todo: check the material?

    commands.spawn(FlyingVoxel {
        origin: vc,
        target: mach.pos.extend(3).xzy(),
        target_mailbox: e,
        material: mat,
        payload: 1, // 1==recycling
    });
}

fn consume_mailbox(
    mut commands: Commands,
    lazy_world: Res<LazyWorld>,
    mut q_machines: Query<(
        Entity,
        &mut VoxelMailbox,
        &BuiltMachine,
        &MyMachine,
        &Direction2D,
    )>,
    targets: Query<&Target>,
    q_blocks: Query<&VoxelBlock>,
) {
    let rand = &mut rand::thread_rng();
    for (e, mut mailbox, bm, mm, dir) in q_machines.iter_mut() {
        // println!("mailbox: {:?} {e:?}", mailbox.0);
        let Some((_, mut vc, pl)) = mailbox.0.pop_front() else {
            continue;
        };

        // println!("got mail:{} {:?} {}", mm.pos, vc, pl);

        match bm.0 {
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
                    origin: mm.pos.extend(3).xzy(),
                    target: target.extend(y).xzy(),
                    target_mailbox: block_e,
                    material: vc,
                    payload: 0,
                });
            }
            GameMachineSettings::Recycler { .. } => {
                assert!(pl == 1);

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

                commands.spawn(FlyingVoxel {
                    origin: mm.pos.extend(3).xzy(),
                    target: VoxelBlock::real_pos(block_p, local_p).as_ivec3() + IVec3::new(0, 3, 0),
                    target_mailbox: block_e,
                    material: vc,
                    payload: 0,
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
    let mut wbw = WholeBlockWorld {
        lazy_world: lazy_world,
        blocks,
    };

    for (bm, mut mm, dir) in q_machines.iter_mut() {
        if matches!(bm.0, GameMachineSettings::Recycler { .. }) {
            continue;
        }

        if !dir
            .line_in_direction(mm.pos, mm.dims)
            .any(|p| wbw.get_block_value(p.extend(0).xzy()).is_full())
        {
            mm.pos += Into::<IVec2>::into(*dir);
        }
    }
}
