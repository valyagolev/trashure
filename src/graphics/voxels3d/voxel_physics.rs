use bevy::prelude::*;
use itertools::Itertools;
use rand::{seq::SliceRandom, Rng};

use crate::{conf::Configuration, game::material::GameMaterial};

use super::{VoxelBlock, VOXEL_BLOCK_SIZE};

pub struct VoxelPhysics;
impl Plugin for VoxelPhysics {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, handle_debug_keyboard);
    }
}

static DIRS_AROUND: &[IVec2] = &[
    IVec2::new(-1, -1),
    IVec2::new(-1, 0),
    IVec2::new(-1, 1),
    IVec2::new(0, -1),
    IVec2::new(0, 1),
    IVec2::new(1, -1),
    IVec2::new(1, 0),
    IVec2::new(1, 1),
];

static AROUND_AND_THIS: &[IVec2] = &[
    IVec2::new(-1, -1),
    IVec2::new(-1, 0),
    IVec2::new(-1, 1),
    IVec2::new(0, -1),
    IVec2::new(0, 0),
    IVec2::new(0, 1),
    IVec2::new(1, -1),
    IVec2::new(1, 0),
    IVec2::new(1, 1),
];

impl VoxelBlock {
    pub fn drop_block(&mut self, pos_xz: IVec2, mat: GameMaterial, rand: &mut impl Rng) {
        let z_place = (0..VOXEL_BLOCK_SIZE)
            .filter(|z| self[IVec3::new(pos_xz.x, *z, pos_xz.y)].is_none())
            .next();

        let Some(z_place) = z_place else {
            let close = DIRS_AROUND
                .iter()
                .map(|p| *p + pos_xz)
                .filter(|p| Self::within_bounds(p.extend(0)))
                .collect_vec();

            let close = close.choose(rand).unwrap();
            return self.drop_block(*close, mat, rand);
        };

        let empty_belows = AROUND_AND_THIS
            .iter()
            .map(|p| IVec3::new(pos_xz.x + p.x, z_place - 1, pos_xz.y + p.y))
            .filter(|p| Self::within_bounds(*p))
            .filter(|p| self[*p].is_none())
            .collect_vec();

        if rand.gen_range(0..9) < empty_belows.len()
            || rand.gen_range(0..9) < empty_belows.len()
            || rand.gen_range(0..9) < empty_belows.len()
        {
            let below = empty_belows.choose(rand).unwrap();
            return self.drop_block(below.xz(), mat, rand);
        }

        self._add_block(IVec3::new(pos_xz.x, z_place, pos_xz.y), mat);
    }
}

fn handle_debug_keyboard(
    keys: Res<Input<KeyCode>>,
    mut commands: Commands,
    mut q_voxel_blocks: Query<&mut VoxelBlock>,

    conf: Res<Configuration>,
) {
    if keys.pressed(KeyCode::A) {
        let rnd = &mut rand::thread_rng();
        let mut q_vb = q_voxel_blocks.iter_mut().next().unwrap();
        // let x = rnd.gen_range(-10..10);
        // let y = rnd.gen_range(-10..10);
        // let pos = IVec3::new(x, y, 0);

        let pos = [IVec2::new(0, 0), IVec2::new(10, 0), IVec2::new(0, 15)]
            .choose(rnd)
            .unwrap()
            .clone();

        q_vb.drop_block(pos, GameMaterial::random(rnd), rnd);
    }
}
