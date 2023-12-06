use bevy::diagnostic::DiagnosticId;
use bevy::prelude::*;

use bevy::diagnostic::DiagnosticsStore;
use bevy::diagnostic::FrameTimeDiagnosticsPlugin;

use super::voxels3d::lazyworld::UNAPPLIED_CHANGES;
use super::voxels3d::lazyworld::WORLD_PARTS_DIAGNOSTIC;
use super::voxels3d::APPLIED_CHANGES;
use super::voxels3d::CHANGED_BLOCKS;
use super::voxels3d::POSTPONED_CHANGES;

pub struct FpsPlugin;

impl Plugin for FpsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_fps_counter);
        app.add_systems(
            Update,
            fps_text_update_system, // (

                                    //     // fps_counter_showhide
                                    // ),
        );
    }
}

/// Marker to find the container entity so we can show/hide the FPS counter
#[derive(Component)]
struct FpsRoot;

#[derive(Component)]
struct DiagnosticText(DiagnosticId);

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
        ("FPS", FrameTimeDiagnosticsPlugin::FPS),
        ("World Parts", WORLD_PARTS_DIAGNOSTIC),
        ("Unapplied Changes", UNAPPLIED_CHANGES),
        ("Applied Changes", APPLIED_CHANGES),
        ("Postponed Changes", POSTPONED_CHANGES),
        ("Changed Blocks", CHANGED_BLOCKS),
    ]
    .map(|(name, did)| {
        commands
            .spawn((
                DiagnosticText(did),
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
}

fn fps_text_update_system(
    diagnostics: Res<DiagnosticsStore>,
    mut query: Query<(&mut Text, &DiagnosticText)>,
) {
    for (mut text, dt) in query.iter_mut() {
        if let Some(value) = diagnostics.get(dt.0).and_then(|fps| //fps.smoothed()
             fps.value())
        {
            text.sections[1].value = format!("{value:>6.0}");
        } else {
            text.sections[1].value = " N/A".into();
        }
    }
}
