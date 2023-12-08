use std::ops::Index;

use bevy::prelude::*;
use bevy_meshem::prelude::get_neighbor;
use bevy_meshem::prelude::Face;
use bevy_meshem::prelude::MeshMD;
use bevy_meshem::Dimensions;

use crate::game::material::GameMaterial;
use crate::graphics::voxels3d::CHUNK_LEN;
use crate::graphics::voxels3d::VOXEL_BLOCK_SIZE;

#[derive(Component)]
pub struct VoxelBlock {
    pub pos: IVec2,
    pub meta: MeshMD<Option<GameMaterial>>,
    pub grid: [Option<GameMaterial>; CHUNK_LEN],
    pub mesh_id: AssetId<Mesh>,
    pub forbidden_columns: [[bool; VOXEL_BLOCK_SIZE as usize]; VOXEL_BLOCK_SIZE as usize],
}

impl VoxelBlock {
    const DIMENSIONS: Dimensions = (
        VOXEL_BLOCK_SIZE as usize,
        VOXEL_BLOCK_SIZE as usize,
        VOXEL_BLOCK_SIZE as usize,
    );

    fn _meshem_neighbors(&self, idx: usize) -> [Option<Option<GameMaterial>>; 6] {
        let mut r = [None; 6];
        for i in 0..6 {
            match get_neighbor(idx, Face::from(i), Self::DIMENSIONS) {
                None => {}
                Some(j) => r[i] = Some(self.grid[j]),
            }
        }
        r
    }

    pub fn _add_block(&mut self, local_pos: IVec3, mat: GameMaterial) {
        assert!(
            Self::within_bounds(local_pos),
            "local_pos out of bounds: {local_pos:?}"
        );
        // no idea why...
        let local_pos_fixed = local_pos.xzy();
        // dbg!(&local_pos);
        let idx = (local_pos_fixed.x
            + local_pos_fixed.y * VOXEL_BLOCK_SIZE
            + local_pos_fixed.z * VOXEL_BLOCK_SIZE * VOXEL_BLOCK_SIZE) as usize;
        // dbg!(&idx);

        // assert!(idx >= 0);

        assert!(self.grid[idx].is_none());
        assert!(!self.forbidden_columns[local_pos.x as usize][local_pos.z as usize]);

        self.grid[idx] = Some(mat);
        self.meta.log(
            bevy_meshem::prelude::VoxelChange::Added,
            idx,
            Some(mat),
            self._meshem_neighbors(idx),
        )
    }

    pub fn _take_block(&mut self, local_pos: IVec3) -> Option<GameMaterial> {
        assert!(
            Self::within_bounds(local_pos),
            "local_pos out of bounds: {local_pos:?}"
        );
        // no idea why...
        let local_pos_fixed = local_pos.xzy();
        // dbg!(&local_pos);
        let idx = (local_pos_fixed.x
            + local_pos_fixed.y * VOXEL_BLOCK_SIZE
            + local_pos_fixed.z * VOXEL_BLOCK_SIZE * VOXEL_BLOCK_SIZE) as usize;
        // dbg!(&idx);

        // assert!(idx >= 0);

        self.meta.log(
            bevy_meshem::prelude::VoxelChange::Broken,
            idx,
            None,
            self._meshem_neighbors(idx),
        );
        self.grid[idx].take()
    }

    pub fn within_bounds(pos: IVec3) -> bool {
        pos.x >= 0
            && pos.x < VOXEL_BLOCK_SIZE
            && pos.y >= 0
            && pos.y < VOXEL_BLOCK_SIZE
            && pos.z >= 0
            && pos.z < VOXEL_BLOCK_SIZE
    }

    pub fn normalize_pos(mut voxel_block_pos: IVec2, mut inner_pos: IVec3) -> (IVec2, IVec3) {
        while inner_pos.x < 0 {
            voxel_block_pos.x -= 1;
            inner_pos.x += VOXEL_BLOCK_SIZE;
        }
        while inner_pos.z < 0 {
            voxel_block_pos.y -= 1;
            inner_pos.z += VOXEL_BLOCK_SIZE;
        }
        while inner_pos.x >= VOXEL_BLOCK_SIZE {
            voxel_block_pos.x += 1;
            inner_pos.x -= VOXEL_BLOCK_SIZE;
        }
        while inner_pos.z >= VOXEL_BLOCK_SIZE {
            voxel_block_pos.y += 1;
            inner_pos.z -= VOXEL_BLOCK_SIZE;
        }

        (voxel_block_pos, inner_pos)
    }

    pub fn forbid_column(&mut self, local_pos: IVec2) {
        for y in 0..VOXEL_BLOCK_SIZE {
            assert!(self[local_pos.extend(y).xzy()].is_none());
        }

        self.forbidden_columns[local_pos.x as usize][local_pos.y as usize] = true;
    }

    pub fn real_pos(voxel_block_pos: IVec2, inner_pos: IVec3) -> Vec3 {
        ((voxel_block_pos * VOXEL_BLOCK_SIZE).extend(0) + inner_pos).as_vec3()
    }

    pub fn inner_pos(pos: Vec3) -> (IVec2, IVec3) {
        let voxel_block_pos = (pos.xz() / VOXEL_BLOCK_SIZE as f32).as_ivec2();
        let inner_pos = (pos.xz() % VOXEL_BLOCK_SIZE as f32)
            .as_ivec2()
            .extend(0)
            .xzy();

        (voxel_block_pos, inner_pos)
    }

    pub fn is_column_empty(&self, pos: IVec2) -> bool {
        assert!(pos.x >= 0 && pos.x < VOXEL_BLOCK_SIZE);

        self[pos.extend(0).xzy()].is_none()
    }

    /// pos in *local* coordinates
    pub fn closest_columns(&self, pos: IVec2, dist: f32) -> impl Iterator<Item = IVec2> {
        let dist_sq = dist * dist;
        let pos = pos.as_vec2();

        (0..VOXEL_BLOCK_SIZE)
            .map(|x| (0..VOXEL_BLOCK_SIZE).map(move |z| IVec2::new(x, z)))
            .flatten()
            .filter(move |col| {
                let dist = (col.as_vec2() - pos).length_squared();

                dist <= dist_sq
            })
    }

    pub fn material_in_col(
        &self,
        pos: IVec2,
        mask: u8,
    ) -> impl Iterator<Item = (GameMaterial, IVec3)> + '_ {
        (0..VOXEL_BLOCK_SIZE)
            .map(move |y| pos.extend(y).xzy())
            .filter_map(move |p| self[p].map(|mat| (mat, p)))
            .filter(move |(mat, _)| mat.mask_contains(mask))
    }
}

impl Index<IVec3> for VoxelBlock {
    type Output = Option<GameMaterial>;

    fn index(&self, index: IVec3) -> &Self::Output {
        let local_pos_fixed = index.xzy();

        let idx = (local_pos_fixed.x
            + local_pos_fixed.y * VOXEL_BLOCK_SIZE
            + local_pos_fixed.z * VOXEL_BLOCK_SIZE * VOXEL_BLOCK_SIZE) as usize;

        &self.grid[idx]
    }
}
