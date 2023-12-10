use bevy::prelude::*;

use bevy::diagnostic::DiagnosticsStore;
use bevy::utils::HashMap;

pub struct StatsPlugin;

impl Plugin for StatsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_fps_counter);
        app.add_systems(
            Update,
            fps_text_update_system, // (

                                    //     // fps_counter_showhide
                                    // ),
        )
        .insert_resource(StatsValues::new());
    }
}

/// Marker to find the container entity so we can show/hide the FPS counter
#[derive(Component)]
struct FpsRoot;

#[derive(Component)]
struct DiagnosticText(&'static str);

#[derive(Resource)]
pub struct StatsValues(HashMap<&'static str, usize>);

impl StatsValues {
    pub fn new() -> Self {
        Self(HashMap::default())
    }

    pub fn increment(&mut self, name: &'static str) {
        self.inc_n(name, 1);
    }

    pub fn inc_n(&mut self, name: &'static str, n: usize) {
        let value = self.0.entry(name).or_insert(0);
        *value += n;
    }
}

fn setup_fps_counter(mut commands: Commands) {
    // create our UI root node
    // this is the wrapper/container for the text
    let root = commands
        .spawn((
            // DiagnosticText(FrameTimeDiagnosticsPlugin::FPS),
            NodeBundle {
                // give it a dark background for readability
                background_color: BackgroundColor(Color::BLACK.with_a(0.5)),
                // make it "always on top" by setting the Z index to maximum
                // we want it to be displayed over all other UI
                z_index: ZIndex::Global(i32::MAX),
                style: Style {
                    position_type: PositionType::Absolute,
                    // position it at the top-right corner
                    // 1% away from the top window edge
                    right: Val::Percent(1.),
                    top: Val::Percent(1.),
                    // set bottom/left to Auto, so it can be
                    // automatically sized depending on the text
                    bottom: Val::Auto,
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

    let children = [
        "Recycled",
        "Maintained",
        "Fuel Consumed",
        // ("Unapplied Changes", UNAPPLIED_CHANGES),
        // ("Applied Changes", APPLIED_CHANGES),
        // ("Postponed Changes", POSTPONED_CHANGES),
        // ("Changed Blocks", CHANGED_BLOCKS),
    ]
    .map(|name| {
        commands
            .spawn((
                DiagnosticText(name),
                TextBundle {
                    // use two sections, so it is easy to update just the number
                    text: Text::from_sections([
                        TextSection {
                            value: name.into(),
                            style: TextStyle {
                                font_size: 16.0,
                                color: Color::WHITE,
                                // if you want to use your game's font asset,
                                // uncomment this and provide the handle:
                                // font: my_font_handle
                                ..default()
                            },
                        },
                        TextSection {
                            value: " N/A".into(),
                            style: TextStyle {
                                font_size: 16.0,
                                color: Color::WHITE,
                                // if you want to use your game's font asset,
                                // uncomment this and provide the handle:
                                // font: my_font_handle
                                ..default()
                            },
                        },
                    ]),
                    ..Default::default()
                },
            ))
            .id()
    });

    commands.entity(root).push_children(&children);

    let won = commands
        .spawn((
            DiagnosticText("win"),
            TextBundle {
                // use two sections, so it is easy to update just the number
                text: Text::from_sections([TextSection {
                    value: "Recycle 1000 to win".into(),
                    style: TextStyle {
                        font_size: 16.0,
                        color: Color::rgb(0.9, 0.9, 0.9),
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

    commands.entity(root).push_children(&[won]);
}

fn fps_text_update_system(
    stats_values: Res<StatsValues>,
    mut query: Query<(&mut Text, &DiagnosticText)>,
) {
    for (mut text, dt) in query.iter_mut() {
        if dt.0 == "win" {
            let value = stats_values.0.get("Recycled").unwrap_or(&0);
            if *value > 1000 {
                text.sections[0].value = "You won!".into();

                text.sections[0].style.color = Color::YELLOW;
            }
        } else {
            let value = stats_values.0.get(dt.0).unwrap_or(&0);

            text.sections[1].value = format!("{value:>6}");
        }
    }
}
