use bevy::prelude::*;

use super::{
    cursor::CursorOver,
    machines::{building::MachineGhost, MachineType},
};

pub struct GameMenuPlugin;
mod looks;
impl Plugin for GameMenuPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(GameMenu(GameMenuState::ToPickBuilding))
            .add_systems(Update, looks::setup_menu)
            .add_systems(Update, (handle_build_click, redraw_menu));
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GameMenuState {
    ToPickBuilding,
    CurrentlyCreating,
    SelectedMachine,
}

#[derive(Resource)]
struct GameMenu(GameMenuState);

#[derive(Component)]
struct LeftBottomUiNode;

#[derive(Component)]
struct TutorialNode;

#[derive(Component)]
struct GameMenuNode;

#[derive(Component)]
struct GameMenuPart(GameMenuState);

#[derive(Component)]
struct GameMenuToPickBuildingForMachineButton(Entity);

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
) {
    for (entity, interaction) in q_interaction.iter() {
        if *interaction == Interaction::Pressed {
            let tp = q_types.get(entity.0).unwrap();

            println!("Clicked on {:?}", tp.name);

            menustate.0 = GameMenuState::CurrentlyCreating;

            *ghost = MachineGhost::start(entity.0, &mut commands, &cursor, tp);
        }
    }
}

fn redraw_menu(
    menu_state: Res<GameMenu>,
    mut q_menu_parts: Query<(Option<&mut Text>, &mut Visibility, &GameMenuPart)>,
    ghost: Res<MachineGhost>,
    q_types: Query<&MachineType>,
) {
    let state = menu_state.0;

    let mut current_text = None;

    for (text, mut vis, part) in q_menu_parts.iter_mut() {
        if part.0 == state {
            *vis = Visibility::Visible;
            current_text = text;
        } else {
            *vis = Visibility::Hidden;
        }
    }

    match state {
        GameMenuState::CurrentlyCreating => {
            let mut text = current_text.unwrap();

            let Some((tp, _)) = ghost.0 else {
                text.sections[1].value = "(bug: No machine selected.)".into();
                return;
            };

            let tp = q_types.get(tp).unwrap();

            text.sections[1].value = tp.name.to_string();
        }
        GameMenuState::SelectedMachine => {
            let mut text = current_text.unwrap();

            let Some((tp, _)) = ghost.0 else {
                text.sections[1].value = "(bug: No machine selected.)".into();
                return;
            };

            let tp = q_types.get(tp).unwrap();

            text.sections[1].value = tp.name.to_string();
        }
        _ => {}
    }
}
