use bevy::{
    core_pipeline::clear_color::ClearColorConfig, pbr::DirectionalLightShadowMap, prelude::*,
    render::camera::ScalingMode,
};
// use bevy_inspector_egui::bevy_egui::EguiContext;

use crate::conf::Configuration;

pub static CAMERA_OFFSET: Vec3 = Vec3::new(50.0, 50.0, 50.0);

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
        brightness: 0.1,
    });

    // commands.insert_resource(DirectionalLightShadowMap { size: 1024 });

    commands.spawn(DirectionalLightBundle {
        directional_light: DirectionalLight {
            color: Color::WHITE,
            illuminance: 15000.0,
            shadows_enabled: true,
            // shadows_enabled: false,
            shadow_depth_bias: 0.02,
            shadow_normal_bias: 1.8,
        },
        transform: Transform::from_xyz(7.0, 5.0, 2.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });

    commands
        .spawn(Camera3dBundle {
            projection: OrthographicProjection {
                scale: 20.0,
                scaling_mode: ScalingMode::FixedVertical(2.0),
                // near: -1000.0,
                near: 0.0,
                ..default()
            }
            .into(),
            transform: Transform::from_translation(CAMERA_OFFSET).looking_at(Vec3::ZERO, Vec3::Y),
            ..default()
        })
        // .with_children(|b| {
        //     b.spawn(PointLightBundle {
        //         point_light: PointLight {
        //             intensity: 6000.0,
        //             range: 800.0,
        //             ..default()
        //         },
        //         transform: Transform::from_xyz(10.0, 14.0, -1.0),
        //         ..default()
        //     });
        // })
        ;

    // commands.spawn(Camera2dBundle {
    //     // transform: Transform::from_xyz(0.0, 0.0, 1000.0),
    //     projection: OrthographicProjection {
    //         // scale: conf.camera_scale,
    //         // far: 1000000.0,
    //         ..default()
    //     },
    //     camera: Camera {
    //         order: 1,
    //         ..default()
    //     },
    //     camera_2d: Camera2d {
    //         clear_color: ClearColorConfig::None,
    //         ..default()
    //     },

    //     ..default()
    // });
}

static KEY_TO_DIRECTION: &[(KeyCode, Vec3)] = &[
    // (KeyCode::Up, Vec2::Z),
    // (KeyCode::Down, Vec2::new(0.0, -1.0)),
    // (KeyCode::Left, Vec2::new(-1.0, 0.0)),
    // (KeyCode::Right, Vec2::X),
    (KeyCode::Up, Vec3::new(-1.0, 0.0, -1.0)),
    (KeyCode::Down, Vec3::new(1.0, 0.0, 1.0)),
    (KeyCode::Left, Vec3::new(-1.0, 0.0, 1.0)),
    (KeyCode::Right, Vec3::new(1.0, 0.0, -1.0)),
];

fn handle_camera_move(
    keys: Res<Input<KeyCode>>,
    conf: Res<Configuration>,
    mut camera: Query<(&mut Transform, With<Camera3d>)>,
) {
    for (key, dir) in KEY_TO_DIRECTION.iter() {
        if keys.pressed(*key) {
            for (mut transform, _) in camera.iter_mut() {
                transform.translation += *dir * conf.camera_speed;
            }
        }
    }
}
