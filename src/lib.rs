pub mod actions;
pub mod audio;
pub mod camera;
pub mod loading;
pub mod player;
pub mod resources;
pub mod splash;
pub mod tiled;
pub mod util;
pub mod world;

use audio::InternalAudioPlugin;
use bevy::prelude::*;
use bevy_ecs_tilemap::TilemapPlugin;
use camera::CameraPlugin;
use splash::SplashPlugin;

use crate::tiled::TiledMapPlugin;
use actions::ActionsPlugin;
use loading::LoadingPlugin;
use player::PlayerPlugin;
use resources::ResourcePlugin;
use world::WorldPlugin;

#[derive(Clone, Eq, PartialEq, Debug, Hash)]
enum GameState {
    Splash,
    Loading,
    Playing,
}

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_state(GameState::Splash)
            .add_plugin(InternalAudioPlugin)
            .add_plugin(TilemapPlugin)
            .add_plugin(TiledMapPlugin)
            .add_plugin(SplashPlugin)
            .add_plugin(LoadingPlugin)
            .add_plugin(ActionsPlugin)
            .add_plugin(WorldPlugin)
            .add_plugin(PlayerPlugin)
            .add_plugin(CameraPlugin)
            .add_plugin(ResourcePlugin);
    }
}
