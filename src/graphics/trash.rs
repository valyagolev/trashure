use bevy::prelude::*;
use bevy_inspector_egui::egui::Key;
use rand::{seq::IteratorRandom, Rng};

use crate::conf::Configuration;

use super::atlases::{AtlasesPluginState, Emojis};

pub struct TrashExperimentPlugin;
impl Plugin for TrashExperimentPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AtlasesPluginState::Finished), setup)
            .add_systems(Update, rewrite_layers)
            .add_systems(Update, handle_debug_keyboard);
    }
}

#[derive(Debug, Component)]
struct Pile {
    layers: Vec<Vec<(String, Option<Entity>)>>,
    to_drop: Vec<Entity>,
}

impl Pile {
    pub fn iter_with_coords(
        &mut self,
    ) -> impl Iterator<Item = (Vec3, &mut (String, Option<Entity>))> {
        self.layers.iter_mut().enumerate().flat_map(|(row_n, row)| {
            let row_size = row.len() as f32;
            let row_mid = (row_size - 1.0) / 2.0;

            row.iter_mut().enumerate().map(move |(col_n, s)| {
                let x = (col_n as f32 - row_mid) * 72.0;
                let y = row_n as f32 * 72.0 * 0.9 + (row_mid - col_n as f32).abs() * 0.3 * 72.0;

                // print!("row_size: {row_size}, row_mid: {row_mid}, col_n: {col_n}, x: {x}, y: {y} ");

                (Vec3::new(x, y, -(row_n as f32)), s)
            })
        })
    }

    pub fn add_top(&mut self, emoji: &str) {
        self.layers.push(vec![(emoji.to_string(), None)]);
        self.rebalance();
    }

    pub fn rebalance(&mut self) {
        let rand = &mut rand::thread_rng();

        for i in (1..self.layers.len()).rev() {
            while self.layers[i].len() >= self.layers[i - 1].len() {
                let itomove = (0..self.layers[i].len()).choose(rand).unwrap();

                let el = self.layers[i].remove(itomove);

                let ix = itomove + rand.gen_range(0..=1);

                self.layers[i - 1].insert(ix, el);
            }

            if self.layers[i].is_empty() {
                self.layers.remove(i);
            }
        }
    }
}

fn rewrite_layers(
    mut commands: Commands,
    mut q_pile: Query<(Entity, &mut Pile), Changed<Pile>>,
    // q_sprites: Query<(&Transform, &TextureAtlasSprite)>,
    emojis: Option<Res<Emojis>>,
) {
    for (pile_id, mut pile) in q_pile.iter_mut() {
        if !pile.to_drop.is_empty() {
            commands.entity(pile_id).remove_children(&pile.to_drop);
            for ent in &pile.to_drop {
                commands.entity(*ent).despawn();
            }
        }
        println!("pile {:?}", pile);

        for (pos, (emoji, ent)) in pile.iter_with_coords() {
            // println!("{} {} {}", x, y, emoji);
            // tighter
            let pos = pos * 0.5;
            // let pos = pos * 0.9;
            if let Some(ent) = ent {
                // let (t, _) = q_sprites.get(*ent).unwrap();
                commands
                    .entity(*ent)
                    .insert(Transform::from_translation(pos));
            } else {
                let e =
                    commands
                        .spawn(SpriteSheetBundle {
                            transform: Transform::from_translation(pos),
                            ..emojis.as_ref().unwrap().sbundle(emoji).unwrap()
                        })
                        .id();
                commands.entity(pile_id).push_children(&[e]);

                *ent = Some(e);
            }
        }
    }
}

fn handle_debug_keyboard(
    keys: Res<Input<KeyCode>>,
    mut q_pile: Query<&mut Pile>,
    emojis: Option<Res<Emojis>>,
) {
    if keys.just_released(KeyCode::A) {
        println!("A");
        let emojis = emojis.unwrap();
        let mut pile = q_pile.single_mut();

        pile.add_top(emojis.random_emoji());
    }
}

fn setup(mut commands: Commands, emojis: Res<Emojis>) {
    commands.spawn((
        Pile {
            layers: vec![
                vec![
                    ("🎑".to_string(), None),
                    ("🎐".to_string(), None),
                    ("🎉".to_string(), None),
                    ("🎑".to_string(), None),
                    ("🎐".to_string(), None),
                    ("🎉".to_string(), None),
                ],
                vec![
                    ("🎑".to_string(), None),
                    ("🎐".to_string(), None),
                    ("🎉".to_string(), None),
                    ("🎑".to_string(), None),
                    ("🎐".to_string(), None),
                ],
            ],
            to_drop: vec![],
        },
        SpatialBundle {
            transform: Transform {
                translation: Vec3::new(0.0, 0.0, 0.0),
                // scale: Vec3::splat(1.0),
                ..default()
            },
            visibility: Visibility::Visible,
            ..default()
        },
    ));

    // commands.spawn(SpriteSheetBundle {
    //     transform: Transform {
    //         translation: Vec3::new(150.0, 0.0, 0.0),
    //         scale: Vec3::splat(1.0),
    //         ..default()
    //     },
    //     ..emojis.sbundle("🎁").unwrap()
    // });

    // commands.spawn(SpriteSheetBundle {
    //     transform: Transform {
    //         translation: Vec3::new(200.0, 0.0, 0.0),
    //         scale: Vec3::splat(1.0),
    //         ..default()
    //     },
    //     ..emojis.sbundle("🎑").unwrap()
    // });
}
