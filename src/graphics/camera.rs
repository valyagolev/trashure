use bevy::prelude::*;

use crate::conf::Configuration;

pub struct CameraPlugin;
impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup)
            .add_systems(Update, camera_setup)
            .add_systems(Update, handle_camera_move);
    }
}

fn camera_setup(
    conf: Res<Configuration>,
    mut camera: Query<(&mut OrthographicProjection, With<Camera2d>)>,
) {
    if conf.is_changed() {
        for (mut proj, _) in camera.iter_mut() {
            proj.scale = conf.camera_scale;
        }
    }
}

fn setup(mut commands: Commands, conf: Res<Configuration>) {
    commands.spawn(Camera2dBundle {
        transform: Transform::from_xyz(0.0, 0.0, 1000.0),
        projection: OrthographicProjection {
            scale: conf.camera_scale,
            far: 1000000.0,
            ..default()
        },
        ..default()
    });
}

static KEY_TO_DIRECTION: [(KeyCode, Vec2); 4] = [
    (KeyCode::Up, Vec2::Y),
    (KeyCode::Down, Vec2::new(0.0, -1.0)),
    (KeyCode::Left, Vec2::new(-1.0, 0.0)),
    (KeyCode::Right, Vec2::X),
];

fn handle_camera_move(
    keys: Res<Input<KeyCode>>,
    conf: Res<Configuration>,
    mut camera: Query<(&mut Transform, With<Camera2d>)>,
) {
    for (key, dir) in KEY_TO_DIRECTION.iter() {
        if keys.pressed(*key) {
            for (mut transform, _) in camera.iter_mut() {
                transform.translation += dir.extend(0.0) * conf.camera_speed;
            }
        }
    }
}
