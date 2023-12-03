use std::path::PathBuf;

use bevy::{asset::LoadedFolder, prelude::*, utils::HashMap};
use bevy_common_assets::json::{self, JsonAssetPlugin};
use itertools::Itertools;
use rand::seq::IteratorRandom;
use serde::{Deserialize, Serialize};

pub struct AtlasesPlugin;

impl Plugin for AtlasesPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((JsonAssetPlugin::<Atlas>::new(&["atlas.json"]),))
            // insert_resource(Configuration::default())
            //     // .insert_resource(Persistent::<Configuration>::n())
            // .add_systems(Startup, load_textures);
            .add_state::<AtlasesPluginState>()
            .add_systems(OnEnter(AtlasesPluginState::Setup), load_textures)
            .add_systems(
                Update,
                check_textures.run_if(in_state(AtlasesPluginState::Setup)),
            )
            .add_systems(OnEnter(AtlasesPluginState::Loaded), setup);
        // .add_systems(Update, on_modify_configuration);
        //     .register_type::<Configuration>() // you need to register your type to display it
        //     .add_plugins(ResourceInspectorPlugin::<Configuration>::default());
    }
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash, States)]
pub enum AtlasesPluginState {
    #[default]
    Setup,
    Loaded,
    Finished,
}

#[derive(Resource, Default)]
struct AtlasesFolder(Handle<LoadedFolder>);

fn check_textures(
    mut next_state: ResMut<NextState<AtlasesPluginState>>,
    rpg_sprite_folder: ResMut<AtlasesFolder>,
    mut events: EventReader<AssetEvent<LoadedFolder>>,
) {
    // Advance the `AtlasesPluginState` once all sprite handles have been loaded by the `AssetServer`
    for event in events.read() {
        if event.is_loaded_with_dependencies(&rpg_sprite_folder.0) {
            next_state.set(AtlasesPluginState::Loaded);
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
struct Emoji {
    emoji: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct ImagePos {
    pub emoji: Emoji,
    pub top: usize,
    pub left: usize,
    pub index: usize,
}

#[derive(Serialize, Deserialize, Debug, bevy::asset::Asset, bevy::reflect::TypePath)]
struct Atlas {
    pub emojis: Vec<ImagePos>,
}

#[derive(Resource)]
pub struct Emojis {
    pub atlases: HashMap<PathBuf, Handle<TextureAtlas>>,
    pub emoji_positions: HashMap<String, (PathBuf, usize)>,
}

impl Emojis {
    fn from(
        atlases: HashMap<PathBuf, Handle<TextureAtlas>>,
        map: &HashMap<PathBuf, &Atlas>,
    ) -> Self {
        Self {
            atlases,
            emoji_positions: map
                .iter()
                .flat_map(|(&ref k, v)| {
                    v.emojis
                        .iter()
                        .map(|x| (x.emoji.emoji.clone(), (k.clone(), x.index)))
                })
                .collect(),
        }
    }

    pub fn lookup(&self, emoji: &str) -> Option<(&PathBuf, usize)> {
        self.emoji_positions.get(emoji).map(|(k, v)| (k, *v))
    }

    pub fn sprite(&self, emoji: &str) -> Option<(&PathBuf, TextureAtlasSprite)> {
        let (k, index) = self.lookup(emoji)?;

        let atlas = self.atlases.get(k).unwrap();

        Some((k, TextureAtlasSprite::new(index)))
    }

    pub fn sbundle(
        &self,
        emoji: &str,
        // atlases: &Assets<TextureAtlas>,
    ) -> Option<SpriteSheetBundle> {
        let (k, sprite) = self.sprite(emoji)?;

        let atlas = self.atlases.get(k)?;

        Some(SpriteSheetBundle {
            sprite,
            texture_atlas: atlas.clone(),
            ..default()
        })
    }

    pub fn random_emoji(&self) -> &str {
        self.emoji_positions
            .keys()
            .choose(&mut rand::thread_rng())
            .unwrap()
    }
}

fn load_textures(mut commands: Commands, asset_server: Res<AssetServer>) {
    // load multiple, individual sprites from a folder
    commands.insert_resource(AtlasesFolder(asset_server.load_folder("atlases")));
}

fn setup(
    mut commands: Commands,
    atlases_handlers: Res<AtlasesFolder>,
    // asset_server: Res<AssetServer>,
    mut next_state: ResMut<NextState<AtlasesPluginState>>,
    loaded_folders: Res<Assets<LoadedFolder>>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    mut textures: ResMut<Assets<Image>>,
    atlas_descriptions: Res<Assets<Atlas>>,
) {
    // let mut texture_atlas_builder = TextureAtlasBuilder::default();
    let loaded_folder: &LoadedFolder = loaded_folders.get(&atlases_handlers.0).unwrap();

    let mut textures_handles = HashMap::new();
    let mut textures_positions = HashMap::new();

    for handle in loaded_folder.handles.iter() {
        let path = handle.path().unwrap();

        match &*path.get_full_extension().unwrap() {
            "atlas.json" => {
                let pure_path = path.path().with_extension("").with_extension("");

                let dataid = handle.id().typed_unchecked::<Atlas>();

                let data = atlas_descriptions.get(dataid).unwrap();

                textures_positions.insert(pure_path, data);
            }
            "png" => {
                let pure_path = path.path().with_extension("");

                let id = handle.id().typed_unchecked::<Image>();
                // let Some(texture) = textures.get(id) else {
                //     warn!(
                //         "{:?} did not resolve to an `Image` asset.",
                //         handle.path().unwrap()
                //     );
                //     continue;
                // };

                textures_handles.insert(pure_path, handle.clone().typed_unchecked::<Image>());
            }
            _ => {
                warn!("Unknown file: {path:?}");
            }
        }
    }

    let mut atlases = HashMap::new();

    for k in textures_handles.keys() {
        let json = textures_positions.get(k).unwrap();

        let column_cnt =
            (json.emojis.iter().max_by_key(|x| x.left).unwrap().left as usize / 72) + 1;

        let row_cnt = (json.emojis.iter().max_by_key(|x| x.top).unwrap().top as usize / 72) + 1;

        let atlas = bevy::prelude::TextureAtlas::from_grid(
            textures_handles[k].clone(),
            Vec2::new(72.0, 72.0),
            column_cnt,
            row_cnt,
            None,
            None,
        );
        atlases.insert(k.clone(), texture_atlases.add(atlas));
    }

    commands.insert_resource(Emojis::from(atlases, &textures_positions));

    next_state.set(AtlasesPluginState::Finished);

    // let (k, sprite) = lookup_atlases(&textures_positions, "🎁").unwrap();

    // commands.spawn(Camera2dBundle::default());

    // commands.spawn(SpriteSheetBundle {
    //     transform: Transform {
    //         translation: Vec3::new(150.0, 0.0, 0.0),
    //         scale: Vec3::splat(1.0),
    //         ..default()
    //     },
    //     sprite,
    //     texture_atlas: atlases[k].clone(),
    //     ..default()
    // });
    // draw the atlas itself
    // commands.spawn(SpriteBundle {
    //     texture: texture_atlas_texture,
    //     transform: Transform::from_xyz(-300.0, 0.0, 0.0),
    //     ..default()
    // });
}
