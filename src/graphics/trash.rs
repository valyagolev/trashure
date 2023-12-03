use bevy::prelude::*;

pub struct TrashExperimentPlugin;

impl Plugin for TrashExperimentPlugin {
    fn build(&self, app: &mut App) {
        // app.insert_resource(Configuration::default())
        //     // .insert_resource(Persistent::<Configuration>::n())
        //     .add_systems(Startup, setup)
        //     .add_systems(Update, on_modify_configuration)
        //     .register_type::<Configuration>() // you need to register your type to display it
        //     .add_plugins(ResourceInspectorPlugin::<Configuration>::default());
    }
}
