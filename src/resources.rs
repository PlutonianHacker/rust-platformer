use bevy::prelude::*;

use crate::{
    loading::{ImageAssets, TileMapAssets},
    tiled::TiledMap,
    GameState,
};

pub struct ResourcePlugin;

impl Plugin for ResourcePlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(GameState::Playing).with_system(setup));
    }
}

#[derive(Component)]
pub struct O2(pub f32);

pub fn setup(
    mut commands: Commands,
    maps: Res<Assets<TiledMap>>,
    tilemaps: Res<TileMapAssets>,
    images: Res<ImageAssets>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let handle = tilemaps.map.clone();
    if let Some(tile_map) = maps.get(handle) {
        let object_groups = &tile_map.map.object_groups;

        for object_group in object_groups {
            for object in &object_group.objects {
                let x = object.x;
                let y = object.y;

                let texture_atlas =
                    TextureAtlas::from_grid(images.o2_can.clone(), Vec2::new(object.width * 2.0, object.height * 2.0), 2, 1);
                let texture_atlas_handle = texture_atlases.add(texture_atlas);

                commands
                    .spawn_bundle(SpriteSheetBundle {
                        texture_atlas: texture_atlas_handle,
                        transform: Transform {
                            translation: Vec3::new(x + object.width / 2.0, y + object.height / 2.0 - 68.0, 2.0),
                            scale: Vec3::splat(0.5),
                            ..Default::default()
                        },
                        ..Default::default()
                    })
                    .insert(O2(100.0));
            }
        }
    }
}
