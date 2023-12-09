use std::collections::VecDeque;

use bevy::prelude::*;

use super::material::GameMaterial;

pub struct VoxelMailboxPlugin;
impl Plugin for VoxelMailboxPlugin {
    fn build(&self, _app: &mut App) {
        // app.insert_resource(Time::<Fixed>::from_seconds(0.05))
        //     .add_systems(FixedUpdate, animate);
    }
}

#[derive(Debug, Component, Default)]
pub struct VoxelMailbox(pub VecDeque<(IVec3, GameMaterial, usize)>);
