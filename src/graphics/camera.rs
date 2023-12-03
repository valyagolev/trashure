use bevy::prelude::*;

use crate::conf::Configuration;

pub struct CameraPlugin;
impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup)
            .add_systems(Update, camera_setup);
    }
}

fn camera_setup(
    conf: Res<Configuration>,
    mut camera: Query<(&mut OrthographicProjection, With<Camera2d>)>,
) {
    if conf.is_changed() {
        println!("camera_setup changed");
        for (mut proj, _) in camera.iter_mut() {
            print!("chanign camera scale to {}", conf.camera_scale);
            proj.scale = conf.camera_scale;
        }
    }
}

fn setup(mut commands: Commands, conf: Res<Configuration>) {
    commands.spawn(Camera2dBundle {
        transform: Transform::from_xyz(0.0, 0.0, 1.0),
        projection: OrthographicProjection {
            scale: conf.camera_scale,
            ..default()
        },
        ..default()
    });
}
