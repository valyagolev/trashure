use bevy::prelude::*;
use itertools::Itertools;
use rand::{seq::SliceRandom, Rng};

use crate::{conf::Configuration, game::material::GameMaterial};

use super::{VoxelBlock, VoxelBlockChanges, VOXEL_BLOCK_SIZE};

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
    pub fn drop_block(
        &mut self,
        pos_xz: IVec2,
        mat: GameMaterial,
        change_collector: &mut VoxelBlockChanges,
        rand: &mut impl Rng,
    ) {
        if !Self::within_bounds(pos_xz.extend(0)) {
            change_collector.register_change(
                self.pos,
                pos_xz.extend(VOXEL_BLOCK_SIZE - 1).xzy(),
                mat,
            );
            return;
        }

        let z_place =
            (0..VOXEL_BLOCK_SIZE).find(|z| self[IVec3::new(pos_xz.x, *z, pos_xz.y)].is_none());

        let Some(z_place) = z_place else {
            let close = DIRS_AROUND.iter().map(|p| *p + pos_xz).collect_vec();

            let close = close.choose(rand).unwrap();

            return self.drop_block(*close, mat, change_collector, rand);
        };

        self.push_block(pos_xz.extend(z_place).xzy(), mat, change_collector, rand);
    }

    pub fn push_block(
        &mut self,
        local_pos: IVec3,
        mat: GameMaterial,
        change_collector: &mut VoxelBlockChanges,
        rand: &mut impl Rng,
    ) {
        if !Self::within_bounds(local_pos) {
            change_collector.register_change(self.pos, local_pos, mat);
            return;
        }

        if local_pos.y > 0 {
            let empty_belows = AROUND_AND_THIS
                .iter()
                .map(|p| p.extend(-1).xzy() + local_pos)
                .filter(|p| !Self::within_bounds(*p) || self[*p].is_none())
                .collect_vec();

            if rand.gen_range(0..9) < empty_belows.len()
                || rand.gen_range(0..9) < empty_belows.len()
                || rand.gen_range(0..9) < empty_belows.len()
            {
                let below = empty_belows.choose(rand).unwrap();

                return self.drop_block(below.xz(), mat, change_collector, rand);
            }
        }

        if self.forbidden_columns[local_pos.x as usize][local_pos.z as usize]
            || self[local_pos].is_some()
        {
            let arounds = DIRS_AROUND
                .iter()
                .map(|p| p.extend(1).xzy() + local_pos)
                .filter(|p| !Self::within_bounds(*p) || self[*p].is_none())
                .collect_vec();

            let pos = arounds.choose(rand).unwrap();

            return self.push_block(*pos, mat, change_collector, rand);
        }

        self._add_block(local_pos, mat);
    }

    pub fn steal_block(
        &mut self,
        local_pos: IVec3,
        change_collector: &mut VoxelBlockChanges,
        rand: &mut impl Rng,
    ) -> GameMaterial {
        assert!(Self::within_bounds(local_pos));

        let mt = self
            ._take_block(local_pos)
            .take()
            .expect("Stealing an empty block");

        let mut mats = vec![];

        for y in local_pos.y + 1..VOXEL_BLOCK_SIZE {
            let lp = IVec3::new(local_pos.x, y, local_pos.z);

            mats.push(self._take_block(lp).take());
        }

        let mut y = local_pos.y;

        for m in mats {
            if let Some(m) = m {
                let lp = IVec3::new(local_pos.x, y, local_pos.z);
                y += 1;

                self._add_block(lp, m);
            }
        }

        mt
    }
}

fn handle_debug_keyboard(
    keys: Res<Input<KeyCode>>,
    _commands: Commands,
    mut q_voxel_blocks: Query<&mut VoxelBlock>,

    _conf: Res<Configuration>,
    mut blockchanges: ResMut<VoxelBlockChanges>,
) {
    if keys.pressed(KeyCode::A) {
        let rnd = &mut rand::thread_rng();
        let mut q_vb = q_voxel_blocks.iter_mut().next().unwrap();
        // let x = rnd.gen_range(-10..10);
        // let y = rnd.gen_range(-10..10);
        // let pos = IVec3::new(x, y, 0);

        let pos = *[IVec2::new(0, 0), IVec2::new(10, 0), IVec2::new(0, 15)]
            .choose(rnd)
            .unwrap();

        // let mut changes = VoxelBlockChanges::outof(&blockchanges);

        q_vb.drop_block(pos, GameMaterial::random(rnd), &mut blockchanges, rnd);

        // changes.drain(&mut blockchanges);
    }
}
