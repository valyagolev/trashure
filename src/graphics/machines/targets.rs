use bevy::{prelude::*, render::view::RenderLayers};
use bevy_mod_raycast::immediate::{Raycast, RaycastSettings, RaycastVisibility};

use crate::graphics::{
    cursor::CursorOver, recolor::Tinted, scenerenderlayer::SceneRenderLayers,
    selectable::CurrentlySelected,
};
pub struct TargetsPlugin;

impl Plugin for TargetsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, Self::setup)
            .add_systems(
                Update,
                (
                    Self::make_targets,
                    // Self::on_scene_load,
                    Self::update_visibility,
                    Self::handle_move_start,
                    Self::handle_move.after(Self::handle_move_start),
                    Self::update_location.after(Self::handle_move),
                ),
            )
            .insert_resource(TargetBeingMoved(None));
    }
}

#[derive(Resource)]
struct TargetResources {
    scene: Handle<Scene>,
}

#[derive(Resource, Deref, DerefMut)]
struct TargetBeingMoved(Option<(Entity, Vec2)>);

#[derive(Component)]
pub struct Target {
    pub global_pos: IVec2,
    instantiated: Option<Entity>,
}

impl Target {
    pub fn new(global_pos: IVec2) -> Self {
        Target {
            global_pos,
            instantiated: None,
        }
    }
}

#[derive(Component)]
pub struct TargetInst(Entity);

// #[derive(Component)]
// pub struct TargetSubMesh;

impl TargetsPlugin {
    fn setup(mut commands: Commands, ass: Res<AssetServer>) {
        commands.insert_resource(TargetResources {
            scene: ass.load("objects/target.glb#Scene0"),
        });
    }

    fn make_targets(
        mut commands: Commands,
        mut q_targets: Query<(Entity, &mut Target), Added<Target>>,
        tres: Res<TargetResources>,
    ) {
        for (e, mut t) in q_targets.iter_mut() {
            if t.instantiated.is_none() {
                println!("spawning");
                let spawned = commands
                    .spawn((
                        Name::new("target inst"),
                        Tinted::empty(),
                        TargetInst(e),
                        SceneRenderLayers(RenderLayers::layer(6)),
                        SceneBundle {
                            scene: tres.scene.clone(),
                            transform: Transform::from_translation(
                                t.global_pos.extend(0).xzy().as_vec3(),
                            ),
                            ..default()
                        },
                    ))
                    .id();

                t.instantiated = Some(spawned);

                // commands.entity(e).add_child(spawned);
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

    fn update_visibility(
        mut q_target_inst: Query<(&TargetInst, &mut Visibility)>,
        selected: Res<CurrentlySelected>,
    ) {
        for (t, mut v) in q_target_inst.iter_mut() {
            *v = if selected.0 == Some(t.0) {
                Visibility::Visible
            } else {
                Visibility::Hidden
            };
        }
    }

    fn update_location(
        q_targets: Query<&Target, Changed<Target>>,
        mut q_scenes: Query<&mut Transform, With<TargetInst>>,
    ) {
        for q_target in q_targets.iter() {
            for mut q_scene in q_scenes.iter_mut() {
                q_scene.translation = q_target.global_pos.extend(0).xzy().as_vec3();
            }
        }
    }

    fn handle_move_start(
        mut raycast: Raycast,
        mut q_targets: Query<
            (
                Entity,
                &GlobalTransform,
                &ViewVisibility,
                &TargetInst,
                &mut Tinted,
            ),
            // With<TargetInst>,
        >,
        mouse: Res<CursorOver>,
        parent_query: Query<&Parent>,
        mouse_inp: Res<Input<MouseButton>>,
        mut target_being_moved: ResMut<TargetBeingMoved>,
    ) {
        if target_being_moved.is_some() {
            if !mouse_inp.pressed(MouseButton::Left) {
                target_being_moved.0 = None;
            }
            return;
        }

        let valid_entities = q_targets
            .iter()
            .filter(|tpl| tpl.2.get())
            .map(|tpl| tpl.0)
            .collect::<Vec<_>>();

        if valid_entities.is_empty() {
            return;
        }
        let hit = raycast
            .cast_ray(
                mouse.ray.into(),
                &RaycastSettings {
                    visibility: RaycastVisibility::MustBeVisible,
                    filter: &|entity: Entity| {
                        parent_query
                            .iter_ancestors(entity)
                            .any(|anc| valid_entities.contains(&anc))
                    },
                    early_exit_test: &|_| true,
                },
            )
            .get(0);

        let hovered_inst = hit.and_then(|hit| {
            parent_query
                .iter_ancestors(hit.0)
                .find(|anc| valid_entities.contains(anc))
        });

        for t in valid_entities {
            let mut tpl = q_targets.get_mut(t).unwrap();

            if hovered_inst == Some(t) {
                *tpl.4 = Tinted::new(Color::rgb(0.3, 0.0, 0.1));
            } else {
                *tpl.4 = Tinted::empty();
            }
        }

        if mouse_inp.pressed(MouseButton::Left) {
            if let Some(hovered_inst) = hovered_inst {
                let tpl = q_targets.get(hovered_inst).unwrap();

                let delta = mouse.ground - tpl.1.translation().xz();

                target_being_moved.0 = Some((tpl.3 .0, delta));
            }
        }
    }

    fn handle_move(
        mut q_target_confs: Query<&mut Target>,
        mouse: Res<CursorOver>,
        mouse_inp: Res<Input<MouseButton>>,
        target_being_moved: ResMut<TargetBeingMoved>,
    ) {
        let Some((target, delta)) = target_being_moved.0 else {
            return;
        };

        if !mouse_inp.pressed(MouseButton::Left) {
            return;
        }

        let new_pos = (mouse.ground - delta).as_ivec2();
        let mut conf = q_target_confs.get_mut(target).unwrap();

        conf.global_pos = new_pos;
    }
}
