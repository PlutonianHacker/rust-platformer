use bevy::prelude::*;
use bevy_kira_audio::AudioSource;
use bevy_asset_loader::{AssetCollection, AssetLoader};

use crate::{tiled::TiledMap, GameState};

pub struct LoadingPlugin;

impl Plugin for LoadingPlugin {
    fn build(&self, app: &mut App) {
        AssetLoader::new(GameState::Loading)
            .with_collection::<ImageAssets>()
            .with_collection::<FontAssets>()
            .with_collection::<AudioAssets>()
            .with_collection::<TileMapAssets>()
            .continue_to_state(GameState::Playing)
            .build(app);
    }
}

#[derive(AssetCollection)]
pub struct ImageAssets {
    #[asset(texture_atlas(tile_size_x = 64.0, tile_size_y = 64.0, columns = 2, rows = 1))]
    #[asset(path = "images/marine-idle.png")]
    pub marine_idle: Handle<TextureAtlas>,
    #[asset(path = "images/marine-run.png")]
    pub marine_run: Handle<Image>,
    #[asset(path = "images/o2_can.png")]
    pub o2_can: Handle<Image>,
    #[asset(path = "images/health-bar-1.png")]
    pub health_bar: Handle<Image>,
}

#[derive(AssetCollection)]
pub struct FontAssets {
    #[asset(path = "fonts/alagard.ttf")]
    pub alagard: Handle<Font>,
}

#[derive(AssetCollection)]
pub struct AudioAssets {
    #[asset(path = "audio/pick_up.ogg")]
    pub pick_up: Handle<AudioSource>,
}

#[derive(AssetCollection)]
pub struct TileMapAssets {
    #[asset(path = "data/map.tmx")]
    pub map: Handle<TiledMap>,
}
