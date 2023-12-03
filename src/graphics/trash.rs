use bevy::prelude::*;

use super::atlases::{AtlasesPluginState, Emojis};

pub struct TrashExperimentPlugin;

impl Plugin for TrashExperimentPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AtlasesPluginState::Finished), setup);
    }
}

fn setup(mut commands: Commands, emojis: Res<Emojis>) {
    commands.spawn(Camera2dBundle::default());

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
