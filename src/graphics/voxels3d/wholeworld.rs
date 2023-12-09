pub use bevy::prelude::*;
use bevy::{ecs::query::WorldQuery, tasks::ParallelIterator};
use itertools::Itertools;
use rand::{seq::SliceRandom, Rng};

use crate::{game::material::GameMaterial, graphics::debug3d};

use super::{changes::VoxelBlockChanges, lazyworld::LazyWorld, VoxelBlock, VOXEL_BLOCK_SIZE};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BlockState {
    Empty,
    Full(GameMaterial),
    Forbidden,
}

impl BlockState {
    pub fn is_full(&self) -> bool {
        matches!(self, BlockState::Full(_))
    }
}

pub struct WholeBlockWorld<'qres, 'qq, 'world, 'state> {
    pub lazy_world: Res<'qres, LazyWorld>,
    pub blocks: Query<'world, 'state, &'qq mut VoxelBlock>,
}

fn blocks_around(pos: IVec3, dist: i32) -> impl Iterator<Item = IVec3> {
    (-dist..=dist)
        .cartesian_product(-dist..=dist)
        .filter(move |(x, z)| x.abs() == dist || z.abs() == dist)
        .map(move |(x, z)| pos + IVec3::new(x, 0, z))
}

impl<'qres, 'qq, 'world, 'state> WholeBlockWorld<'qres, 'qq, 'world, 'state> {
    pub fn is_initialized_by_blockpos(&self, block_pos: IVec2) -> bool {
        self.lazy_world.known_parts.contains_key(&block_pos)
            && self
                .blocks
                .get(self.lazy_world.known_parts[&block_pos])
                .is_ok()
    }

    pub fn is_in_forbidden_column(&mut self, global_pos: IVec3) -> bool {
        let Some((block, local_pos)) = self.get_voxel_block_for_pos(global_pos) else {
            return false;
        };

        block.forbidden_columns[local_pos.x as usize][local_pos.z as usize]
    }

    pub fn get_voxel_block_for_pos(
        &mut self,
        global_pos: IVec3,
    ) -> Option<(Mut<'_, VoxelBlock>, IVec3)> {
        let (block_pos, local_pos) = VoxelBlock::normalize_pos(IVec2::ZERO, global_pos);

        let block = self.lazy_world.known_parts.get(&block_pos)?;

        let block = self.blocks.get_mut(*block).unwrap();

        Some((block, local_pos))
    }

    pub fn get_block_value(&mut self, global_pos: IVec3) -> BlockState {
        if self.is_in_forbidden_column(global_pos) {
            return BlockState::Forbidden;
        }

        let Some((block, local_pos)) = self.get_voxel_block_for_pos(global_pos) else {
            return BlockState::Empty;
        };

        if let Some(mat) = block[local_pos] {
            BlockState::Full(mat)
        } else {
            BlockState::Empty
        }
    }

    pub fn steal_block(
        &mut self,
        global_pos: IVec3,
        change_collector: &mut VoxelBlockChanges,
        rand: &mut impl Rng,
    ) -> Option<GameMaterial> {
        let (mut block, local_pos) = self.get_voxel_block_for_pos(global_pos)?;

        let block_pos = block.pos;

        let mt = block._take_block(local_pos).take()?;

        let mut mats = vec![];

        for y in local_pos.y + 1..VOXEL_BLOCK_SIZE {
            let lp = IVec3::new(local_pos.x, y, local_pos.z);

            mats.push(block._take_block(lp).take());
        }

        let mut y = local_pos.y;

        for m in mats {
            if let Some(m) = m {
                let lp = IVec3::new(local_pos.x, y, local_pos.z);
                y += 1;

                let gp = (block_pos * VOXEL_BLOCK_SIZE).extend(0).xzy() + lp;

                // self._add_block(lp, m);
                self.push_block(gp, m, change_collector, rand, Some(Color::FUCHSIA));
            }
        }

        Some(mt)
    }

