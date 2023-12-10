use std::f32::consts::PI;

use bevy::{prelude::*, time::Stopwatch};
use rand::Rng;

use crate::{
    game::Direction2D,
    graphics::{
        machines::{DebugCube, MachineResources, MyMachine},
        recolor::Tinted,
        voxels3d::{lazyworld::LazyWorld, VoxelBlock},
    },
};

use super::{Radar, RadarFoundVoxel};

pub struct RadarGraphicsPlugin;

impl Plugin for RadarGraphicsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (setup_radars_graphics, redraw_radars));
    }
}

#[derive(Component)]
pub struct RadarScene;

fn setup_radars_graphics(
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
    }
}

fn redraw_radars(q_radars: Query<&Radar>, mut q_scenes: Query<&mut Transform, With<RadarScene>>) {
    for r in q_radars.iter() {
        let Some(mut t) = r.scene.and_then(|e| q_scenes.get_mut(e).ok()) else {
            continue;
        };

        let dist = r.dist();

        t.scale = Vec3::splat(dist * 2.0);
    }
}
