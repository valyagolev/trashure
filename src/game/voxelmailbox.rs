use std::collections::VecDeque;

use bevy::prelude::*;

use crate::graphics::machines::radar::RadarType;

use super::material::GameMaterial;

pub struct VoxelMailboxPlugin;
impl Plugin for VoxelMailboxPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<VoxelMailbox>();
        // app.insert_resource(Time::<Fixed>::from_seconds(0.05))
        //     .add_systems(FixedUpdate, animate);
    }
}

#[derive(Debug, Component, Default, Reflect)]
pub struct VoxelMailbox(pub VecDeque<(IVec3, GameMaterial, RadarType)>);
