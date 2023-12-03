use std::path::PathBuf;

use bevy::{
    asset::{LoadState, LoadedFolder, UntypedAssetId},
    prelude::*,
    utils::HashMap,
};
use bevy_common_assets::json::{self, JsonAssetPlugin};
use bevy_inspector_egui::egui::TextBuffer;
use itertools::Itertools;
use rand::seq::IteratorRandom;
use serde::{Deserialize, Serialize};

pub struct AtlasesPlugin;

impl Plugin for AtlasesPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((JsonAssetPlugin::<Atlas>::new(&["atlas.json"]),))
            .insert_resource(AtlasesFolder::default())
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
struct AtlasesFolder(
    Vec<(&'static str, Handle<Image>)>,
    Vec<(&'static str, Handle<Atlas>)>,
);

fn load_textures(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut folder: ResMut<AtlasesFolder>,
) {
    warn!("load_textures");
    let imgs = [(
        "activities-00",
        asset_server.load("atlases/activities-00.png") as Handle<Image>,
    )];
    let jsons = [(
        "activities-00",
        asset_server.load("atlases/activities-00.atlas.json") as Handle<Atlas>,
    )];

    for i in imgs {
        folder.0.push((i.0, i.1));
    }
    for j in jsons {
        folder.1.push((j.0, j.1));
    }
}

fn check_textures(
    mut next_state: ResMut<NextState<AtlasesPluginState>>,
    atlases_folder: ResMut<AtlasesFolder>,
    // asset_server: Res<AssetServer>,
    atlas_descriptions: Res<Assets<Atlas>>,
    images: Res<Assets<Image>>,
) {
    let loaded_textures = atlases_folder.0.iter().all(|e| {
        // asset_server
        //     .get_load_state(e.1.id())
        //     .map(|x| {
        //         warn!("load state: {e:?} {x:?}");
        //         x == LoadState::Loaded
        //     })
        //     .unwrap_or(true)
        images.get(e.1.id()).is_some()
    });

    let loaded_jsons = atlases_folder.1.iter().all(|e| {
        // asset_server
        //     .get_load_state(e.1)
        //     .map(|x| {
        //         warn!("load state: {e:?} {x:?}");
        //         x == LoadState::Loaded
        //     })
        //     .unwrap_or(true)
        atlas_descriptions.get(e.1.id()).is_some()
    });

    warn!("loaded: {:?} {:?}", loaded_textures, loaded_jsons);

    if loaded_textures && loaded_jsons {
        next_state.set(AtlasesPluginState::Loaded);
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
    pub atlases: HashMap<&'static str, Handle<TextureAtlas>>,
    pub emoji_positions: HashMap<String, (&'static str, usize)>,
}

impl Emojis {
    fn from(
        atlases: HashMap<&'static str, Handle<TextureAtlas>>,
        map: &HashMap<&'static str, &Atlas>,
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

    pub fn lookup(&self, emoji: &str) -> Option<(&'static str, usize)> {
        self.emoji_positions.get(emoji).map(|(k, v)| (*k, *v))
    }

    pub fn sprite(&self, emoji: &str) -> Option<(&'static str, TextureAtlasSprite)> {
        let (k, index) = self.lookup(emoji)?;

        // let atlas = self.atlases.get(k).unwrap();

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

fn setup(
    mut commands: Commands,
    atlases_handlers: Res<AtlasesFolder>,
    atlas_folder: Res<AtlasesFolder>,
    asset_server: Res<AssetServer>,
    mut next_state: ResMut<NextState<AtlasesPluginState>>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,

    images: Res<Assets<Image>>,
    atlas_descriptions: Res<Assets<Atlas>>,
) {
    // let mut texture_atlas_builder = TextureAtlasBuilder::default();
    // let loaded_folder: &LoadedFolder = loaded_folders.get(&atlases_handlers.0).unwrap();

    let mut textures_handles = HashMap::new();
    let mut textures_positions = HashMap::new();

    for (path, handle) in atlas_folder.0.iter() {
        // let handle = images.get(*id).unwrap();

        // let path = handle.path().unwrap();

        warn!("path: {:?}", path);

        // let Some(texture) = textures.get(id) else {
        //     warn!(
        //         "{:?} did not resolve to an `Image` asset.",
        //         handle.path().unwrap()
        //     );
        //     continue;
        // };

        textures_handles.insert(*path, handle);
    }

    for (path, id) in atlas_folder.1.iter() {
        // let handle = asset_server.get_id_handle(*id).unwrap();

        // let path = handle.path().unwrap();

        warn!("path: {:?}", path);

        // let pure_path = path.path().with_extension("").with_extension("");

        // let dataid = handle.id();

        let data = atlas_descriptions.get(id.id()).unwrap();

        textures_positions.insert(*path, data);
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
}
