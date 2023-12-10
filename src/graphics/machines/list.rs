use bevy::prelude::*;

use crate::graphics::{
    camera3d::CAMERA_OFFSET,
    gamemenu::{GameMenu, GameMenuState},
    selectable::CurrentlySelected,
};

use super::BuiltMachine;

pub struct MachineListPlugin;

impl Plugin for MachineListPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, Self::load_ui).add_systems(
            Update,
            (
                Self::add_ui_nodes,
                Self::redraw_ui_nodes,
                Self::handle_click,
            ),
        );
    }
}

#[derive(Component)]
struct MachineListUiRoot;

#[derive(Component)]
struct MachineListUiMachine(Entity);

impl MachineListPlugin {
    fn load_ui(mut commands: Commands) {
        commands
            .spawn((
                Name::new("Machine List UI Root"),
                MachineListUiRoot,
                NodeBundle {
                    background_color: BackgroundColor(Color::BLACK.with_a(0.5)),
                    style: Style {
                        position_type: PositionType::Absolute,

                        right: Val::Percent(1.),
                        top: Val::Auto,
                        bottom: Val::Percent(1.),
                        left: Val::Auto,

                        padding: UiRect::all(Val::Px(4.0)),
                        flex_direction: FlexDirection::Column,

                        align_items: AlignItems::FlexEnd,
                        ..Default::default()
                    },
                    ..Default::default()
                },
            ))
            .with_children(|parent| {
                parent.spawn(TextBundle {
                    text: Text::from_section("Machines", TextStyle { ..default() }),
                    ..Default::default()
                });
            });
    }

    fn add_ui_nodes(
        mut commands: Commands,
        q_root: Query<Entity, With<MachineListUiRoot>>,
        // q_existing_nodes: Query<&MachineListUiMachine>,
        q_machines: Query<(Entity, &Name), Added<BuiltMachine>>,
    ) {
        let root = q_root.single();
        // let existing = q_existing_nodes.iter().map(|e| e.0).collect_vec();

        for (ent, n) in q_machines.iter() {
            // if !existing.contains(&ent) {
            let node = commands
                .spawn((
                    MachineListUiMachine(ent),
                    ButtonBundle {
                        style: Style {
                            ..Default::default()
                        },
                        background_color: Color::rgba(0.0, 0.0, 0.0, 0.0).into(),
                        ..Default::default()
                    },
                ))
                .with_children(|parent| {
                    parent.spawn(TextBundle {
                        text: Text::from_sections([
                            TextSection::new(
                                n,
                                TextStyle {
                                    font_size: 18.0,
                                    color: Color::WHITE,
                                    ..default()
                                },
                            ),
                            TextSection::new(
                                "   ",
                                TextStyle {
                                    font_size: 18.0,
                                    color: Color::WHITE,
                                    ..default()
                                },
                            ),
                            TextSection::new(
                                "0",
                                TextStyle {
                                    font_size: 18.0,
                                    color: Color::rgb(0.8, 0.8, 0.8),
                                    ..default()
                                },
                            ),
                            TextSection::new(
                                "m from center",
                                TextStyle {
                                    font_size: 18.0,
                                    color: Color::rgb(0.8, 0.8, 0.8),
                                    ..default()
                                },
                            ),
                        ]),
                        ..Default::default()
                    });
                })
                .id();

            commands.entity(root).push_children(&[node]);
            // }
        }
    }

    fn redraw_ui_nodes(
        q_nodes: Query<(&MachineListUiMachine, &Children)>,
        mut q_text_nodes: Query<&mut Text, With<Parent>>,
        q_machines: Query<(Entity, &Name, &GlobalTransform), With<BuiltMachine>>,
        selected: Res<CurrentlySelected>,
    ) {
        for (mach, children) in q_nodes.iter() {
            let (ent, name, tr) = q_machines.get(mach.0).unwrap();

            for ch in children.iter() {
                let Ok(mut text) = q_text_nodes.get_mut(*ch) else {
                    continue;
                };

                text.sections[0].value = name.into();

                text.sections[2].value = format!("{:.1}", tr.translation().length());

                text.sections[0].style.color = if selected.0 == Some(ent) {
                    Color::GREEN
                } else {
                    Color::WHITE
                };
            }
        }
    }

    fn handle_click(
        q_nodes: Query<(&MachineListUiMachine, &Interaction)>,
        q_machines: Query<&GlobalTransform, With<BuiltMachine>>,
        mut selected: ResMut<CurrentlySelected>,
        mut menu_state: ResMut<GameMenu>,
        mut camera: Query<&mut Transform, (With<Camera3d>, Without<Parent>)>,
    ) {
        for (node, inter) in q_nodes.iter() {
            if *inter == Interaction::Pressed {
                selected.0 = Some(node.0);
                menu_state.0 = GameMenuState::SelectedMachine;

                let tr = q_machines.get(node.0).unwrap();
                let tr = Transform::from_translation(tr.translation() + CAMERA_OFFSET)
                    .looking_at(tr.translation(), Vec3::Y);

                for mut cam in camera.iter_mut() {
                    *cam = tr;
                }
            }
        }
    }
}
