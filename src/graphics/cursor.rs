use bevy::{input::mouse::MouseMotion, prelude::*, window::PrimaryWindow};

use super::{
    dbgtext::DebugTexts,
    voxels3d::{VoxelBlock, VOXEL_BLOCK_SIZE},
};

pub struct CursorPlugin;

impl Plugin for CursorPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(CursorOver::default())
            .add_systems(Update, handle_cursor);
    }
}

#[derive(Resource, Debug, Default)]
pub struct CursorOver {
    ground: Vec3,
    block: IVec3,
    lazy_block: (IVec2, IVec3),
    viewport: Vec2,
}

fn handle_cursor(
    q_windows: Query<&Window, With<PrimaryWindow>>,
    q_camera: Query<(&Camera, &GlobalTransform), With<Camera3d>>,

    mut cursor: ResMut<CursorOver>,

    mut texts: ResMut<DebugTexts>,
) {
    if let Some(viewport) = q_windows.single().cursor_position() {
        let (camera, camera_transform) = q_camera.single();

        // Ask Bevy to give us a ray pointing from the viewport (screen) into the world
        let Some(ray) = camera.viewport_to_world(camera_transform, viewport) else {
            // if it was impossible to compute for whatever reason; we can't do anything
            return;
        };

        let Some(distance) = ray.intersect_plane(Vec3::ZERO, Vec3::Y) else {
            // If the ray does not intersect the ground
            // (the camera is not looking towards the ground), we can't do anything
            return;
        };

        let ground = ray.get_point(distance);

        let (block, pos) = VoxelBlock::inner_pos(ground);

        let full_block = (block * VOXEL_BLOCK_SIZE).extend(0).xzy() + pos;

        *cursor = CursorOver {
            ground,
            block: full_block,
            lazy_block: (block, pos),
            viewport,
        };

        texts.set("cursor", format!("{:#?}", cursor));
    }
}
