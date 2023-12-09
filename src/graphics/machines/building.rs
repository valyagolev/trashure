use bevy::{
    prelude::*,
    render::view::RenderLayers,
    utils::{HashMap, Instant},
};

use crate::{
    game::{
        machines::{GameMachineSettings, GameMachineSettingsDiscriminants},
        voxelmailbox::VoxelMailbox,
        Direction2D,
    },
    graphics::{
        cursor::CursorOver,
        gamemenu::{GameMenu, GameMenuState},
        recolor::Tinted,
        sceneobjectfinder::{SceneFoundObject, SceneObjectFinder, SceneObjectsFound},
        scenerenderlayer::SceneRenderLayers,
        selectable::{CurrentlySelected, Selectable},
        voxels3d::lazyworld::{LazyWorld, WorldGenTrigger},
        voxels3d::VoxelBlock,
    },
};

use super::{colors::MachineRecolor, BuiltMachine, MachineType, MyMachine};

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
                handle_esc,
            ),
        )
        .insert_resource(MachineGhost(
            None,
            false,
            Instant::now() + std::time::Duration::from_millis(200),
        ))
        .insert_resource(MachineCounter(HashMap::default()));
    }
}

#[derive(Debug, Resource, Reflect)]
/// MachineType, MyMachine
pub struct MachineGhost(pub Option<(Entity, Entity)>, pub bool, pub Instant);

#[derive(Resource)]
pub struct MachineCounter(pub HashMap<GameMachineSettingsDiscriminants, usize>);

impl MachineGhost {
    pub fn start(
        tp: Entity,
        commands: &mut Commands,
        cursor: &Res<CursorOver>,
        machine_type: &MachineType,
    ) -> Self {
        let ent = commands
            .spawn((
                Name::new(format!("{} Ghost", machine_type.name)),
                VoxelMailbox(default()),
                Into::<Tinted>::into(MachineRecolor::Ghost),
                WorldGenTrigger(Vec2::ZERO),
                // BuiltMachine,
                MyMachine {
                    tp,
                    gmt: machine_type.gmt,
                    dims: machine_type.dims,
                    pos: cursor.block.xz(),
                    // direction: ,
                },
                Direction2D::Backward,
                SceneObjectFinder::new(["RecycledOrigin", "RecyclingTarget"]),
            ))
            .id();

        Self(
            Some((tp, ent)),
            false,
            Instant::now() + std::time::Duration::from_millis(200),
        )
    }
}

fn move_ghost(
    ghost: ResMut<MachineGhost>,
    mut q_machines: Query<(&mut MyMachine, &mut Direction2D), Without<BuiltMachine>>,
    cursor: Res<CursorOver>,
    keyb: Res<Input<KeyCode>>,
) {
    let Some((_, ghost)) = ghost.0 else {
        return;
    };
    let Ok((mut m, mut dir)) = q_machines.get_mut(ghost) else {
        return;
    };

    m.pos = cursor.block.xz();

    if keyb.just_released(KeyCode::R) {
        *dir = dir.rotate();
    }
}

fn place_ghost(
    mut commands: Commands,
    mut mghost: ResMut<MachineGhost>,
    q_machines: Query<(&MyMachine, &SceneObjectsFound), Without<BuiltMachine>>,
    cursor: Res<Input<MouseButton>>, // keyb: Res<Input<KeyCode>>,

    mut selected: ResMut<CurrentlySelected>,
    mut menu_state: ResMut<GameMenu>,

    q_found_transforms: Query<&Transform, With<SceneFoundObject>>,

    mut machine_counter: ResMut<MachineCounter>,
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

    let Ok((m, scob)) = q_machines.get(ghost) else {
        return;
    };

    if cursor.just_released(MouseButton::Left) {
        let v = machine_counter
            .0
            .entry(m.gmt)
            .and_modify(|c| *c += 1)
            .or_insert(1);

        commands.entity(ghost).insert((
            Name::new(format!("{:?} ({})", m.gmt, v)),
            Tinted::empty(),
            VisibilityBundle::default(),
            Selectable,
            SceneRenderLayers(RenderLayers::default().with(6)),
        ));

        GameMachineSettings::instantiate(ghost, &mut commands, m, scob, &q_found_transforms);

        mghost.0 = None;

        selected.0 = Some(ghost);
        menu_state.0 = GameMenuState::SelectedMachine;
    }
}

fn check_placement(
    mut mghost: ResMut<MachineGhost>,
    mut q_machines: Query<
        (&MyMachine, &mut Tinted, &Direction2D),
        (Changed<MyMachine>, Without<BuiltMachine>),
    >,

    q_existing_machines: Query<(&MyMachine, &Direction2D), With<BuiltMachine>>,

    lazyworld: Res<LazyWorld>,
    blocks: Query<&VoxelBlock>,
    q_types: Query<&MachineType>,
) {
    let Some((mt_e, ghost_e)) = mghost.0 else {
        return;
    };
    let Ok((ghost, mut tinted, dir)) = q_machines.get_mut(ghost_e) else {
        return;
    };
    let Ok(mt) = q_types.get(mt_e) else {
        return;
    };

    let center = mt.dims / 2;

    let mut bad = q_existing_machines
        .iter()
        .any(|(m, mdir)| m.intersects(*mdir, ghost, *dir));

    if !bad {
        'outer: for x in 0..mt.dims.x {
            for z in 0..mt.dims.y {
                let pos = ghost.pos + dir.rotate_size(IVec2::new(x, z) - center);

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

fn handle_esc(
    mut commands: Commands,
    keyboard: Res<Input<KeyCode>>,
    mut currently_building: ResMut<MachineGhost>,
    mut selected: ResMut<CurrentlySelected>,
    mut menu_state: ResMut<GameMenu>,
) {
    if keyboard.just_pressed(KeyCode::Escape) {
        if let Some((_, ghost)) = currently_building.0 {
            commands.entity(ghost).despawn_recursive();
        }

        currently_building.0 = None;
        selected.0 = None;
        menu_state.0 = GameMenuState::ToPickBuilding;
    }
}
