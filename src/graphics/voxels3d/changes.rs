use std::time::Instant;

use bevy::{
    diagnostic::{DiagnosticMeasurement, DiagnosticsStore},
    prelude::*,
    utils::HashMap,
};

use crate::game::material::GameMaterial;

use super::{VoxelBlock, APPLIED_CHANGES, CHANGED_BLOCKS, POSTPONED_CHANGES};

#[derive(Resource, Default)]
pub struct VoxelBlockChanges {
    pub added: HashMap<IVec2, Vec<(IVec3, GameMaterial)>>,
}

impl VoxelBlockChanges {
    pub fn register_change(&mut self, voxel_block_pos: IVec2, inner_pos: IVec3, mat: GameMaterial) {
        let (voxel_block_pos, inner_pos) = VoxelBlock::normalize_pos(voxel_block_pos, inner_pos);

        self.added
            .entry(voxel_block_pos)
            .or_insert_with(Vec::new)
            .push((inner_pos, mat));
    }
}

pub fn apply_changes(
    mut changes: ResMut<VoxelBlockChanges>,
    mut blocks: Query<&mut VoxelBlock>,
    mut diagnostics: ResMut<DiagnosticsStore>,
) {
    let mut total_changes = 0;
    let mut total_postponed = 0;
    let mut changed_blocks = 0;

    let rand = &mut rand::thread_rng();
    let mut new_changes = VoxelBlockChanges::default();

    for mut b in blocks.iter_mut() {
        if let Some(changes) = changes.added.remove(&b.pos) {
            if !changes.is_empty() {
                changed_blocks += 1;
            }

            for (pos, mat) in changes {
                total_changes += 1;
                b.push_block(pos, mat, &mut new_changes, rand)
            }
        }
    }

    for (pos, ch) in new_changes.added.drain() {
        total_postponed += ch.len();
        changes.added.entry(pos).or_insert_with(Vec::new).extend(ch);
    }

    let measurements = [
        (APPLIED_CHANGES, total_changes),
        (POSTPONED_CHANGES, total_postponed),
        (CHANGED_BLOCKS, changed_blocks),
    ];

    for (diagnostic, value) in measurements.iter() {
        diagnostics
            .get_mut(*diagnostic)
            .unwrap()
            .add_measurement(DiagnosticMeasurement {
                time: Instant::now(),
                value: *value as f64,
            });
    }
}
