use bevy::prelude::*;

use super::machines::{building::MachineGhost, MachineType};

pub struct GameMenuPlugin;
mod looks;
impl Plugin for GameMenuPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(GameMenu(GameMenuState::ToPickBuilding))
            .add_systems(Update, looks::setup_menu)
            .add_systems(Update, redraw_menu);
    }
}

pub enum GameMenuState {
    ToPickBuilding,
    CurrentlyCreating,
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
struct GameMenuCurrentlyCreating;

#[derive(Component)]
struct GameMenuToPickBuilding;

#[derive(Component)]
struct GameMenuToPickBuildingForMachineButton(Entity);

fn redraw_menu(
    menu_state: Res<GameMenu>,
    mut q_menu_currently: Query<
        (&mut Text, &mut Visibility),
        (
            With<GameMenuCurrentlyCreating>,
            Without<GameMenuToPickBuilding>,
        ),
    >,
    mut q_menu_to_pick: Query<
        &mut Visibility,
        (
            With<GameMenuToPickBuilding>,
            Without<GameMenuCurrentlyCreating>,
        ),
    >,
    ghost: Res<MachineGhost>,
    q_types: Query<&MachineType>,
) {
    let Ok(mut q_menu_currently) = q_menu_currently.get_single_mut() else {
        return;
    };
    let mut q_menu_to_pick = q_menu_to_pick.single_mut();

    match menu_state.0 {
        GameMenuState::CurrentlyCreating => {
            let (mut text, mut vis) = q_menu_currently;

            *vis = Visibility::Visible;
            *q_menu_to_pick = Visibility::Hidden;

            let Some((tp, _)) = ghost.0 else {
                text.sections[1].value = "(bug: No machine selected.)".into();
                return;
            };

            let tp = q_types.get(tp).unwrap();

            text.sections[1].value = tp.name.to_string().into();
        }
        GameMenuState::ToPickBuilding => {
            *q_menu_currently.1 = Visibility::Hidden;
            *q_menu_to_pick = Visibility::Visible;
        }
    }
}
