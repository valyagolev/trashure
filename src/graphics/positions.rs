// use bevy::prelude::*;

// use crate::conf::Configuration;

// pub struct IntegerPositionedPlugin;

// impl Plugin for IntegerPositionedPlugin {
//     fn build(&self, app: &mut App) {
//         app.register_type::<IntegerPositioned>()
//             .add_systems(Update, reposition);
//     }
// }

// #[derive(Debug, Component, Reflect)]
// pub struct IntegerPositioned(pub IVec3);

// fn reposition(mut q: Query<(&mut Transform, &IntegerPositioned)>, conf: Res<Configuration>) {
//     for (mut transform, grid_positioned) in q.iter_mut() {
//         transform.translation = grid_positioned.0.as_vec3() * conf.tile_size;
//     }
// }
