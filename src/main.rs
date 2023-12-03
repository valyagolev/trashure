mod conf;
// mod game;
mod graphics;

use bevy::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use conf::ConfigPlugin;
use graphics::{
    animated::AnimatedPlugin, atlases::AtlasesPlugin, camera::CameraPlugin,
    trash::TrashExperimentPlugin,
};
// use game::train::TrainPlugin;
// use graphics::{
//     cam::setup_camera,
//     ui::{self, mouse_button_input},
// };

fn main() {
    App::new()
        // .insert_resource(GreetTimer(Timer::from_seconds(2.0, TimerMode::Repeating)))
        .add_plugins(DefaultPlugins)
        .add_plugins(WorldInspectorPlugin::new())
        .add_plugins(ConfigPlugin)
        .add_plugins(CameraPlugin)
        .add_plugins(AtlasesPlugin)
        .add_plugins(TrashExperimentPlugin)
        .add_plugins(AnimatedPlugin)
        // .add_plugins(TrainPlugin)
        // .add_systems(Startup, (setup_camera, ui::setup))
        // .add_systems(Update, (mouse_button_input, ui::check_config_changed))
        .run();
}
