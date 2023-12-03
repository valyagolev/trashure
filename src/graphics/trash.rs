use bevy::{
    diagnostic::{DiagnosticsStore, FrameTimeDiagnosticsPlugin},
    prelude::*,
};
use bevy_debug_text_overlay::screen_print;
use bevy_inspector_egui::egui::Key;
use rand::{seq::IteratorRandom, Rng};

use crate::{conf::Configuration, graphics::positions::GridPositioned};

use super::{
    animated::MovingToPosition,
    atlases::{AtlasesPluginState, Emojis},
};

pub struct TrashExperimentPlugin;
impl Plugin for TrashExperimentPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AtlasesPluginState::Finished), setup)
            .add_systems(Update, rewrite_layers)
            .add_systems(Update, handle_debug_keyboard)
            .add_systems(Update, fps_text_update_system);
    }
}

#[derive(Debug, Component, Reflect)]
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
    emojis: Option<Res<Emojis>>,
    conf: Res<Configuration>,
) {
    // warn!("trash::rewrite_layers");
    for (pile_id, mut pile) in q_pile.iter_mut() {
        if !pile.to_drop.is_empty() {
            commands.entity(pile_id).remove_children(&pile.to_drop);
            for ent in &pile.to_drop {
                commands.entity(*ent).despawn();
            }
        }

        for (pos, (emoji, ent)) in pile.iter_with_coords() {
            // println!("{} {} {}", x, y, emoji);

            // tighter
            let pos = pos * 0.5;
            // let pos = pos * 0.9;

            let mut sbundle = emojis
                .as_ref()
                .expect("emojis didn't load")
                .sbundle(emoji)
                .expect("couldn't find emoji?");

            if pos.x < 0.0 {
                sbundle.sprite.color =
                    Color::rgb(conf.shadow_tint, conf.shadow_tint, conf.shadow_tint);
            };

            if let Some(ent) = ent {
                commands
                    .entity(*ent)
                    .insert(MovingToPosition {
                        target: pos,
                        speed: 30.0,
                    })
                    .insert(sbundle.sprite);
            } else {
                sbundle.transform = Transform::from_translation(pos);

                let e = commands.spawn(sbundle).id();

                commands.entity(pile_id).push_children(&[e]);

                *ent = Some(e);
            }
        }
    }
}

fn fps_text_update_system(diagnostics: Res<DiagnosticsStore>, q_piles: Query<&Pile>) {
    let entities_cnt = q_piles
        .iter()
        .map(|pile| pile.layers.iter().map(|row| row.len()).sum::<usize>())
        .sum::<usize>();

    screen_print!("Entities: {}", entities_cnt);
    if let Some(fps) = diagnostics.get(FrameTimeDiagnosticsPlugin::FPS) {
        screen_print!("FPS: {:.2}", fps.value().unwrap_or(0.0));
    }
}

fn handle_debug_keyboard(
    keys: Res<Input<KeyCode>>,
    mut q_pile: Query<&mut Pile>,
    emojis: Option<Res<Emojis>>,
) {
    if keys.pressed(KeyCode::A) {
        let emojis = emojis.unwrap();
        let mut pile = q_pile.iter_mut().choose(&mut rand::thread_rng()).unwrap();

        pile.add_top(emojis.random_emoji());
    }
}

fn setup(mut commands: Commands, emojis: Res<Emojis>) {
    warn!("trash::setup");
    commands.spawn((
        Pile {
            layers: vec![],
            to_drop: vec![],
        },
        SpatialBundle {
            visibility: Visibility::Visible,
            ..default()
        },
        GridPositioned(IVec2::new(0, 0)),
    ));
    commands.spawn((
        Pile {
            layers: vec![],
            to_drop: vec![],
        },
        SpatialBundle {
            visibility: Visibility::Visible,
            ..default()
        },
        GridPositioned(IVec2::new(0, 1)),
    ));
    commands.spawn((
        Pile {
            layers: vec![],
            to_drop: vec![],
        },
        SpatialBundle {
            visibility: Visibility::Visible,
            ..default()
        },
        GridPositioned(IVec2::new(1, 1)),
    ));
}
