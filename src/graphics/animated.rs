use bevy::prelude::*;

use crate::conf::Configuration;

pub struct AnimatedPlugin;
impl Plugin for AnimatedPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Time::<Fixed>::from_seconds(0.05))
            .add_systems(FixedUpdate, animate);
    }
}

#[derive(Debug, Component)]
pub struct MovingToPosition {
    pub target: IVec3,
    pub speed: f32,
}

impl MovingToPosition {
    pub fn new(target: IVec3, speed: f32) -> Self {
        Self { target, speed }
    }

    pub fn is_close_to(&self, current_pos: Vec3) -> bool {
        let delta = self.target.as_vec3() - current_pos;
        delta.length() < 0.1
    }
}

pub fn transform_to_voxel_grid(conf: &Res<Configuration>, a: IVec3) -> Vec3 {
    let vs = a.as_vec3() * conf.grid_size;
    let x = vs.x + vs.z * 0.5;
    let y = vs.y + vs.z * 0.5;

    Vec3::new(x - y, x + y, -vs.z * 10.0 + x + y)
}

fn animate(
    mut q_animation: Query<(&mut Transform, &MovingToPosition)>,
    time: Res<Time>,
    conf: Res<Configuration>,
) {
    let dt = time.delta_seconds();

    for (mut transform, moving_to_position) in q_animation.iter_mut() {
        let target = transform_to_voxel_grid(&conf, moving_to_position.target);

        let delta = target - transform.translation;

        let dist_to_cover = moving_to_position.speed * dt;

        let remaining_distance = delta.length();

        let movement = delta.normalize() * dist_to_cover;

        if remaining_distance < moving_to_position.speed {
            transform.translation = target;
        } else {
            transform.translation += movement;
        }

        if transform.translation.x.is_nan() {
            panic!("transform.translation.x is NaN");
        }
    }
}
