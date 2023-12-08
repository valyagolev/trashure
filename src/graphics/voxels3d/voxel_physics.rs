use bevy::prelude::*;
use itertools::Itertools;
use rand::{seq::SliceRandom, Rng};

use crate::{conf::Configuration, game::material::GameMaterial, graphics::debug3d};

use super::{VoxelBlock, VoxelBlockChanges, VOXEL_BLOCK_SIZE};

pub struct VoxelPhysics;
impl Plugin for VoxelPhysics {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, handle_debug_keyboard);
    }
}

impl VoxelBlock {
    /// find an empty cell at column
    pub fn empty_at_col(&self, col: IVec2) -> Option<IVec3> {
        if self.forbidden_columns[col.x as usize][col.y as usize] {
            return None;
        }

        let mut empty = IVec3::new(col.x, 0, col.y);

        for y in 0..VOXEL_BLOCK_SIZE {
            empty.y = y;

            if self[empty].is_some() {
                return Some(empty);
            }
        }

        None
    }
}

fn handle_debug_keyboard(keys: Res<Input<KeyCode>>, mut blockchanges: ResMut<VoxelBlockChanges>) {
    if keys.pressed(KeyCode::A) {
        let rnd = &mut rand::thread_rng();

        // let pos = *[
        //     IVec2::new(0, 0),
        //      IVec2::new(10, 0),
        //     IVec2::new(0, 15),
        // ]
        // .choose(rnd)
        // .unwrap();
        let pos = IVec2::new(0, 0);
        // println!("======");
        blockchanges.register_change(
            pos.extend(VOXEL_BLOCK_SIZE).xzy(),
            GameMaterial::random(rnd),
        );
    }
}
