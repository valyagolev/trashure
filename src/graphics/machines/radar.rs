use std::f32::consts::PI;

use bevy::{prelude::*, time::Stopwatch};

use crate::graphics::recolor::Tinted;

use super::{DebugCube, MachineResources};

pub struct RadarPlugin;

impl Plugin for RadarPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (setup_radars, update_radars.after(setup_radars)));
    }
}

#[derive(Component)]
pub struct Radar {
    watch: Stopwatch,
    scene: Option<Entity>,
}

#[derive(Component)]
pub struct RadarScene;

impl Radar {
    pub fn new() -> Self {
        Radar {
            watch: Stopwatch::new(),
            scene: None,
        }
    }

    fn dist(&self) -> f32 {
        //todo
        30.0 * ((self.watch.elapsed().as_secs_f32() / 5.0).sin()).abs()
    }
}

pub fn setup_radars(
    mut commands: Commands,
    mut q_radars: Query<(Entity, &mut Radar), Added<Radar>>,
    res: Res<MachineResources>,
) {
    for (e, mut r) in q_radars.iter_mut() {
        let radar_e = commands
            .spawn((
                RadarScene,
                SceneBundle {
                    transform: Transform::from_rotation(Quat::from_rotation_y(-PI / 4.0)),
                    scene: res.radar.clone(),
                    ..Default::default()
                },
                Tinted::new(Color::rgba(0.7, 0.3, 0.3, 0.2)).alpha(),
            ))
            .id();
        r.scene = Some(radar_e);
        commands.entity(e).add_child(radar_e);

        // let cube = commands
        //     .spawn((
        //         DebugCube,
        //         PbrBundle {
        //             mesh: res.selection_cube.clone(),
        //             material: res.debug_reddish.clone(),
        //             // transform: Transform::from_scale(Vec3::new(tp.dims.x as f32, 32.0, tp.dims.y as f32)),
        //             ..default()
        //         },
        //     ))
        //     .id();
        // commands.entity(e).add_child(cube);
    }
}

pub fn update_radars(
    time: Res<Time>,
    mut q_radars: Query<(&mut Radar, &Children)>,
    mut q_scenes: Query<&mut Transform, (With<RadarScene>, Without<DebugCube>)>,
    // mut q_cubes: Query<&mut Transform, (With<DebugCube>, Without<RadarScene>)>,
) {
    for (mut r, _children) in q_radars.iter_mut() {
        r.watch.tick(time.delta());

        let Ok(mut t) = q_scenes.get_mut(r.scene.unwrap()) else {
            continue;
        };

        let dist = r.dist();

        // 2.0 is the scale of the radar scene
        t.scale = Vec3::splat(dist / 2.0);

        // for ch in children {
        //     if let Ok(mut cube) = q_cubes.get_mut(*ch) {
        //         cube.scale = Vec3::splat(dist);
        //     }
        // }
    }
}
