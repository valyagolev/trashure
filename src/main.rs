mod conf;
#[cfg(feature = "dbg")]
mod debugeditor;
mod game;
mod graphics;
#[allow(unused_imports)]
use bevy::{asset::AssetMetaCheck, prelude::*};
use bevy::{log::LogPlugin, window::PresentMode};

// use game::train::TrainPlugin;
// use graphics::{
//     cam::setup_camera,
//     ui::{self, mouse_button_input},
// };

fn main() {
    let mut app = App::new();
    app.insert_resource(AssetMetaCheck::Never)
        // outside
        .add_plugins(
            #[cfg(not(feature = "graph"))]
            DefaultPlugins,
            #[cfg(feature = "graph")]
            DefaultPlugins.build().disable::<LogPlugin>(),
        )
        // .insert_resource(Time::<Fixed>::from_seconds(0.1))
        // .add_plugins(bevy::pbr::wireframe::WireframePlugin)
        .add_plugins(bevy_debug_text_overlay::OverlayPlugin {
            font_size: 23.0,
            ..default()
        })
        .add_plugins(bevy::diagnostic::FrameTimeDiagnosticsPlugin)
        // .add_plugins(bevy_inspector_egui::quick::WorldInspectorPlugin::new())
        // mine
        .add_plugins((
            conf::ConfigPlugin,
            #[cfg(feature = "dbg")]
            debugeditor::DebugEditorPlugin,
        ))
        // .add_plugins(bevy::render::camera::CameraPlugin)
        .add_plugins(graphics::camera3d::Camera3dPlugin)
        // .add_plugins(graphics::atlases::AtlasesPlugin)
        .add_plugins(graphics::voxels3d::Voxels3dPlugin)
        // .add_plugins(TrashExperimentPlugin)
        // .add_plugins(graphics::animated::AnimatedPlugin)
        // .add_plugins(graphics::pieces::PiecesPlugin)
        .add_plugins(graphics::voxels3d::lazyworld::LazyWorldPlugin)
        .add_plugins((
            graphics::fps::FpsPlugin,
            // graphics::dbgtext::DbgTextPlugin
        ))
        .add_plugins((
            game::GameUtilsPlugin,
            graphics::gamemenu::GameMenuPlugin,
            graphics::machines::MachinesPlugin,
            graphics::cursor::CursorPlugin,
            graphics::recolor::RecolorPlugin,
            graphics::selectable::SelectablePlugin,
            graphics::flyingvoxel::FlyingVoxelPlugin,
            game::voxelmailbox::VoxelMailboxPlugin,
            graphics::debug3d::Debug3dPlugin,
            graphics::scenerenderlayer::SceneRenderLayersPlugin,
            graphics::sceneobjectfinder::SceneObjectFinderPlugin,
        ));
    // .add_plugins(graphics::voxels::VoxelsPlugin)
    // .add_plugins(graphics::positions::IntegerPositionedPlugin)
    // .add_plugins(TrainPlugin)
    // .add_systems(Startup, (setup_camera, ui::setup))
    // .add_systems(Update, (mouse_button_input, ui::check_config_changed))

    #[cfg(feature = "graph")]
    {
        bevy_mod_debugdump::print_schedule_graph(&mut app, Update);
        return;
    }

    app.run();
}
