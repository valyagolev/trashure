use bevy::prelude::*;

pub struct AnimatedPlugin;
impl Plugin for AnimatedPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, animate);
    }
}

#[derive(Debug, Component)]
pub struct MovingToPosition {
    pub target: Vec3,
    pub speed: f32,
}

fn animate(mut q_animation: Query<(&mut Transform, &MovingToPosition)>) {
    for (mut transform, moving_to_position) in q_animation.iter_mut() {
        let delta = moving_to_position.target - transform.translation;

        println!("transform: {:?}", transform.translation);
        println!("target: {:?}", moving_to_position.target);
        println!("delta: {:?}", delta);

        let distance = delta.length();

        let direction = delta.normalize();

        println!("distance: {}", distance);
        println!("direction: {:?}", direction);

        let movement = direction * moving_to_position.speed * 0.1;

        println!("movement: {:?}", movement);

        if distance < movement.length() || distance < moving_to_position.speed {
            transform.translation = moving_to_position.target;
        } else {
            transform.translation += movement;
        }

        if transform.translation.x.is_nan() {
            panic!("transform.translation.x is NaN");
        }
    }
}
