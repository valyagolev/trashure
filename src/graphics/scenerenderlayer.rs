use bevy::{prelude::*, render::view::RenderLayers, scene::SceneInstance};

pub struct SceneRenderLayersPlugin;

#[derive(Component)]
pub struct SceneRenderLayers(pub RenderLayers);

impl Plugin for SceneRenderLayersPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, Self::update_render_layers);
    }
}

impl SceneRenderLayersPlugin {
    fn update_render_layers(
        mut commands: Commands,
        q_scenes: Query<
            (&SceneInstance, &SceneRenderLayers),
            Or<(Changed<SceneInstance>, Changed<SceneRenderLayers>)>,
        >,
        scene_spawner: Res<SceneSpawner>,
    ) {
        for (inst, layers) in q_scenes.iter() {
            for i in scene_spawner.iter_instance_entities(**inst) {
                commands.entity(i).insert(layers.0);
            }
        }
    }

    // fn on_scene_load(
    //     mut commands: Commands,
    //     mut events: EventReader<SceneInstanceReady>,
    //     targets: Query<&SceneInstance, With<TargetInst>>,
    //     spawner: Res<SceneSpawner>,
    // ) {
    //     for event in events.read() {
    //         let Ok(scene) = targets.get(event.parent) else {
    //             continue;
    //         };

    //         for i in spawner.iter_instance_entities(**scene) {
    //             commands.entity(i).insert(RenderLayers::layer(6));
    //         }
    //     }
    // }
}
