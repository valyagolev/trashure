use bevy::prelude::*;

use crate::graphics::{
    gamemenu::{
        GameMenuNode, GameMenuToPickBuildingForMachineButton, LeftBottomUiNode, TutorialNode,
    },
    machines::MachineType,
};

use super::{GameMenuPart, GameMenuState};

pub fn setup_menu(
    mut commands: Commands,
    q_mtypes: Query<(Entity, &MachineType)>,
    mut already: Local<bool>,
) {
    if q_mtypes.iter().next().is_none() {
        return;
    }

    if *already {
        return;
    }

    *already = true;

    println!("Setting up menu");
    let ui_root = commands
        .spawn((
            LeftBottomUiNode,
            NodeBundle {
                // z_index: ZIndex::Global(i32::MAX),
                style: Style {
                    position_type: PositionType::Absolute,

                    right: Val::Auto,
                    top: Val::Auto,
                    bottom: Val::Percent(1.),
                    left: Val::Percent(1.),
                    // give it some padding for readability
                    padding: UiRect::all(Val::Px(4.0)),
                    flex_direction: FlexDirection::Column,
                    // align_content: AlignContent::FlexEnd,
                    align_items: AlignItems::FlexStart,
                    ..Default::default()
                },
                ..Default::default()
            },
        ))
        .id();

    let tutorial_root = make_tutorial_root(&mut commands);

    let menu_root = commands
        .spawn((
            GameMenuNode,
            NodeBundle {
                background_color: BackgroundColor(Color::BLACK.with_a(0.8)),
                z_index: ZIndex::Global(i32::MAX),
                style: Style {
                    margin: UiRect::all(Val::Px(4.0)),
                    padding: UiRect::all(Val::Px(4.0)),
                    flex_direction: FlexDirection::Column,
                    align_items: AlignItems::FlexStart,
                    ..Default::default()
                },
                ..Default::default()
            },
        ))
        .id();

    let currently_creating_text = currently_creating(&mut commands);
    let tpbm = to_pick_building_menu(&mut commands, q_mtypes);

    let selected_building_text = selected_building(&mut commands);

    commands.entity(menu_root).push_children(&[
        currently_creating_text,
        tpbm,
        selected_building_text,
    ]);

    commands
        .entity(ui_root)
        .push_children(&[tutorial_root, menu_root]);
}

fn currently_creating(commands: &mut Commands<'_, '_>) -> Entity {
    let currently_creating_text = commands
        .spawn((
            GameMenuPart(GameMenuState::CurrentlyCreating),
            TextBundle {
                visibility: Visibility::Hidden,
                text: Text::from_sections([
                    TextSection {
                        // Currently building: {}. Press R to rotate.
                        value: "Currently building: ".into(),
                        style: TextStyle {
                            font_size: 20.0,
                            color: Color::WHITE,
                            ..default()
                        },
                    },
                    TextSection {
                        value: "?".into(),
                        style: TextStyle {
                            font_size: 20.0,
                            color: Color::BLUE,
                            ..default()
                        },
                    },
                    TextSection {
                        value: " Press R to rotate.".into(),
                        style: TextStyle {
                            font_size: 20.0,
                            color: Color::WHITE,
                            ..default()
                        },
                    },
                ]),
                ..Default::default()
            },
        ))
        .id();
    currently_creating_text
}

fn selected_building(commands: &mut Commands<'_, '_>) -> Entity {
    let selected_building_text = commands
        .spawn((
            GameMenuPart(GameMenuState::SelectedMachine),
            TextBundle {
                visibility: Visibility::Hidden,
                text: Text::from_sections([
                    TextSection {
                        // Currently building: {}. Press R to rotate.
                        value: "Selected: ".into(),
                        style: TextStyle {
                            font_size: 20.0,
                            color: Color::WHITE,
                            ..default()
                        },
                    },
                    TextSection {
                        value: "?".into(),
                        style: TextStyle {
                            font_size: 20.0,
                            color: Color::BLUE,
                            ..default()
                        },
                    },
                    TextSection {
                        value: " Press R to rotate.".into(),
                        style: TextStyle {
                            font_size: 20.0,
                            color: Color::WHITE,
                            ..default()
                        },
                    },
                ]),
                ..Default::default()
            },
        ))
        .id();
    selected_building_text
}

fn make_tutorial_root(commands: &mut Commands<'_, '_>) -> Entity {
    let tutorial_root = commands
        .spawn((
            GameMenuNode,
            NodeBundle {
                background_color: BackgroundColor(Color::BLACK.with_a(0.8)),
                // z_index: ZIndex::Global(i32::MAX),
                style: Style {
                    margin: UiRect::all(Val::Px(4.0)),
                    padding: UiRect::all(Val::Px(4.0)),
                    flex_direction: FlexDirection::Column,
                    align_items: AlignItems::FlexStart,
                    ..Default::default()
                },
                ..Default::default()
            },
        ))
        .id();

    let text = commands
        .spawn((
            TutorialNode,
            TextBundle {
                text: Text::from_sections([TextSection {
                    value: "tutorial".into(),
                    style: TextStyle {
                        font_size: 20.0,
                        color: Color::WHITE,
                        ..default()
                    },
                }]),
                ..Default::default()
            },
        ))
        .id();

    commands.entity(tutorial_root).push_children(&[text]);
    tutorial_root
}

fn to_pick_building_menu(
    commands: &mut Commands,
    q_mtypes: Query<(Entity, &MachineType)>,
) -> Entity {
    let root = commands
        .spawn((
            GameMenuPart(GameMenuState::ToPickBuilding),
            NodeBundle {
                style: Style {
                    margin: UiRect::all(Val::Px(4.0)),
                    padding: UiRect::all(Val::Px(4.0)),
                    flex_direction: FlexDirection::Column,
                    align_items: AlignItems::FlexStart,
                    ..Default::default()
                },
                ..Default::default()
            },
        ))
        .with_children(|commands| {
            commands.spawn((TextBundle {
                text: Text::from_sections([TextSection {
                    value: "Pick a building to build".into(),
                    style: TextStyle {
                        font_size: 20.0,
                        color: Color::WHITE,
                        ..default()
                    },
                }]),
                ..Default::default()
            },));

            for (e, tp) in q_mtypes.iter() {
                println!("Adding button for {:?}", tp.name);
                commands
                    .spawn((
                        GameMenuToPickBuildingForMachineButton(e),
                        ButtonBundle {
                            style: Style {
                                margin: UiRect::top(Val::Px(4.0)),
                                padding: UiRect::all(Val::Px(4.0)),
                                ..Default::default()
                            },
                            ..Default::default()
                        },
                    ))
                    .with_children(|c| {
                        c.spawn(TextBundle {
                            text: Text::from_sections([TextSection {
                                value: tp.name.to_string(),
                                style: TextStyle {
                                    font_size: 20.0,
                                    color: Color::RED,
                                    ..default()
                                },
                            }]),
                            ..Default::default()
                        });
                    });
            }
        })
        .id();

    root
}
