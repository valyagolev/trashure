use std::hash::BuildHasher;

use bevy::{
    prelude::*,
    render::view::RenderLayers,
    utils::{HashMap, Instant},
};

use crate::{
    game::{
        machines::{GameMachineSettings, GameMachineSettingsDiscriminants},
        material::GameMaterial,
        voxelmailbox::VoxelMailbox,
        Direction2D,
    },
    graphics::{
        cursor::CursorOver,
        gamemenu::{tutorial::mark_tutorial_event, GameMenu, GameMenuState},
        recolor::Tinted,
        sceneobjectfinder::{SceneFoundObject, SceneObjectFinder, SceneObjectsFound},
        scenerenderlayer::SceneRenderLayers,
        selectable::{CurrentlySelected, Selectable},
        voxels3d::lazyworld::{LazyWorld, WorldGenTrigger},
        voxels3d::VoxelBlock,
    },
};

use super::{
    colors::MachineRecolor,
    radar::{consumption::RadarConsumer, RadarBundle, RadarType},
    BuiltMachine, MachineResources, MachineType, MyMachine,
};

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
                finish_building,
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

#[derive(Component)]
pub struct GhostMachineFloor;

impl MachineGhost {
    pub fn start(
        tp: Entity,
        commands: &mut Commands,
        cursor: &Res<CursorOver>,
        machine_type: &MachineType,
        machine_res: &Res<MachineResources>,
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
                    fuel: 0,
                    max_fuel: machine_type.max_fuel,
                    needed_maintenance: 0,
                    still_building: 20,
                    useful_ish_work_done: 0.0,
                    last_slow_work: None,
                },
                Direction2D::Backward,
                SceneObjectFinder::new(["RecycledOrigin", "RecyclingTarget"]),
                VisibilityBundle::default(),
                TransformBundle::default(),
            ))
            .with_children(|b| {
                b.spawn((
                    GhostMachineFloor,
                    PbrBundle {
                        mesh: machine_res.floor.clone(),
                        transform: Transform::from_scale(Vec3::new(
                            machine_type.dims.x as f32,
                            1.0,
                            machine_type.dims.y as f32,
                        )),
                        material: machine_res.white_floor.clone(),
                        ..Default::default()
                    },
                ));
            })
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
    q_machines: Query<(&MyMachine, &Children), Without<BuiltMachine>>,
    cursor: Res<Input<MouseButton>>, // keyb: Res<Input<KeyCode>>,

    mut selected: ResMut<CurrentlySelected>,
    mut menu_state: ResMut<GameMenu>,

    mut machine_counter: ResMut<MachineCounter>,
    q_floors: Query<Entity, With<GhostMachineFloor>>,
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

    let Ok((m, children)) = q_machines.get(ghost) else {
        return;
    };

    if cursor.just_released(MouseButton::Left) {
        if m.gmt == GameMachineSettingsDiscriminants::Recycler {
            mark_tutorial_event("recycler_placed");
        }

        let v = machine_counter
            .0
            .entry(m.gmt)
            .and_modify(|c| *c += 1)
            .or_insert(1);

        commands.entity(ghost).insert((
            Name::new(format!("{:?} ({})", m.gmt, v)),
            Tinted::new(Color::rgb(0.0, 0.1, 0.0)),
            VisibilityBundle::default(),
            Selectable,
            SceneRenderLayers(
                RenderLayers::default(), // .with(6)
            ),
        ));

        let build_radar = commands
            .spawn((
                Name::new("build radar"),
                RadarBundle::new(
                    &[GameMaterial::Greenish],
                    None,
                    RadarConsumer {
                        flying_target: None,
                        // target_mailbox: None,
                        target_mailbox: Some(ghost),
                    },
                    2.0,
                    10.0,
                    RadarType::Building,
                ),
                // VoxelMailbox(default()),
            ))
            .id();

        commands.entity(ghost).push_children(&[build_radar]);

        q_floors
            .iter_many(children)
            .for_each(|e| commands.entity(e).despawn_recursive());

        mghost.0 = None;

        // selected.0 = Some(ghost);
        // menu_state.0 = GameMenuState::SelectedMachine;
        selected.0 = None;
        menu_state.0 = GameMenuState::ToPickBuilding;
    }
}

fn finish_building(
    mut commands: Commands,
    q_machines: Query<(Entity, &MyMachine, &SceneObjectsFound), Without<BuiltMachine>>,

    q_found_transforms: Query<&Transform, With<SceneFoundObject>>,

    q_types: Query<&MachineType>,
) {
    for (ghost, mm, scob) in q_machines.iter() {
        let Ok(mt) = q_types.get(mm.tp) else {
            continue;
        };

        if mm.still_building == 0 {
            if mm.gmt == GameMachineSettingsDiscriminants::Recycler {
                mark_tutorial_event("recycler_finished");
            }

            if mm.gmt == GameMachineSettingsDiscriminants::Plower {
                mark_tutorial_event("plower_built");
            }

            commands.entity(ghost).insert((Tinted::empty(),));

            GameMachineSettings::instantiate(
                ghost,
                &mut commands,
                mm,
                scob,
                &q_found_transforms,
                &mt,
            );
        }
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
