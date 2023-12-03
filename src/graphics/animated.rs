use bevy::prelude::*;

pub struct AnimatedPlugin;
impl Plugin for AnimatedPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Time::<Fixed>::from_seconds(0.05))
            .add_systems(FixedUpdate, animate);
    }
}

#[derive(Debug, Component)]
pub struct MovingToPosition {
    pub target: Vec3,
    pub speed: f32,
}

fn animate(mut q_animation: Query<(&mut Transform, &MovingToPosition)>, time: Res<Time>) {
    let dt = time.delta_seconds();

    for (mut transform, moving_to_position) in q_animation.iter_mut() {
        let delta = moving_to_position.target - transform.translation;

        let dist_to_cover = moving_to_position.speed * dt;

        let remaining_distance = delta.length();

        let movement = delta.normalize() * dist_to_cover;

        if remaining_distance < moving_to_position.speed {
            transform.translation = moving_to_position.target;
        } else {
            transform.translation += movement;
        }

        if transform.translation.x.is_nan() {
            panic!("transform.translation.x is NaN");
        }
    }
}
