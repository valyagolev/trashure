use std::path::PathBuf;

use bevy::{asset::LoadedFolder, prelude::*, utils::HashMap};
use bevy_common_assets::json::{self, JsonAssetPlugin};
use itertools::Itertools;
use serde::{Deserialize, Serialize};

pub struct AtlasesPlugin;

impl Plugin for AtlasesPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((JsonAssetPlugin::<Atlas>::new(&["atlas.json"]),))
            // insert_resource(Configuration::default())
            //     // .insert_resource(Persistent::<Configuration>::n())
            // .add_systems(Startup, load_textures);
            .add_state::<AppState>()
            .add_systems(OnEnter(AppState::Setup), load_textures)
            .add_systems(Update, check_textures.run_if(in_state(AppState::Setup)))
            .add_systems(OnEnter(AppState::Finished), setup);
        // .add_systems(Update, on_modify_configuration);
        //     .register_type::<Configuration>() // you need to register your type to display it
        //     .add_plugins(ResourceInspectorPlugin::<Configuration>::default());
    }
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash, States)]
enum AppState {
    #[default]
    Setup,
    Finished,
}

#[derive(Resource, Default)]
struct AtlasesFolder(Handle<LoadedFolder>);

fn check_textures(
    mut next_state: ResMut<NextState<AppState>>,
    rpg_sprite_folder: ResMut<AtlasesFolder>,
    mut events: EventReader<AssetEvent<LoadedFolder>>,
) {
    // Advance the `AppState` once all sprite handles have been loaded by the `AssetServer`
    for event in events.read() {
        if event.is_loaded_with_dependencies(&rpg_sprite_folder.0) {
            next_state.set(AppState::Finished);
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Emoji {
    pub emoji: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ImagePos {
    pub emoji: Emoji,
    pub top: usize,
    pub left: usize,
    pub index: usize,
}

#[derive(Serialize, Deserialize, Debug, bevy::asset::Asset, bevy::reflect::TypePath)]
pub struct Atlas {
    pub emojis: Vec<ImagePos>,
}

impl Atlas {
    fn lookup(&self, emoji: &str) -> Option<TextureAtlasSprite> {
        let em = self.emojis.iter().find(|x| x.emoji.emoji == emoji)?;

        Some(TextureAtlasSprite {
            index: em.index,
            ..Default::default()
        })
    }
}

fn lookup_atlases<'a, K>(
    atlases: &'a HashMap<K, &Atlas>,
    emoji: &str,
) -> Option<(&'a K, TextureAtlasSprite)> {
    let (k, tas) =
        atlases
            .iter()
            .filter_map(|(k, a)| Some((k, a.lookup(emoji)?)))
            .exactly_one()
            .ok()?;

    Some((k, tas))
}

fn load_textures(mut commands: Commands, asset_server: Res<AssetServer>) {
    // load multiple, individual sprites from a folder
    commands.insert_resource(AtlasesFolder(asset_server.load_folder("atlases")));
}

fn setup(
    mut commands: Commands,
    atlases_handlers: Res<AtlasesFolder>,
    asset_server: Res<AssetServer>,
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

                dbg!(&data);

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
        atlases.insert(k, texture_atlases.add(atlas));
    }

    let (k, sprite) = lookup_atlases(&textures_positions, "🎁").unwrap();

    commands.spawn(Camera2dBundle::default());

    commands.spawn(SpriteSheetBundle {
        transform: Transform {
            translation: Vec3::new(150.0, 0.0, 0.0),
            scale: Vec3::splat(1.0),
            ..default()
        },
        sprite,
        texture_atlas: atlases[k].clone(),
        ..default()
    });
    // draw the atlas itself
    // commands.spawn(SpriteBundle {
    //     texture: texture_atlas_texture,
    //     transform: Transform::from_xyz(-300.0, 0.0, 0.0),
    //     ..default()
    // });
}
