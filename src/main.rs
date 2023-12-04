mod conf;
mod game;
mod graphics;

use bevy::{asset::AssetMetaCheck, prelude::*};

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
        .add_plugins(bevy::pbr::wireframe::WireframePlugin)
        .add_plugins(bevy_debug_text_overlay::OverlayPlugin {
            font_size: 23.0,
            ..default()
        })
        .add_plugins(bevy::diagnostic::FrameTimeDiagnosticsPlugin)
        // .add_plugins(bevy_inspector_egui::quick::WorldInspectorPlugin::new())
        // mine
        .add_plugins(conf::ConfigPlugin)
        // .add_plugins(bevy::render::camera::CameraPlugin)
        .add_plugins(graphics::camera3d::Camera3dPlugin)
        // .add_plugins(graphics::atlases::AtlasesPlugin)
        .add_plugins(graphics::voxels3d::Voxels3dPlugin)
        // .add_plugins(TrashExperimentPlugin)
        .add_plugins(graphics::animated::AnimatedPlugin)
        // .add_plugins(graphics::pieces::PiecesPlugin)
        .add_plugins(graphics::lazyworld::LazyWorldPlugin)
        .add_plugins(graphics::fps::FpsPlugin)
        // .add_plugins(graphics::voxels::VoxelsPlugin)
        // .add_plugins(graphics::positions::IntegerPositionedPlugin)
        // .add_plugins(TrainPlugin)
        // .add_systems(Startup, (setup_camera, ui::setup))
        // .add_systems(Update, (mouse_button_input, ui::check_config_changed))
        .run();
}
