use bevy::prelude::*;

use super::machines::{building::MachineGhost, MachineType};

pub struct GameMenuPlugin;

impl Plugin for GameMenuPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(GameMenu(GameMenuState::CurrentlyCreating))
            .add_systems(Startup, setup_menu)
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
struct GameMenuNode;

#[derive(Component)]
struct GameMenuNodeText;

fn redraw_menu(
    menu_state: Res<GameMenu>,
    mut q_menu: Query<&mut Text, With<GameMenuNodeText>>,
    ghost: Res<MachineGhost>,
    q_types: Query<&MachineType>,
) {
    if !menu_state.is_changed() && !ghost.is_changed() {
        return;
    }

    let mut text = q_menu.single_mut();

    match menu_state.0 {
        GameMenuState::CurrentlyCreating => {
            let Some((tp, _)) = ghost.0 else {
                text.sections[0].value = "No machine selected.".into();
                return;
            };

            let tp = q_types.get(tp).unwrap();

            text.sections[0].value = format!("Currently building: {}. Press R to rotate.", tp.name);
        }
        GameMenuState::ToPickBuilding => {
            text.sections[0].value = "Press B to build a machine.".into();
        }
    }
}

fn setup_menu(mut commands: Commands) {
    // create our UI root node
    // this is the wrapper/container for the text
    let root = commands
        .spawn((
            GameMenuNode,
            NodeBundle {
                // give it a dark background for readability
                background_color: BackgroundColor(Color::BLACK.with_a(0.8)),
                // make it "always on top" by setting the Z index to maximum
                // we want it to be displayed over all other UI
                z_index: ZIndex::Global(i32::MAX),
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
                    align_items: AlignItems::FlexEnd,
                    ..Default::default()
                },
                ..Default::default()
            },
        ))
        .id();

    let text = commands
        .spawn((
            GameMenuNodeText,
            TextBundle {
                // use two sections, so it is easy to update just the number
                text: Text::from_sections([TextSection {
                    value: "".into(),
                    style: TextStyle {
                        font_size: 20.0,
                        color: Color::WHITE,
                        // if you want to use your game's font asset,
                        // uncomment this and provide the handle:
                        // font: my_font_handle
                        ..default()
                    },
                }]),
                ..Default::default()
            },
        ))
        .id();

    commands.entity(root).push_children(&[text]);
}
