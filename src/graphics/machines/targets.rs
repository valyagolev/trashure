use bevy::prelude::*;
pub struct TargetsPlugin;

impl Plugin for TargetsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, Self::setup).add_systems(
            Update,
            (
                Self::make_targets,
                // Self::update_visibility,
                Self::update_location,
                // Self::handle_move,
                // Self::debug,
            ),
        );
    }
}

#[derive(Resource)]
struct TargetResources {
    scene: Handle<Scene>,
}

#[derive(Component)]
pub struct Target {
    global_pos: IVec2,
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
pub struct TargetInst;

impl TargetsPlugin {
    // fn debug(
    //     q_ihv: Query<(&InheritedVisibility, &Parent)>,
    //     q_noihv: Query<Entity, (Without<InheritedVisibility>, With<Children>)>,
    //     // reg: Res<AppTypeRegistry>,
    //     world: &World,
    // ) {
    //     for (ihv, p) in q_ihv.iter() {
    //         if let Ok(noihv) = q_noihv.get(p.get()) {
    //             println!("{:#?}", world.inspect_entity(noihv));
    //         }
    //     }
    // }

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
                        TargetInst,
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
}
