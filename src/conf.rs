use std::path::Path;

use bevy::prelude::*;
use bevy_inspector_egui::{prelude::*, quick::ResourceInspectorPlugin};
use bevy_persistent::prelude::*;
use serde::{Deserialize, Serialize};

pub struct ConfigPlugin;

impl Plugin for ConfigPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Configuration::default())
            // .insert_resource(Persistent::<Configuration>::n())
            .add_systems(Startup, setup)
            // .add_systems(Update, on_modify_configuration)
            .register_type::<Configuration>() // you need to register your type to display it
            .add_plugins(ResourceInspectorPlugin::<Configuration>::default());
    }
}

#[derive(Reflect, Resource, Default, InspectorOptions, Serialize, Deserialize, Clone)]
#[reflect(Resource, InspectorOptions)]
pub struct Configuration {
    // #[inspector(min = 0.0, max = 2.0)]
    // pub track_point_scale: f32,
    #[inspector(min = 0.0, max = 2000.0)]
    pub camera_scale: f32,

    #[inspector(min = 0.0, max = 200000.0)]
    pub grid_size: f32,
}

// fn on_modify_configuration(
//     config: Res<Configuration>,
//     mut persistent_config: ResMut<Persistent<Configuration>>,
// ) {
//     if !config.is_changed() {
//         return;
//     }

//     persistent_config
//         .update(|pers| {
//             *pers = config.clone();
//         })
//         .expect("failed to update config");
// }

fn setup(mut commands: Commands) {
    // let config_dir = dirs::config_dir().unwrap().join("your-amazing-game");
    //todo
    // let config_dir = Path::new("/Users/valentin/Work/cw/trashure");

    // let pers = Persistent::<Configuration>::builder()
    //     .name("configuration")
    //     .format(StorageFormat::Json)
    //     .path(config_dir.join("configuration.json"))
    //     .default(Configuration {
    //         camera_scale: 2.5,
    //         // track_point_scale: 1.0,
    //         // track_point_min_distance: 2.0,
    //     })
    //     .build()
    //     .expect("failed to initialize ");

    commands.insert_resource(Configuration {
        camera_scale: 2.5,
        grid_size: 300.0,
    });
    // commands.insert_resource(pers);
}
