use std::borrow::Cow;

use bevy::{
    prelude::*,
    render::view::RenderLayers,
    scene::{SceneInstance, SceneInstanceReady},
    transform::commands,
    utils::HashMap,
};
use itertools::Itertools;

pub struct SceneObjectFinderPlugin;

impl Plugin for SceneObjectFinderPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, Self::find_scene_objects);
    }
}

#[derive(Component)]
pub struct SceneObjectFinder(Vec<Cow<'static, str>>);

#[derive(Component)]
pub struct SceneObjectsFound(HashMap<Cow<'static, str>, Entity>);

impl SceneObjectFinder {
    pub fn new<S: Into<Cow<'static, str>>>(strs: impl IntoIterator<Item = S>) -> Self {
        Self(strs.into_iter().map(|s| s.into()).collect())
    }
}

impl SceneObjectFinderPlugin {
    fn find_scene_objects(
        mut commands: Commands,
        mut targets: Query<(Entity, &SceneInstance, &mut SceneObjectFinder)>,
        spawner: Res<SceneSpawner>,
        names: Query<&Name>,
    ) {
        for (e, scene, mut objects) in targets.iter_mut() {
            if !spawner.instance_is_ready(**scene) {
                continue;
            }

            let mut hm = HashMap::default();

            for ent in spawner.iter_instance_entities(**scene) {
                let Ok(name) = names.get(ent) else {
                    continue;
                };
                let name: &str = &*name;

                // println!("checking {}", name);

                if let Some((i, _)) = objects.0.iter().find_position(|s| s == &&name) {
                    let name = objects.0.remove(i);

                    // println!("found {}", name);

                    hm.insert(name, ent);
                }
            }

            commands
                .entity(e)
                .insert(SceneObjectsFound(hm))
                .remove::<SceneObjectFinder>();
        }
    }
}
