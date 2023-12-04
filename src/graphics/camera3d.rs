use bevy::{prelude::*, render::camera::ScalingMode};
use bevy_inspector_egui::bevy_egui::EguiContext;

use crate::conf::Configuration;

pub struct Camera3dPlugin;
impl Plugin for Camera3dPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup)
            .add_systems(Update, camera_setup)
            .add_systems(Update, handle_camera_move);
    }
}

fn camera_setup(
    conf: Res<Configuration>,
    mut camera: Query<(&mut OrthographicProjection, With<Camera>)>,
) {
    if conf.is_changed() {
        for (mut proj, _) in camera.iter_mut() {
            proj.scale = conf.camera_scale;
        }
    }
}

fn setup(mut commands: Commands, conf: Res<Configuration>) {
    // ambient light
    commands.insert_resource(AmbientLight {
        color: Color::WHITE,
        brightness: 0.06,
    });

    commands
        .spawn(Camera3dBundle {
            projection: OrthographicProjection {
                scale: 3.0,
                scaling_mode: ScalingMode::FixedVertical(2.0),
                ..default()
            }
            .into(),
            transform: Transform::from_xyz(5.0, 5.0, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
            ..default()
        })
        .with_children(|b| {
            b.spawn(PointLightBundle {
                transform: Transform::from_xyz(4.0, 4.0, -6.0),
                ..default()
            });
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
    mut camera: Query<(&mut Transform, With<Camera3d>)>,
) {
    for (key, dir) in KEY_TO_DIRECTION.iter() {
        if keys.pressed(*key) {
            for (mut transform, _) in camera.iter_mut() {
                transform.translation += dir.extend(0.0) * conf.camera_speed;
            }
        }
    }
}
