use bevy::{
    diagnostic::{DiagnosticsStore, FrameTimeDiagnosticsPlugin},
    prelude::*,
    utils::{HashMap, HashSet},
};
use bevy_debug_text_overlay::screen_print;
use itertools::Itertools;
use rand::{seq::SliceRandom, Rng};

use crate::conf::Configuration;

use super::{
    animated::MovingToPosition,
    voxels3d::VoxelResources,
    // positions::IntegerPositioned,
};

pub struct PiecesPlugin;

const SPAWN_CLOSER_THAN: f32 = 50.0;
const DESPAWN_FARTHER_THAN: f32 = 55.0;

impl Plugin for PiecesPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            // OnEnter(AtlasesPluginState::Finished),
            Startup, setup,
        )
        .add_systems(
            Update,
            spawn_sprites.after(disambiguate_entities), // .run_if(in_state(AtlasesPluginState::Finished)),
        )
        .add_systems(Update, handle_debug_keyboard)
        .insert_resource(EntitiesPos::default())
        .add_systems(Update, disambiguate_entities)
        .add_systems(Update, show_stats)
        // .add_systems(Update, shade_voxels.after(disambiguate_entities))
        .register_type::<Voxel>();
    }
}

#[derive(Debug, Reflect, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Material {
    Reddish,
    Greenish,
    Blueish,
    Brownish,
}

impl Material {
    pub fn random(rng: &mut impl Rng) -> Self {
        match rng.gen_range(0..25) {
            0 => Self::Reddish,
            1..=3 => Self::Greenish,
            4..=8 => Self::Blueish,
            _ => Self::Brownish,
        }
    }
}

#[derive(Debug, Component, Reflect)]
pub struct Voxel {
    pub material: Material,
    // shade: bool,
    pub tight_at: Option<IVec3>,
}

pub fn setup(
    mut commands: Commands,
    // emojis: Res<Emojis>,
    conf: Res<Configuration>,
) {
    let rand = &mut rand::thread_rng();

    // for i in 0..3 {
    //     for j in 0..3 {
    //         // for z in 0..3 {
    //         let p = IVec3::new(i, j, 0);
    //         commands.spawn((
    //             Voxel {
    //                 material: Material::random(rand),
    //                 // shade: false,
    //                 tight_at: None,
    //             },
    //             Transform::from_translation(p.into()),
    //             MovingToPosition::new(p, 40.0),
    //         ));
    //         // }
    //     }
    // }

    for row in 0..6 {
        let half_radius = 6 - row;

        for i in -half_radius..half_radius {
            for j in -half_radius..half_radius {
                let pos = IVec3::new(i, row, j);

                commands.spawn((
                    Voxel {
                        material: Material::random(rand),
                        // shade: false,
                        tight_at: None,
                    },
                    Transform::from_translation(pos.as_vec3()),
                    MovingToPosition::new(pos, 40.0),
                ));
            }
        }
    }

    // for mut pos in [
    //     IVec3::new(0, 0, 0),
    //     IVec3::new(1, 0, 0),
    //     IVec3::new(0, 1, 0),
    //     IVec3::new(1, 1, 0),
    //     IVec3::new(-1, 0, 0),
    //     IVec3::new(0, -1, 0),
    //     IVec3::new(-1, -1, 0),
    //     IVec3::new(-1, 1, 0),
    //     IVec3::new(1, -1, 0),
    //     IVec3::new(0, -2, 0),
    // ] {
    //     commands.spawn((
    //         Voxel {
    //             // emoji: "⚽️".to_owned(), // emojis.random_emoji().to_owned(),
    //             // shade: false,
    //             material: Material::random(rand),
    //             tight_at: None,
    //         },
    //         // Transform::from_translation(transform_to_voxel_grid(&conf, pos)),
    //         Transform::from_translation(pos.into()),
    //         MovingToPosition::new(pos, 40000.0),
    //     ));

    //     pos.z += 1;

    //     commands.spawn((
    //         Voxel {
    //             emoji: "⚽️".to_owned(), // emojis.random_emoji().to_owned(),
    //             shade: false,
    //             tight_at: None,
    //         },
    //         Transform::from_translation(transform_to_voxel_grid(&conf, pos)),
    //         MovingToPosition::new(pos, 40000.0),
    //     ));

    //     pos.z += 1;

    //     commands.spawn((
    //         Voxel {
    //             emoji: "⚽️".to_owned(), // emojis.random_emoji().to_owned(),
    //             shade: false,
    //             tight_at: None,
    //         },
    //         Transform::from_translation(transform_to_voxel_grid(&conf, pos)),
    //         MovingToPosition::new(pos, 40000.0),
    //     ));
    // }
}

