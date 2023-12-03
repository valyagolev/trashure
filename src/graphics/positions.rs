use bevy::prelude::*;

use crate::conf::Configuration;

pub struct GridPositionedPlugin;

impl Plugin for GridPositionedPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<GridPositioned>()
            .add_systems(Update, reposition);
    }
}

#[derive(Debug, Component, Reflect)]
pub struct GridPositioned(pub IVec2);

fn reposition(mut q: Query<(&mut Transform, &GridPositioned)>, conf: Res<Configuration>) {
    for (mut transform, grid_positioned) in q.iter_mut() {
        transform.translation = Vec3::new(
            (grid_positioned.0.x as f32) * conf.grid_size,
            (grid_positioned.0.y as f32) * conf.grid_size,
            (grid_positioned.0.y as f32) * -100.0, // - 10000.0,
        );
    }
}
