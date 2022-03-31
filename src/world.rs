use bevy::prelude::*;
use bevy_ecs_tilemap::Map;
//use bevy_ecs_tilemap::prelude::*;

use crate::{
    loading::TileMapAssets,
    //loading::ImageAssets,
    tiled::{TiledMap, TiledMapBundle},
    util,
    GameState,
};

#[derive(Component)]
struct Item;

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(GameState::Playing).with_system(setup_world))
            .add_system_set(
                SystemSet::on_update(GameState::Playing)
                    .with_system(util::set_texture_filters_to_nearest),
            );
    }
}

fn setup_world(mut commands: Commands, tilemaps: Res<TileMapAssets>) {
    let handle: Handle<TiledMap> = tilemaps.map.clone();

    let map_entity = commands.spawn().id();

    commands.entity(map_entity).insert_bundle(TiledMapBundle {
        tiled_map: handle,
        map: Map::new(0u16, map_entity),
        transform: Transform::from_xyz(0.0, 0.0, 0.0),
        ..Default::default()
    });
}