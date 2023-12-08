use std::f32::consts::PI;

use bevy::{prelude::*, time::Stopwatch};
use rand::{seq::SliceRandom, Rng};

use crate::{
    game::{material::GameMaterial, Direction2D},
    graphics::{
        debug3d,
        recolor::Tinted,
        voxels3d::{lazyworld::LazyWorld, VoxelBlock, VOXEL_BLOCK_SIZE},
    },
};

use super::{DebugCube, MachineResources};

pub struct RadarPlugin;

impl Plugin for RadarPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                setup_radars,
                // radar_search.after(setup_radars),
                redraw_radars.after(radar_search),
            ),
        )
        .add_systems(FixedUpdate, radar_search);
    }
}

#[derive(Bundle)]
pub struct RadarBundle {
    radar: Radar,
    transform_bundle: TransformBundle,
}

impl RadarBundle {
    pub fn new(mats: &[GameMaterial], direction: Option<Direction2D>) -> Self {
        RadarBundle {
            radar: Radar::new(mats, direction),
            transform_bundle: TransformBundle::default(),
        }
    }
}

#[derive(Component)]
pub struct Radar {
    material_mask: u8,
    watch: Stopwatch,
    scene: Option<Entity>,

    found_voxel: Option<(GameMaterial, IVec3)>,

    pub direction: Option<Direction2D>,
}

impl Radar {
    pub fn take_voxel(&mut self) -> Option<(GameMaterial, IVec3)> {
        let v = self.found_voxel.take()?;

        self.watch.reset();

        Some(v)
    }
}

#[derive(Component)]
pub struct RadarScene;

impl Radar {
    pub fn new(mats: &[GameMaterial], direction: Option<Direction2D>) -> Self {
        Radar {
            material_mask: GameMaterial::any_of_mask(mats),
            watch: Stopwatch::new(),
            scene: None,
            found_voxel: None,
            direction,
        }
    }

    fn dist(&self) -> f32 {
        // 30.0 * ((self.watch.elapsed().as_secs_f32() / 5.0).sin()).abs()
        self.watch.elapsed().as_secs_f32() * 5.0 * 6.0
    }
}

fn setup_radars(
    mut commands: Commands,
    mut q_radars: Query<(Entity, &mut Radar), Added<Radar>>,
    res: Res<MachineResources>,
) {
    for (e, mut r) in q_radars.iter_mut() {
        let radar_e = commands
            .spawn((
                RadarScene,
                SceneBundle {
                    transform: Transform::from_rotation(Quat::from_rotation_y(-PI / 4.0)),
                    scene: res.radar.clone(),
                    ..Default::default()
                },
                Tinted::new(Color::rgba(0.7, 0.3, 0.3, 0.2)).alpha(),
            ))
            .id();
        r.scene = Some(radar_e);
        commands.entity(e).add_child(radar_e);
    }
}

fn radar_search(
    time: Res<Time>,
    mut q_radars: Query<(&mut Radar, &Parent, &GlobalTransform)>,
    q_parent_dirs: Query<&Direction2D, With<Children>>,
    lazyworld: Res<LazyWorld>,
    q_blocks: Query<&VoxelBlock>,
) {
    let rand = &mut rand::thread_rng();

    for (mut r, rpar, gt) in q_radars.iter_mut() {
        if r.found_voxel.is_some() {
            continue;
        }

        let rpardir = q_parent_dirs.get(**rpar).unwrap();

        // println!("checking radar: {:?}", r);

        r.watch.tick(time.delta());

        let dist = r.dist();

        // make it more interesting lmao
        let dist = dist + rand.gen_range(0.0..3.0);

        let radar_ipos = gt.translation().xz().as_ivec2();

        let mut candidates = vec![];

        for (bigblock_pos, ent) in lazyworld.lookup_around(radar_ipos, dist) {
            // dbg!(bigblock_pos, ent);
            let Ok(voxel_block) = q_blocks.get(ent) else {
                continue;
            };

            let local_pos = radar_ipos - bigblock_pos * VOXEL_BLOCK_SIZE;

            for col in voxel_block.closest_columns(local_pos, dist) {
                // dbg!(col);
                for (mat, pos) in voxel_block.material_in_col(col, r.material_mask) {
                    if let Some(dir) = r.direction {
                        let radar_local_pos = (bigblock_pos * VOXEL_BLOCK_SIZE) - radar_ipos + col;

                        // println!("radar_local_pos={:?} ent={ent:?}", radar_local_pos);
                        if !(dir * *rpardir).within_cone(radar_local_pos) {
                            // println!("not within cone");
                            continue;
                        }
                    }

                    let full_pos = (bigblock_pos * VOXEL_BLOCK_SIZE).extend(0).xzy() + pos;

                    // r.found_voxel = Some((mat, full_pos));

                    // let real_dist =
                    //     (full_pos.xz().as_vec2() - radar_ipos.as_vec2()).length_squared();

                    // println!(
                    //     "found voxel: {:?} dist={dist} real_dist={real_dist} radar_local_pos={local_pos} found_local_pos={col}",
                    //     r.found_voxel
                    // );

                    // continue 'radars;

                    candidates.push((mat, full_pos));
                }
            }
        }

        if !candidates.is_empty() {
            let winner = candidates.choose(rand).unwrap();

            r.found_voxel = Some(*winner);
        }
    }
}

fn redraw_radars(
    q_radars: Query<(&Radar, &Children)>,
    mut q_scenes: Query<(&mut Transform, &GlobalTransform), (With<RadarScene>, Without<DebugCube>)>,
    // mut q_cubes: Query<&mut Transform, (With<DebugCube>, Without<RadarScene>)>,
) {
    for (r, _children) in q_radars.iter() {
        let Ok((mut t, gl)) = q_scenes.get_mut(r.scene.unwrap()) else {
            continue;
        };

        // println!("redrawing radar: {:?}", r);

        let dist = r.dist();

        // 2.0 is the scale of the radar scene
        t.scale = Vec3::splat(dist * 2.0);
    }
}
