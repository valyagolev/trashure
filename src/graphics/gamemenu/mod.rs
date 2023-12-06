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

            text.sections[1].value = tp.name.to_string();
        }
        GameMenuState::ToPickBuilding => {
            *q_menu_currently.1 = Visibility::Hidden;
            *q_menu_to_pick = Visibility::Visible;
        }
    }
}