    pub fn push_block(
        &mut self,
        global_pos: IVec3,
        mat: GameMaterial,
        change_collector: &mut VoxelBlockChanges,
        rand: &mut impl Rng,
        debug_color: Option<Color>,
    ) {
        // dbg!(global_pos);

        if global_pos.y >= VOXEL_BLOCK_SIZE {
            if self.get_block_value(global_pos - IVec3::new(0, 1, 0)) == BlockState::Empty {
                // println!("1");
                return self.push_block(
                    // global_pos - IVec3::new(0, 1, 0),
                    IVec3::new(global_pos.x, VOXEL_BLOCK_SIZE - 1, global_pos.z),
                    mat,
                    change_collector,
                    rand,
                    debug_color,
                );
            }

            for rad in 1..30 {
                let empties_below = blocks_around(global_pos - IVec3::new(0, 1, 0), rad)
                    .filter(|p| self.get_block_value(*p) == BlockState::Empty)
                    .collect_vec();

                if !empties_below.is_empty() {
                    // println!("2:{rad}");
                    return self.push_block(
                        *empties_below.choose(rand).unwrap(),
                        mat,
                        change_collector,
                        rand,
                        debug_color,
                    );
                }
            }

            warn!("no empty space found below, discarding block");
            return;
        }

        if global_pos.y > 0 {
            let cells_below = blocks_around(global_pos - IVec3::new(0, 1, 0), 1)
                .chain([global_pos - IVec3::new(0, 1, 0)])
                .map(|p| (p, self.get_block_value(p)))
                .collect_vec();

            let empties_below = cells_below
                .iter()
                .filter(|(_, s)| *s == BlockState::Empty)
                .copied()
                .collect_vec();

            let allowed_below = cells_below
                .iter()
                .filter(|(_, s)| *s != BlockState::Forbidden)
                .count();

            if empties_below.len() == allowed_below {
                // println!("3");
                // don't wiggle in the air
                return self.push_block(
                    global_pos - IVec3::new(0, 1, 0),
                    mat,
                    change_collector,
                    rand,
                    debug_color,
                );
            }

            // println!("empties below: {}/{allowed_below} ", empties_below.len());

            let must_fall = empties_below.len() > 0
                && (self.get_block_value(global_pos - IVec3::new(0, 1, 0)) == BlockState::Empty
                    || empties_below.len() >= allowed_below - 1
                    || rand.gen_range(0..=allowed_below) > empties_below.len()
                    || rand.gen_range(0..=allowed_below) > empties_below.len()
                    || rand.gen_range(0..=allowed_below) > empties_below.len());

            if must_fall {
                // println!("4");
                return self.push_block(
                    empties_below.choose(rand).unwrap().0,
                    mat,
                    change_collector,
                    rand,
                    debug_color,
                );
            }
        }

        if self.get_block_value(global_pos) == BlockState::Empty {
            if let Some((mut block, local_pos)) = self.get_voxel_block_for_pos(global_pos) {
                // debug3d::draw_gizmos(2.0, move |gizmos| {
                //     gizmos.sphere(
                //         global_pos.as_vec3(),
                //         Quat::IDENTITY,
                //         3.0,
                //         debug_color.unwrap_or(Color::BLUE),
                //     );
                // });

                block._add_block(local_pos, mat);
            } else {
                change_collector.register_change(global_pos, mat);
            }

            return;
        }

        for rad in 1..30 {
            let mut empties_around = blocks_around(global_pos, rad)
                .filter(|p| self.get_block_value(*p) == BlockState::Empty)
                .collect_vec();

            let global_pos_above = global_pos + IVec3::new(0, 2, 0);

            empties_around.push(global_pos_above);
            empties_around.push(global_pos_above);
            empties_around.push(global_pos_above);

            // if !empties_around.is_empty() {
            // println!("6 :{rad}");
            return self.push_block(
                *empties_around.choose(rand).unwrap(),
                mat,
                change_collector,
                rand,
                debug_color,
            );
            // }
        }

        warn!("no empty space found, discarding block");
        return;
    }

    pub fn drop_block(
        &mut self,
        global_pos_xz: IVec2,
        mat: GameMaterial,
        change_collector: &mut VoxelBlockChanges,
        rand: &mut impl Rng,
    ) {
        self.push_block(
            global_pos_xz.extend(VOXEL_BLOCK_SIZE).xzy(),
            mat,
            change_collector,
            rand,
            Some(Color::RED),
        );
    }
}

// pub fn apply_changes(
//     mut changes: ResMut<VoxelBlockChanges>,
//     mut blocks: Query<&mut VoxelBlock>,
//     lazy_world: Res<LazyWorld>,
//     // mut diagnostics: ResMut<DiagnosticsStore>,
// ) {
//     let whole_world = WholeBlockWorld {
//         lazy_world: &lazy_world,
//         blocks: &mut blocks,
//     };
// }
