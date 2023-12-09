use bevy::{prelude::*, render::mesh::shape::Plane, time::Stopwatch};
use rand::prelude::Rng;

use crate::{
    conf::Configuration,
    game::{material::GameMaterial, voxelmailbox::VoxelMailbox},
};

use super::{
    debug3d,
    machines::MachineResources,
    voxels3d::{VoxelResources, VOXEL_BLOCK_SIZE},
};

pub struct FlyingVoxelPlugin;
impl Plugin for FlyingVoxelPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (initialize_voxel, fly_voxel));
    }
}

#[derive(Debug, Component)]
pub struct FlyingVoxel {
    pub origin: Vec3,
    pub target: Vec3,
    pub target_mailbox: Entity,
    pub material: GameMaterial,
    pub payload: (IVec3, usize),
}

#[derive(Debug, Component)]
struct FlyingVoxelState {
    a: f32,
    b: f32,
    t: Stopwatch,
    max_t: f32,
}

fn initialize_voxel(
    mut commands: Commands,
    q_fv: Query<(Entity, &FlyingVoxel), Without<FlyingVoxelState>>,
    res: Res<MachineResources>,
    vres: Res<VoxelResources>,
) {
    for (e, fv) in q_fv.iter() {
        // println!("new target:{:?}", fv.target_mailbox);

        let target_reorig = fv.target - fv.origin;
        let target_reorig_vertical_plane: Vec2 =
            Vec2::new(target_reorig.xz().length(), target_reorig.y);

        let (x0, y0) = target_reorig_vertical_plane.into();

        let y1 = {
            let min_max_y = target_reorig.y * 0.2;

            let max_max_y = (VOXEL_BLOCK_SIZE as f32) * 0.8;

            if max_max_y <= min_max_y {
                min_max_y
            } else {
                rand::thread_rng().gen_range(min_max_y..max_max_y)
            }
        };
        let x1 = x0 / 2.0;

        let a = (-x0 * y1 + x1 * y0) / (x0 * x0 * x1 - x0 * x1 * x1);
        let b = (y0 - a * x0.powi(2)) / x0;

        commands.entity(e).insert((
            FlyingVoxelState {
                a,
                b,
                t: Stopwatch::new(),
                max_t: target_reorig.length() * rand::thread_rng().gen_range(0.5..1.0) / 5.0,
            },
            PbrBundle {
                mesh: res.cube.clone(),
                material: vres.material_handles[fv.material.as_usize()].clone(),
                // transform: Transform::from_scale(Vec3::new(tp.dims.x as f32, 32.0, tp.dims.y as f32)),
                ..default()
            },
        ));
    }
}
fn fly_voxel(
    mut commands: Commands,
    time: Res<Time>,
    mut q_fvs: Query<(Entity, &FlyingVoxel, &mut FlyingVoxelState, &mut Transform)>,
    mut q_mailboxes: Query<&mut VoxelMailbox>,
) {
    for (e, fv, mut fvs, mut tr) in q_fvs.iter_mut() {
        fvs.t.tick(time.delta());

        let t = fvs.t.elapsed().as_secs_f32() / fvs.max_t;

        if t >= 1.0 {
            commands.entity(e).despawn_recursive();

            // println!("sedning to: {:?}", fv.target_mailbox);

            let mut mb = q_mailboxes.get_mut(fv.target_mailbox).unwrap();

            mb.0.push_back((fv.payload.0, fv.material, fv.payload.1));
        }

        let target = fv.target;
        let origin = fv.origin;

        let target_reorig = target - origin;
        let target_reorig_vertical_plane: Vec2 =
            Vec2::new(target_reorig.xz().length(), target_reorig.y);

        let plane_x = target_reorig_vertical_plane.x * t;
        let y = fvs.a * plane_x.powi(2) + fvs.b * plane_x;

        let real_xz = plane_x * target_reorig.xz().normalize();

        let real_pos = origin + Vec3::new(real_xz.x, y, real_xz.y);

        tr.translation = real_pos;

        // debug3d::draw_gizmos(1.0, move |gz| {
        //     gz.sphere(real_pos, Quat::IDENTITY, 0.1, Color::RED);
        // });
    }
}
