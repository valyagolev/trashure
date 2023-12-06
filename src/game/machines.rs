use bevy::prelude::*;
use strum::EnumDiscriminants;

use crate::graphics::{
    flyingvoxel::FlyingVoxel,
    machines::{radar::Radar, BuiltMachine, MyMachine},
    voxels3d::{lazyworld::LazyWorld, VoxelBlock, VoxelBlockChanges},
};

use super::{material::GameMaterial, Direction2D};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Reflect, EnumDiscriminants)]
#[strum_discriminants(derive(Reflect))]
pub enum GameMachineSettings {
    Recycler { recycling_radar: Entity },
}

impl GameMachineSettings {
    pub fn instantiate(ghost: Entity, commands: &mut Commands, mc: &MyMachine) {
        let set = match mc.gmt {
            GameMachineSettingsDiscriminants::Recycler => {
                let recycling_radar = commands
                    .spawn((
                        Radar::new(GameMaterial::all(), Some(Direction2D::Forward)),
                        TransformBundle::default(),
                    ))
                    .id();

                commands.entity(ghost).add_child(recycling_radar);

                GameMachineSettings::Recycler { recycling_radar }
            }
        };

        commands.entity(ghost).insert(BuiltMachine(set));
    }
}

pub struct MachinesPlugin;

impl Plugin for MachinesPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, consume_radars);
    }
}

pub fn consume_radars(
    mut commands: Commands,
    lazy_world: Res<LazyWorld>,
    q_machines: Query<(Entity, &BuiltMachine, &MyMachine)>,
    mut q_radars: Query<&mut Radar>,
    mut q_blocks: Query<&mut VoxelBlock>,
    mut blockchanges: ResMut<VoxelBlockChanges>,
) {
    let rand = &mut rand::thread_rng();
    for (e, m, mach) in q_machines.iter() {
        match m.0 {
            GameMachineSettings::Recycler { recycling_radar } => {
                let Ok(mut r) = q_radars.get_mut(recycling_radar) else {
                    continue;
                };

                let Some((mat, vc)) = r.take_voxel() else {
                    continue;
                };

                let (gp, lp) = VoxelBlock::normalize_pos(IVec2::ZERO, vc);

                let mut block = q_blocks.get_mut(lazy_world.known_parts[&gp]).unwrap();

                let mat = block.steal_block(lp, &mut blockchanges, rand);

                commands.spawn(FlyingVoxel {
                    origin: vc,
                    target: mach.pos.extend(3).xzy(),
                    target_mailbox: e,
                    material: mat,
                });
            }
        }
    }
}
