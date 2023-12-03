use bevy::prelude::*;

pub struct GridPositionedPlugin;

impl Plugin for GridPositionedPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, reposition);
    }
}

#[derive(Debug, Component)]
pub struct GridPositioned(pub Vec2);

fn reposition(mut q: Query<(&mut Transform, &GridPositioned)>) {
    for (mut transform, grid_positioned) in q.iter_mut() {
        transform.translation =
            Vec3::new(grid_positioned.0.x * 72.0, grid_positioned.0.y * 72.0, 0.0);
    }
}
