use bevy::prelude::*;

use crate::graphics::{
    flyingvoxel::FlyingVoxel,
    voxels3d::{
        changes::VoxelBlockChanges, lazyworld::LazyWorld, wholeworld::WholeBlockWorld, VoxelBlock,
    },
};

use super::RadarFoundVoxel;

pub struct RadarConsumptionPlugin;

#[derive(Component)]
pub struct RadarConsumer {
    pub flying_target: Option<Vec3>,
    pub target_mailbox: Entity,
    pub paiload_ix: usize,
}

impl Plugin for RadarConsumptionPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, Self::consume_radars.after(super::radar_search));
    }
}

impl RadarConsumptionPlugin {
    fn consume_radars(
        mut commands: Commands,
        mut q_events: EventReader<RadarFoundVoxel>,
        q_radar_consumers: Query<(&RadarConsumer, &GlobalTransform)>,
        lazy_world: Res<LazyWorld>,
        blocks: Query<&mut VoxelBlock>,
        mut blockchanges: ResMut<VoxelBlockChanges>,
    ) {
        if q_events.is_empty() {
            return;
        }

        let mut whole_world = WholeBlockWorld { lazy_world, blocks };
        let rand = &mut rand::thread_rng();

        for ev in q_events.read() {
            let (cons, tr) = q_radar_consumers.get(ev.radar).unwrap();

            let target = tr.transform_point(cons.flying_target.unwrap_or_default());

            let Some(_mat) = whole_world.steal_block(ev.pos, &mut blockchanges, rand) else {
                continue;
            };

            // if mat != ev.material {
            //     continue;
            //     would have to put back
            // }

            commands.spawn(FlyingVoxel {
                origin: ev.pos.as_vec3(),
                target,
                target_mailbox: cons.target_mailbox,
                material: ev.material,
                payload: (ev.pos, cons.paiload_ix),
            });
        }
    }
}
