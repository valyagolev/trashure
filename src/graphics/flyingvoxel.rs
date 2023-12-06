use bevy::prelude::*;

use crate::conf::Configuration;

pub struct FlyingVoxelPlugin;
impl Plugin for FlyingVoxelPlugin {
    fn build(&self, app: &mut App) {
        // app.insert_resource(Time::<Fixed>::from_seconds(0.05))
        //     .add_systems(FixedUpdate, animate);
    }
}

#[derive(Debug, Component)]
pub struct FlyingVoxel {
    pub target: IVec3,
    pub target_mailbox: Entity,
}
