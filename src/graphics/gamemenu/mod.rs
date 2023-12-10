use bevy::prelude::*;

use self::textref::{QueryTexts, TextRefs};

use super::{
    cursor::CursorOver,
    machines::{building::MachineGhost, MachineResources, MachineType, MyMachine},
    selectable::CurrentlySelected,
};

pub struct GameMenuPlugin;
mod looks;
pub mod textref;
pub mod tutorial;

impl Plugin for GameMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(tutorial::TutorialPlugin)
            .insert_resource(GameMenu(GameMenuState::ToPickBuilding))
            .add_systems(Update, looks::setup_menu)
            .add_systems(
                Update,
                (
                    handle_build_click,
                    redraw_menu,
                    redraw_tabs,
                    handle_tabs_click,
                ),
            )
            .register_type::<GameMenuButton>()
            .register_type::<GameMenuState>();
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Reflect)]
pub enum GameMenuState {
    ToPickBuilding,
    CurrentlyCreating,
    SelectedMachine,
}

#[derive(Resource, Deref)]
pub struct GameMenu(pub GameMenuState);

#[derive(Component)]
struct LeftBottomUiNode;

#[derive(Component)]
struct TutorialNode;

#[derive(Component)]
struct GameMenuNode;

#[derive(Component, Deref)]
struct GameMenuPart(GameMenuState);

#[derive(Component, Deref)]
struct GameMenuToPickBuildingForMachineButton(Entity);

#[derive(Component, Deref, Reflect)]
pub struct GameMenuButton(GameMenuState);

fn handle_build_click(
    mut commands: Commands,
    q_interaction: Query<
        (&GameMenuToPickBuildingForMachineButton, &Interaction),
        Changed<Interaction>,
    >,
    q_types: Query<&MachineType>,
    mut menustate: ResMut<GameMenu>,
    mut ghost: ResMut<MachineGhost>,
    cursor: Res<CursorOver>,
    machres: Res<MachineResources>,
) {
    for (entity, interaction) in q_interaction.iter() {
        if *interaction == Interaction::Pressed {
            let tp = q_types.get(entity.0).unwrap();

            println!("Clicked on {:?}", tp.name);

            menustate.0 = GameMenuState::CurrentlyCreating;

            *ghost = MachineGhost::start(entity.0, &mut commands, &cursor, tp, &machres);
        }
    }
}

fn redraw_menu(
    menu_state: Res<GameMenu>,
    mut q_menu_parts: Query<(&mut Visibility, &GameMenuPart, Option<&TextRefs>)>,
    ghost: Res<MachineGhost>,
    q_types: Query<&MachineType>,
    selected: Res<CurrentlySelected>,
    q_machines: Query<(&MyMachine, &Name)>,
    mut q_texts: QueryTexts,
) {
    let state = menu_state.0;
    let mut textref = None;

    for (mut vis, part, textrefs) in q_menu_parts.iter_mut() {
        if part.0 == state {
            *vis = Visibility::Visible;
            textref = textrefs;
        } else {
            *vis = Visibility::Hidden;
        }
    }

    match state {
        GameMenuState::CurrentlyCreating => {
            let textref = textref.unwrap();

            let Some((tp, _)) = ghost.0 else {
                // text.sections[1].value = "(bug: No machine selected.)".into();
                return;
            };

            let tp = q_types.get(tp).unwrap();

            textref.update(&mut q_texts, "name", tp.name.to_string(), None);
        }
        GameMenuState::SelectedMachine => {
            let textref = textref.unwrap();

            let Some(tp) = selected.0 else {
                // text.sections[1].value = "(bug: No machine selected.)".into();
                return;
            };

            let (mm, name) = q_machines.get(tp).unwrap();

            textref.update(&mut q_texts, "name", name.to_string(), None);
            textref.update(
                &mut q_texts,
                "fuel",
                format!("{}/{}", mm.fuel, mm.max_fuel),
                None,
            );
            textref.update(
                &mut q_texts,
                "maintenance",
                format!("{}", mm.needed_maintenance),
                None,
            );
            textref.update(
                &mut q_texts,
                "build",
                format!("{}", mm.still_building),
                None,
            );
        }
        _ => {}
    }
}

fn redraw_tabs(
    menu_state: Res<GameMenu>,
    mut q_menu_buttons: Query<(&GameMenuButton, &mut BackgroundColor, &Children)>,
    mut q_menu_button_texts: Query<&mut Text>,
    selected: Res<CurrentlySelected>,
    q_machines: Query<&Name, With<MyMachine>>,
) {
    for (button, mut color, children) in q_menu_buttons.iter_mut() {
        if button.0 == menu_state.0 {
            color.0 = Color::BLACK.with_a(0.8);
        } else {
            color.0 = Color::WHITE;
        }

        for child in children.iter() {
            if let Ok(mut text) = q_menu_button_texts.get_mut(*child) {
                if button.0 == menu_state.0 {
                    text.sections[0].style.color = Color::WHITE;
                } else {
                    text.sections[0].style.color = Color::RED;
                }

                if button.0 == GameMenuState::SelectedMachine {
                    if let Some(selected) = selected.0 {
                        let selected_name = q_machines
                            .get(selected)
                            .map(|name| name.as_str())
                            .unwrap_or("Nameless");

                        text.sections[0].value = format!("Selected: {}", selected_name);
                    } else {
                        text.sections[0].value = "Selected: None".into();
                    }
                }
            }
        }
    }
}

fn handle_tabs_click(
    mut menu_state: ResMut<GameMenu>,
    mut q_menu_buttons: Query<(&GameMenuButton, &Interaction)>,
    mut selected: ResMut<CurrentlySelected>,
) {
    let Some(clicked_state) = q_menu_buttons
        .iter_mut()
        .find(|(_, interaction)| **interaction == Interaction::Pressed)
        .map(|(button, _)| button.0)
    else {
        return;
    };

    match clicked_state {
        GameMenuState::CurrentlyCreating => todo!(),
        GameMenuState::ToPickBuilding => {
            selected.0 = None;
            menu_state.0 = GameMenuState::ToPickBuilding;
        }
        GameMenuState::SelectedMachine => {
            if selected.0.is_none() {
                return;
            }

            menu_state.0 = GameMenuState::SelectedMachine;
        }
    }
}