pub fn spawn_sprites(
    mut commands: Commands,
    mut q_no_sprite: Query<(Entity, &Voxel, &Transform, Option<&mut Visibility>)>,
    // mut q_no_sprite: Query<(Entity, &Voxel, &Transform, Option<&Visibility>)>,
    // emojis: Res<Emojis>,
    q_camera: Query<&GlobalTransform, With<Camera3d>>,
    voxel_resources: Option<Res<VoxelResources>>,
) {
    let Some(voxel_resources) = voxel_resources else {
        return;
    };

    let camera = q_camera.single();

    let mut total = 0;
    let mut visible = 0;
    let mut spawned = 0;

    for (entity, voxel, transform, mbvis) in q_no_sprite.iter_mut() {
        // let mut sbundle = emojis.sbundle(&voxel.emoji).expect("couldn't find emoji?");

        // sbundle.transform = transform.clone();

        total += 1;

        if mbvis.is_none() {
            // let child =
            //     commands.spawn(voxel_resources.pbr_bundle(voxel.material, transform.translation));
            if transform.translation.distance(camera.translation()) < SPAWN_CLOSER_THAN {
                commands
                    .entity(entity)
                    .insert(voxel_resources.pbr_bundle(voxel.material, transform.translation));

                spawned += 1;
            }
        } else {
            let mut mbvis = mbvis.unwrap();
            if transform.translation.distance(camera.translation()) > DESPAWN_FARTHER_THAN {
                *mbvis = Visibility::Hidden;

                // commands.entity(entity).remove::<PbrBundle>();
            } else {
                visible += 1;
                *mbvis = Visibility::Visible;
            }
        }
    }
    screen_print!(
        "total: {}, visible: {}, spawned: {}",
        total,
        visible,
        spawned
    );
}

pub fn show_stats(diagnostics: Res<DiagnosticsStore>) {
    if let Some(rps) = diagnostics
        .get(FrameTimeDiagnosticsPlugin::FPS)
        .and_then(|d| d.average())
    {
        screen_print!("FPS: {}", rps);
    }
}

fn handle_debug_keyboard(
    keys: Res<Input<KeyCode>>,
    mut commands: Commands,
    // emojis: Option<Res<Emojis>>,
    conf: Res<Configuration>,
) {
    // let Some(emojis) = emojis else {
    //     return;
    // };

    if keys.pressed(KeyCode::A) {
        let rnd = &mut rand::thread_rng();
        // let x = rnd.gen_range(-10..10);
        // let y = rnd.gen_range(-10..10);
        // let pos = IVec3::new(x, y, 0);

        let pos = [
            IVec3::new(0, 0, 0),
            IVec3::new(10, 0, 0),
            IVec3::new(0, 0, 15),
        ]
        .choose(rnd)
        .unwrap()
        .clone();

        commands.spawn((
            Voxel {
                // emoji: "⚽️".to_owned(), // emojis.random_emoji().to_owned(),
                // shade: false,
                material: Material::random(rnd),
                tight_at: None,
            },
            Transform::from_translation(pos.as_vec3()), //transform_to_voxel_grid(&conf, pos)),
            MovingToPosition::new(pos, 40000.0),
        ));
    }
}

#[derive(Reflect, Resource, Default)]
struct EntitiesPos(HashMap<IVec3, Entity>);

static PUSH_DIRECTIONS: &[IVec3] = &[
    IVec3::new(1, 0, 0),
    IVec3::new(-1, 0, 0),
    IVec3::new(0, 0, 1),
    IVec3::new(0, 0, -1),
    IVec3::new(0, 1, 0),
    // &IVec3::new(0, 0, -1),
];

static DROP_DIRECTIONS: &[IVec3] = &[
    IVec3::new(1, 0, 0),
    IVec3::new(-1, 0, 0),
    IVec3::new(0, 0, 1),
    IVec3::new(0, 0, -1),
    // &IVec3::new(0, 0, -1),
];

fn find_available_cell(
    entities: &HashMap<IVec3, Vec<Entity>>,
    placed: &HashMap<IVec3, Entity>,
    pos: IVec3,
) -> IVec3 {
    let rng = &mut rand::thread_rng();

    let mut directions = (*PUSH_DIRECTIONS).iter().collect_vec();
    directions.shuffle(rng);

    let available = directions.iter().find(|d| {
        let near = pos + ***d;

        !entities.contains_key(&near) && !placed.contains_key(&near)
    });

    if let Some(available) = available {
        return pos + **available;
    }

    find_available_cell(entities, placed, pos + IVec3::new(0, 1, 0))
}

