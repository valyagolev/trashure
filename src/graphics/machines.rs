use bevy::prelude::*;

pub struct MachinesPlugin;

impl Plugin for MachinesPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, load_machines)
            // .add_systems(Update, debug_keyboard)
            ;
    }
}

// #[derive(Debug, Component)]
// pub struct MyMachine;
fn load_machines(mut commands: Commands, ass: Res<AssetServer>) {
    // note that we have to include the `Scene0` label
    let my_gltf = ass.load("objects/recycler.glb#Scene0");

    // to position our 3d model, simply use the Transform
    // in the SceneBundle
    commands.spawn(SceneBundle {
        scene: my_gltf,
        transform: Transform::from_xyz(7.0, 0.5, 5.0)
            // .with_scale(Vec3::new(2.0, 2.0, 2.0))
            .with_rotation(Quat::from_rotation_y(4.4)),
        ..Default::default()
    });
}
