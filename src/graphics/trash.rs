use bevy::prelude::*;

use crate::conf::Configuration;

use super::atlases::{AtlasesPluginState, Emojis};

pub struct TrashExperimentPlugin;

impl Plugin for TrashExperimentPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AtlasesPluginState::Finished), setup)
            .add_systems(
                Update,
                camera_setup.run_if(in_state(AtlasesPluginState::Finished)),
            );
    }
}

fn camera_setup(
    conf: Res<Configuration>,
    mut camera: Query<(&mut OrthographicProjection, With<Camera2d>)>,
) {
    if conf.is_changed() {
        println!("camera_setup changed");
        for (mut proj, _) in camera.iter_mut() {
            print!("chanign camera scale to {}", conf.camera_scale);
            proj.scale = conf.camera_scale;
        }
    }
}

fn setup(mut commands: Commands, emojis: Res<Emojis>, conf: Res<Configuration>) {
    commands.spawn(Camera2dBundle {
        transform: Transform::from_xyz(0.0, 0.0, 1.0),
        projection: OrthographicProjection {
            scale: conf.camera_scale,
            ..default()
        },
        ..default()
    });

    commands.spawn(SpriteSheetBundle {
        transform: Transform {
            translation: Vec3::new(150.0, 0.0, 0.0),
            scale: Vec3::splat(1.0),
            ..default()
        },
        ..emojis.sbundle("🎁").unwrap()
    });

    commands.spawn(SpriteSheetBundle {
        transform: Transform {
            translation: Vec3::new(200.0, 0.0, 0.0),
            scale: Vec3::splat(1.0),
            ..default()
        },
        ..emojis.sbundle("🎑").unwrap()
    });
}
