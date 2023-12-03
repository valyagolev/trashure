mod conf;
// mod game;
mod graphics;

use bevy::{asset::AssetMetaCheck, diagnostic::FrameTimeDiagnosticsPlugin, prelude::*};
use bevy_debug_text_overlay::OverlayPlugin;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use conf::ConfigPlugin;
use graphics::{
    animated::AnimatedPlugin, atlases::AtlasesPlugin, camera::CameraPlugin,
    positions::GridPositionedPlugin, trash::TrashExperimentPlugin,
};
// use game::train::TrainPlugin;
// use graphics::{
//     cam::setup_camera,
//     ui::{self, mouse_button_input},
// };

fn main() {
    App::new()
        .insert_resource(AssetMetaCheck::Never)
        // outside
        .add_plugins(DefaultPlugins)
        .add_plugins(OverlayPlugin {
            font_size: 23.0,
            ..default()
        })
        .add_plugins(FrameTimeDiagnosticsPlugin)
        .add_plugins(WorldInspectorPlugin::new())
        // mine
        .add_plugins(ConfigPlugin)
        .add_plugins(CameraPlugin)
        .add_plugins(AtlasesPlugin)
        .add_plugins(TrashExperimentPlugin)
        .add_plugins(AnimatedPlugin)
        .add_plugins(GridPositionedPlugin)
        // .add_plugins(TrainPlugin)
        // .add_systems(Startup, (setup_camera, ui::setup))
        // .add_systems(Update, (mouse_button_input, ui::check_config_changed))
        .run();
}
