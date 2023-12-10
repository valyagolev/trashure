use bevy::{
    pbr::NotShadowCaster,
    prelude::*,
    render::render_resource::{AsBindGroup, ShaderRef},
    scene::SceneInstanceReady,
};

use super::Radar;

pub struct RadarGraphicsPlugin;

impl Plugin for RadarGraphicsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(MaterialPlugin::<RadarMaterial>::default())
            .add_systems(Startup, setup)
            .add_systems(
                Update,
                (setup_radars_graphics, redraw_radars, update_radar_material),
            );
    }
}

#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
pub struct RadarMaterial {
    #[uniform(1, visibility(fragment))]
    color: Color,
    // #[texture(2, visibility(fragment))]
    // color_texture: Option<Handle<Image>>,
    // #[sampler(3, visibility(fragment))]
    // alpha_mode: AlphaMode,
}

impl Material for RadarMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/radar.wgsl".into()
    }

    fn vertex_shader() -> ShaderRef {
        "shaders/radar.wgsl".into()
    }

    fn alpha_mode(&self) -> AlphaMode {
        AlphaMode::Blend
    }
}

#[derive(Resource)]
pub struct RadarResources {
    pub material: Handle<RadarMaterial>,
    // pub mesh: Handle<Mesh>,
    pub round_scene: Handle<Scene>,
    pub sector_scene: Handle<Scene>,
}

fn setup(
    mut commands: Commands,
    mut materials: ResMut<Assets<RadarMaterial>>,
    // mut meshes: ResMut<Assets<Mesh>>,
    ass: Res<AssetServer>,
    // spawner: Res<SceneSpawner>,
) {
    // let scene = ;
    commands.insert_resource(RadarResources {
        material: materials.add(
            RadarMaterial {
                color: Color::rgba(0.7, 0.3, 0.3, 0.9),
                // color_texture: None,
                // alpha_mode: AlphaMode::Blend,
            }
            .into(),
        ),
        // mesh: ,
        round_scene: ass.load("objects/radar_round.glb#Scene0"),
        sector_scene: ass.load("objects/radar_sector.glb#Scene0"),
    });
}

#[derive(Component)]
pub struct RadarScene;

fn setup_radars_graphics(
    mut commands: Commands,
    mut q_radars: Query<(Entity, &mut Radar), Added<Radar>>,
    rres: Res<RadarResources>,
) {
    for (e, mut r) in q_radars.iter_mut() {
        let radar_e = commands
            .spawn((
                RadarScene,
                NotShadowCaster,
                SceneBundle {
                    // mesh: rres.mesh.clone(),
                    // material: rres.material.clone(),
                    scene: if r.direction.is_some() {
                        rres.sector_scene.clone()
                    } else {
                        rres.round_scene.clone()
                    },
                    //     transform: Transform::from_rotation(Quat::from_rotation_y(-PI / 4.0)),
                    ..default()
                },
            ))
            .id();
        r.scene = Some(radar_e);
        commands.entity(e).add_child(radar_e);
    }
}

fn update_radar_material(
    mut commands: Commands,
    mut er: EventReader<SceneInstanceReady>,
    q_scenes: Query<&RadarScene>,
    q_material_uses: Query<Entity, With<Handle<StandardMaterial>>>,
    q_descendants: Query<&Children>,
    rres: Res<RadarResources>,
) {
    for ev in er.read() {
        // println!("radar scene ready: {:?}", ev.parent);
        if let Ok(_) = q_scenes.get(ev.parent) {
            let all_descendants = q_descendants.iter_descendants(ev.parent);

            let material_uses = q_material_uses.iter_many(all_descendants);

            for m in material_uses {
                commands.entity(m).remove::<Handle<StandardMaterial>>();

                commands
                    .entity(m)
                    .insert((NotShadowCaster, rres.material.clone()));
            }
        }
    }
}

fn redraw_radars(q_radars: Query<&Radar>, mut q_scenes: Query<&mut Transform, With<RadarScene>>) {
    for r in q_radars.iter() {
        let Some(mut t) = r.scene.and_then(|e| q_scenes.get_mut(e).ok()) else {
            continue;
        };

        let dist = r.dist();

        t.scale = Vec3::splat(
            dist, //  * 2.0
        );
    }
}
