use bevy::{prelude::*, scene::SceneInstance, utils::HashMap};
use itertools::Itertools;

pub struct RecolorPlugin;

impl Plugin for RecolorPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (ensure_stores, update_colors.after(ensure_stores)))
            .register_type::<Tinted>()
            .insert_resource(TintedMaterials::default());
    }
}

#[derive(Debug, Reflect, Component)]
pub struct Tinted {
    pub color: Option<Color>,
    pub emissive: Option<Color>,
    pub alpha_mode: Option<AlphaMode>,
}

#[derive(Debug, Component)]
struct TintedStore {
    last_applied: (Option<Color>, Option<Color>, Option<AlphaMode>),
    origs: HashMap<Entity, Handle<StandardMaterial>>,
}

impl Tinted {
    pub fn empty() -> Self {
        Tinted {
            color: None,
            emissive: None,
            alpha_mode: None,
        }
    }
    pub fn new(color: Color) -> Self {
        Tinted {
            color: Some(color),
            emissive: None,
            alpha_mode: None,
        }
    }

    pub fn new_emissive(color: Color, emissive: Color) -> Self {
        Tinted {
            color: Some(color),
            emissive: Some(emissive),
            alpha_mode: None,
        }
    }

    pub fn alpha(self) -> Self {
        Tinted {
            alpha_mode: Some(AlphaMode::Blend),
            ..self
        }
    }
}

#[derive(Resource, Default)]
struct TintedMaterials(HashMap<(AssetId<StandardMaterial>, [u8; 4]), Handle<StandardMaterial>>);

fn ensure_stores(mut cmds: Commands, mut q_scenes: Query<(Entity, &Tinted), Without<TintedStore>>) {
    for (e, tnted) in q_scenes.iter_mut() {
        cmds.entity(e).insert(TintedStore {
            last_applied: (None, None, None),
            origs: HashMap::default(),
        });
    }
}

fn update_colors(
    mut cmds: Commands,
    mut q_scenes: Query<(Entity, &SceneInstance, &mut Tinted, &mut TintedStore)>,
    q_material_uses: Query<(Entity, &Handle<StandardMaterial>)>,
    scene_manager: Res<SceneSpawner>,
    mut pbr_materials: ResMut<Assets<StandardMaterial>>,
    mut custom_materials: ResMut<TintedMaterials>,
) {
    for (e, instance, mut tnted, mut tnted_store) in q_scenes.iter_mut() {
        if tnted_store.last_applied == (tnted.color, tnted.emissive, tnted.alpha_mode) {
            continue;
        }

        if scene_manager.instance_is_ready(**instance) {
            println!("Recoloring");

            let material_uses =
                q_material_uses.iter_many(scene_manager.iter_instance_entities(**instance));

            for (entity, material_handle) in material_uses {
                if tnted_store.origs.get(&entity).is_none() {
                    tnted_store.origs.insert(entity, material_handle.clone());
                }

                let orig_material = tnted_store.origs.get(&entity).unwrap();

                let id = orig_material.id();

                let mut new_material;

                if let Some(color) = tnted.color {
                    if let Some(custom) = custom_materials.0.get(&(id, color.as_rgba_u8())) {
                        println!("Using existing recolor");
                        // cmds.entity(entity).insert(custom.clone());
                        new_material = custom.clone();
                    }

                    let Some(material) = pbr_materials.get(orig_material) else {
                        continue;
                    };

                    println!("New recolor");

                    let mut new_material_m = material.clone();
                    // new_material_m.base_color += color;

                    new_material_m.base_color = Color::from(
                        Vec4::from(color).lerp(Vec4::from(new_material_m.base_color), 0.5),
                    );
                    // new_material_m.base_color.set_a(color.a());
                    new_material_m.specular_transmission = color.a();
                    new_material_m.diffuse_transmission = color.a();

                    if let Some(em) = tnted.emissive {
                        new_material_m.emissive = em;
                    }

                    if let Some(am) = tnted.alpha_mode {
                        new_material_m.alpha_mode = am;
                    }

                    new_material = pbr_materials.add(new_material_m);

                    custom_materials
                        .0
                        .insert((id, color.as_rgba_u8()), new_material.clone());
                } else {
                    new_material = tnted_store.origs.get(&entity).unwrap().clone();
                }

                cmds.entity(entity).insert(new_material);
            }

            tnted_store.last_applied = (tnted.color, tnted.emissive, tnted.alpha_mode);
        }
    }
}

// impl RecoloredScenes {
//     pub fn new(ass: Res<AssetServer>, fname: &'static str) -> Self {
//         RecoloredScenes {
//             scenes: (0..RECOLORS_COUNT)
//                 .map(|_| ass.load(fname))
//                 .collect_vec()
//                 .try_into()
//                 .unwrap(),
//         }
//     }
// }

// #[derive(Component)]
// pub struct MachineTypeRecolors;

// pub fn recolor(
//     mut commands: Commands,
//     ass: Res<AssetServer>,
//     // mut meshes: ResMut<Assets<Mesh>>,
//     // mut materials: ResMut<Assets<StandardMaterial>>,
//     mut scenes: ResMut<Assets<Scene>>,
//     q_machines: Query<(Entity, &MachineType), Without<MachineTypeRecolors>>,
// ) {
//     for (me, mt) in q_machines.iter() {
//         if let Some(scene) = scenes.get(&mt.scenes.scenes[0]) {
//             println!("Recoloring {}", mt.name);

//             commands.entity(me).insert(MachineTypeRecolors);

//             let scene2 = scenes.get(&mt.scenes.scenes[1]).unwrap();

//             println!("{:?} {:?}", scene.world.id(), scene2.world.id());
//         }
//     }
// }
