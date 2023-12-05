use bevy::prelude::*;

use crate::{
    game::Direction2D,
    graphics::{
        cursor::CursorOver, dbgtext::DebugTexts, lazyworld::LazyWorld, recolor::Tinted,
        voxels3d::VoxelBlock,
    },
};

use super::{colors::MachineRecolor, BuiltMachine, MachineType, MyMachine};

pub struct MachinesBuildingPlugin;

impl Plugin for MachinesBuildingPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (debug_setup, move_ghost, check_placement))
            .insert_resource(MachineGhost(None));
    }
}

#[derive(Debug, Resource, Reflect)]
pub struct MachineGhost(pub Option<Entity>);

fn debug_setup(
    mut commands: Commands,
    mut ghost: ResMut<MachineGhost>,
    q_types: Query<Entity, With<MachineType>>,
    // mut q_messages: ResMut<DebugTexts>,
    cursor: Res<CursorOver>,
) {
    if ghost.0.is_none() {
        let Some(t) = q_types.iter().next() else {
            return;
        };

        let tp = commands
            .spawn((
                Tinted::from(MachineRecolor::Ghost.into()),
                // BuiltMachine,
                MyMachine {
                    tp: t,
                    pos: cursor.block.xz(),
                    direction: Direction2D::Backward,
                },
            ))
            .id();

        ghost.0 = Some(tp);
    }
}

fn move_ghost(
    mut ghost: ResMut<MachineGhost>,
    mut q_machines: Query<&mut MyMachine, Without<BuiltMachine>>,
    cursor: Res<CursorOver>,
    keyb: Res<Input<KeyCode>>,
) {
    let Some(ghost) = ghost.0 else {
        return;
    };
    let Ok(mut m) = q_machines.get_mut(ghost) else {
        return;
    };

    m.pos = cursor.block.xz();

    if keyb.just_released(KeyCode::R) {
        m.direction = m.direction.rotate();
    }
}

fn check_placement(
    ghost: Res<MachineGhost>,
    mut q_machines: Query<(&MyMachine, &mut Tinted), (Changed<MyMachine>, Without<BuiltMachine>)>,

    lazyworld: Res<LazyWorld>,
    blocks: Query<&VoxelBlock>,
    q_types: Query<&MachineType>,
) {
    let Some(ghost_e) = ghost.0 else {
        return;
    };
    let Ok((ghost, mut tinted)) = q_machines.get_mut(ghost_e) else {
        return;
    };
    let Ok(mt) = q_types.get(ghost.tp) else {
        return;
    };

    let center = mt.dims / 2;

    let mut bad = false;

    'outer: for x in 0..mt.dims.x {
        for z in 0..mt.dims.y {
            let pos = ghost.pos + ghost.direction.rotate_size(IVec2::new(x, z) - center);

            let (block_i, inner) = VoxelBlock::normalize_pos(IVec2::ZERO, pos.extend(0).xzy());

            let Ok(block) = blocks.get(lazyworld.known_parts[&block_i]) else {
                continue;
            };

            dbg!((pos, block_i, inner));

            if block[inner].is_some() {
                println!("{pos:?}");
                bad = true;
                break 'outer;
            }
        }
    }

    if !bad {
        println!("good");
    }

    *tinted = if bad {
        MachineRecolor::ForbiddenGhost.into()
    } else {
        MachineRecolor::Ghost.into()
    };
}
