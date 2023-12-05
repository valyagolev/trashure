use bevy::diagnostic::DiagnosticId;
use bevy::prelude::*;

use bevy::diagnostic::DiagnosticsStore;
use bevy::diagnostic::FrameTimeDiagnosticsPlugin;
use bevy::utils::HashMap;
use itertools::Itertools;

use super::lazyworld::UNAPPLIED_CHANGES;
use super::lazyworld::WORLD_PARTS_DIAGNOSTIC;

pub struct DbgTextPlugin;

impl Plugin for DbgTextPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_text)
            .add_systems(Update, text_update_system)
            .insert_resource(DebugTexts(default()));
    }
}

#[derive(Resource)]
pub struct DebugTexts(HashMap<&'static str, String>);

impl DebugTexts {
    pub fn set(&mut self, id: &'static str, value: String) {
        self.0.insert(id, value);
    }
    pub fn delete(&mut self, id: &'static str) {
        self.0.remove(id);
    }
}

#[derive(Component)]
struct DbgText;

fn setup_text(mut commands: Commands) {
    // create our UI root node
    // this is the wrapper/container for the text
    let root = commands
        .spawn((
            DbgText,
            NodeBundle {
                // give it a dark background for readability
                background_color: BackgroundColor(Color::BLACK.with_a(0.5)),
                // make it "always on top" by setting the Z index to maximum
                // we want it to be displayed over all other UI
                z_index: ZIndex::Global(i32::MAX),
                style: Style {
                    position_type: PositionType::Absolute,

                    right: Val::Percent(1.),
                    top: Val::Auto,
                    bottom: Val::Percent(1.),
                    left: Val::Auto,
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
            DbgText,
            TextBundle {
                // use two sections, so it is easy to update just the number
                text: Text::from_sections([TextSection {
                    value: "".into(),
                    style: TextStyle {
                        font_size: 16.0,
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

fn text_update_system(texts: Res<DebugTexts>, mut query: Query<(&mut Text, &DbgText)>) {
    for (mut text, dt) in query.iter_mut() {
        let total_text =
            texts
                .0
                .iter()
                .sorted_by_key(|kv| kv.0)
                .fold(String::new(), |mut acc, (k, v)| {
                    acc.push_str(&format!("{}: {}\n", k, v));
                    acc
                });

        text.sections[0].value = total_text.into();
    }
}
