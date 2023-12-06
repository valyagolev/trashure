use std::{borrow::Cow, sync::Mutex, time::Duration};

use bevy::{
    core_pipeline::clear_color::ClearColorConfig,
    prelude::*,
    render::{camera::ScalingMode, view::RenderLayers},
    transform::commands,
    utils::{HashMap, Instant},
};
use once_cell::sync::Lazy;

use super::camera3d::CAMERA_OFFSET;

pub struct Debug3dPlugin;

impl Plugin for Debug3dPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_gizmo_camera)
            .add_systems(Update, draw_stuff)
            .insert_resource(GizmoConfig {
                // render_layers: RenderLayers::layer(13),
                ..default()
            });
    }
}

type GizmosCb = Box<dyn Fn(&mut Gizmos) + Send + Sync>;

static STORE: Lazy<Mutex<Vec<(Option<Cow<'static, str>>, GizmosCb, Instant)>>> =
    Lazy::new(|| Mutex::new(Vec::new()));

pub fn draw_gizmos_labeled(
    name: impl Into<Cow<'static, str>>,
    secs: f32,
    cb: impl Fn(&mut Gizmos) + Send + Sync + 'static,
) {
    let name = name.into();
    let mut st = STORE.lock().unwrap();
    st.retain(|(n, _, _)| n.as_ref() != Some(&name));
    st.push((
        Some(name.into()),
        Box::new(cb),
        Instant::now() + Duration::from_secs_f32(secs),
    ));
}

pub fn draw_gizmos(secs: f32, cb: impl Fn(&mut Gizmos) + Send + Sync + 'static) {
    STORE.lock().unwrap().push((
        None,
        Box::new(cb),
        Instant::now() + Duration::from_secs_f32(secs),
    ));
}

fn setup_gizmo_camera(mut commands: Commands) {
    commands.spawn((
        RenderLayers::layer(13),
        Camera3dBundle {
            camera: Camera {
                order: 100,
                ..default()
            },
            camera_3d: Camera3d {
                clear_color: ClearColorConfig::None,
                ..default()
            },
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
        },
    ));
}

fn draw_stuff(mut gizmos: Gizmos) {
    let mut st = STORE.lock().unwrap();
    let now = Instant::now();

    st.retain(|(_, _, t)| *t > now);

    for (_, cb, _) in st.iter() {
        cb(&mut gizmos);
    }
}

// #[derive(Resource)]
// pub struct DebugStuff {
//     sphere: Handle<Mesh>,
//     red_material: Handle<StandardMaterial>,
// }

// fn setup(
//     mut commands: Commands,
//     mut meshes: ResMut<Assets<Mesh>>,
//     mut materials: ResMut<Assets<StandardMaterial>>,
// ) {
//     let sphere = meshes.add(Mesh::from(Sphere {
//         radius: 0.5,
//         subdivisions: 4,
//     }));
//     let red_material = materials.add(Color::rgb(8.0, 2.0, 2.0).into());

//     commands.insert_resource(DebugStuff {
//         sphere,
//         red_material,
//     });
// }
