use bevy::{prelude::*, utils::Instant};

use crate::{
    game::{material::GameMaterial, Direction2D},
    graphics::{
        cursor::CursorOver,
        gamemenu::{GameMenu, GameMenuState},
        recolor::Tinted,
        selectable::CurrentlySelected,
        voxels3d::lazyworld::LazyWorld,
        voxels3d::VoxelBlock,
    },
};

use super::{colors::MachineRecolor, radar::Radar, BuiltMachine, MachineType, MyMachine};

pub struct MachinesBuildingPlugin;

impl Plugin for MachinesBuildingPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                // debug_setup,
                move_ghost,
                check_placement,
                place_ghost.after(check_placement),
            ),
        )
        .insert_resource(MachineGhost(
            None,
            false,
            Instant::now() + std::time::Duration::from_millis(200),
        ));
    }
}

#[derive(Debug, Resource, Reflect)]
/// MachineType, MyMachine
pub struct MachineGhost(pub Option<(Entity, Entity)>, pub bool, pub Instant);

impl MachineGhost {
    pub fn start(
        tp: Entity,
        commands: &mut Commands,
        cursor: &Res<CursorOver>,
        machine_type: &MachineType,
    ) -> Self {
        let ent = commands
            .spawn::<(Tinted, _)>((
                MachineRecolor::Ghost.into(),
                // BuiltMachine,
                MyMachine {
                    tp,
                    gmt: machine_type.gmt,
                    dims: machine_type.dims,
                    pos: cursor.block.xz(),
                    direction: Direction2D::Backward,
                },
            ))
            .id();

        Self(
            Some((tp, ent)),
            false,
            Instant::now() + std::time::Duration::from_millis(200),
        )
    }
}

// fn debug_setup(
//     mut commands: Commands,
//     mut ghost: ResMut<MachineGhost>,
//     q_types: Query<Entity, With<MachineType>>,
//     // mut q_messages: ResMut<DebugTexts>,
//     cursor: Res<CursorOver>,
// ) {
//     if ghost.0.is_none() {
//         let Some(t) = q_types.iter().next() else {
//             return;
//         };

//         let tp = commands
//             .spawn((
//                 Tinted::from(MachineRecolor::Ghost.into()),
//                 // BuiltMachine,
//                 MyMachine {
//                     tp: t,
//                     pos: cursor.block.xz(),
//                     direction: Direction2D::Backward,
//                 },
//             ))
//             .id();

//         ghost.0 = Some((t, tp));
//     }
// }

fn move_ghost(
    ghost: ResMut<MachineGhost>,
    mut q_machines: Query<&mut MyMachine, Without<BuiltMachine>>,
    cursor: Res<CursorOver>,
    keyb: Res<Input<KeyCode>>,
) {
    let Some((_, ghost)) = ghost.0 else {
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

fn place_ghost(
    mut commands: Commands,
    mut mghost: ResMut<MachineGhost>,
    // mut q_machines: Query<&mut MyMachine, Without<BuiltMachine>>,
    cursor: Res<Input<MouseButton>>, // keyb: Res<Input<KeyCode>>,

    mut selected: ResMut<CurrentlySelected>,
    mut menu_state: ResMut<GameMenu>,
) {
    if !mghost.1 {
        return;
    }

    if mghost.2 > Instant::now() {
        return;
    }

    let Some((_tp, ghost)) = mghost.0 else {
        return;
    };

    if cursor.just_released(MouseButton::Left) {
        commands.entity(ghost).insert((
            BuiltMachine,
            Tinted::empty(),
            Radar::new(&[GameMaterial::Brownish]),
        ));
        mghost.0 = None;

        selected.0 = Some(ghost);
        menu_state.0 = GameMenuState::SelectedMachine;
    }
}

fn check_placement(
    mut mghost: ResMut<MachineGhost>,
    mut q_machines: Query<(&MyMachine, &mut Tinted), (Changed<MyMachine>, Without<BuiltMachine>)>,

    q_existing_machines: Query<&MyMachine, With<BuiltMachine>>,

    lazyworld: Res<LazyWorld>,
    blocks: Query<&VoxelBlock>,
    q_types: Query<&MachineType>,
) {
    let Some((mt_e, ghost_e)) = mghost.0 else {
        return;
    };
    let Ok((ghost, mut tinted)) = q_machines.get_mut(ghost_e) else {
        return;
    };
    let Ok(mt) = q_types.get(mt_e) else {
        return;
    };

    let center = mt.dims / 2;

    let mut bad = q_existing_machines.iter().any(|m| m.intersects(ghost));

    if !bad {
        'outer: for x in 0..mt.dims.x {
            for z in 0..mt.dims.y {
                let pos = ghost.pos + ghost.direction.rotate_size(IVec2::new(x, z) - center);

                let (block_i, inner) = VoxelBlock::normalize_pos(IVec2::ZERO, pos.extend(0).xzy());

                let Ok(block) = blocks.get(lazyworld.known_parts[&block_i]) else {
                    continue;
                };

                if block[inner].is_some() {
                    bad = true;
                    break 'outer;
                }
            }
        }
    }

    *tinted = if bad {
        MachineRecolor::ForbiddenGhost.into()
    } else {
        MachineRecolor::Ghost.into()
    };

    mghost.1 = !bad;
}