fn disambiguate_entities(
    mut entities_pos: ResMut<EntitiesPos>,
    mut q_voxels: Query<(Entity, &mut MovingToPosition, &mut Voxel)>,
    q_camera: Query<&GlobalTransform, With<Camera3d>>,
) {
    // return;
    entities_pos.0.clear();

    let camera = q_camera.single();

    let mut entities = HashMap::<IVec3, Vec<_>>::new();

    {
        let my_span = info_span!("disambiguate_entities::gather_vecs").entered();

        for (entity, pos, _) in q_voxels.iter() {
            if camera.translation().distance(pos.target.as_vec3()) < DESPAWN_FARTHER_THAN {
                entities.entry(pos.target).or_default().push(entity);
            }
        }
    }

    let rng = &mut rand::thread_rng();

    let mut changes = Vec::new();

    {
        let my_span = info_span!("disambiguate_entities::check_dupes").entered();

        for (pos, cell_entities) in &entities {
            if cell_entities.len() > 1 {
                let mut cell_entities = cell_entities.clone();

                while cell_entities.len() > 1 {
                    let next_cell = find_available_cell(&entities, &entities_pos.0, *pos);
                    let entity_i = rng.gen_range(0..cell_entities.len());

                    let entity = cell_entities.remove(entity_i);

                    entities_pos.0.insert(next_cell, entity);
                    changes.push((entity, next_cell));
                }
                entities_pos.0.insert(*pos, cell_entities[0]);
            } else {
                entities_pos.0.insert(*pos, cell_entities[0]);
            }
        }
    }

    // now we're dropping some
    let mut to_drop = vec![];
    let mut taken = HashSet::new();
    {
        let my_span = info_span!("disambiguate_entities::gravity").entered();

        for (k, entity) in &entities_pos.0 {
            // continue;
            if k.y <= 0 {
                continue;
            }

            let (_, _, mut voxel) = q_voxels.get_mut(*entity).unwrap();

            let below = *k + IVec3::new(0, -1, 0);

            if !entities_pos.0.contains_key(&below) && !taken.contains(&below) {
                to_drop.push((*k, below, *entity));
                taken.insert(below);
                continue;
            }

            if voxel.tight_at == Some(*k) {
                continue;
            }

            let empties_below = DROP_DIRECTIONS
                .iter()
                .map(|d| below + *d)
                .filter(|p| !entities_pos.0.contains_key(p) && !taken.contains(p))
                .collect_vec();

            // either nothing is below; or we're not tight then there's a probability of drop

            if empties_below.len() == 0 || {
                let mut rng = rand::thread_rng();
                rng.gen_range(0..4) > empties_below.len()
            } {
                voxel.tight_at = Some(*k);
                continue;
            }

            let pos = empties_below.choose(rng).unwrap();

            to_drop.push((*k, *pos, *entity));
            taken.insert(*pos);
        }
    }

    {
        let my_span = info_span!("disambiguate_entities::apply_changes").entered();
        for (entity, cell) in changes {
            let (_, mut pos, _) = q_voxels.get_mut(entity).unwrap();
            pos.target = cell;
        }
    }
    {
        let my_span = info_span!("disambiguate_entities::apply_drops").entered();
        for (from, to, entity) in to_drop {
            let (_, mut pos, _) = q_voxels.get_mut(entity).unwrap();
            pos.target = to;
            entities_pos.0.insert(to, entity);
            entities_pos.0.remove(&from);
        }
    }
}

// fn shade_voxels(
//     entities_pos: Res<EntitiesPos>,
//     mut q_voxels: Query<(
//         &mut Voxel,
//         &MovingToPosition,
//         &mut TextureAtlasSprite,
//         &mut Visibility,
//     )>,
//     conf: Res<Configuration>,
// ) {
//     return;
//     for (mut voxel, pos, mut spr, mut vis) in q_voxels.iter_mut() {
//         voxel.shade = entities_pos
//             .0
//             .contains_key(&(pos.target - IVec3::new(0, 1, 0)))
//             && entities_pos
//                 .0
//                 .contains_key(&(pos.target - IVec3::new(0, 2, 0)))
//             && entities_pos
//                 .0
//                 .contains_key(&(pos.target + IVec3::new(0, 0, 1)))
//             && entities_pos
//                 .0
//                 .contains_key(&(pos.target + IVec3::new(0, 0, 2)));

//         // println!("voxel.shade: {}", voxel.shade);

//         // spr.color = if voxel.shade {
//         //     Color::rgb(conf.shadow_tint, conf.shadow_tint, conf.shadow_tint)
//         //     // Color::RED
//         // } else {
//         //     // Color::WHITE
//         //     Color::rgb(0.9, 0.9, 0.9)
//         // };

//         // if voxel.shade {
//         //     *vis = Visibility::Hidden;
//         // } else {
//         //     *vis = Visibility::Visible;
//         // }

//         // spr.color += Color::rgba(0.0, 0.0, (0.05 * (pos.target.z as f32)).min(0.5), 0.0);
//     }
// }
