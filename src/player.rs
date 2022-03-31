use bevy::prelude::*;
use bevy::sprite::collide_aabb::collide;
use bevy_ecs_tilemap::{Tile, TilePos};
use bevy_kira_audio::Audio;

use crate::actions::Actions;
use crate::loading::{AudioAssets, ImageAssets};
use crate::resources::O2;
use crate::GameState;

pub struct PlayerPlugin;

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct HealthBar;

/// Track entity's speed.
#[derive(Component)]
pub struct Speed(pub f32);

#[derive(Component)]
pub struct Velocity {
    pub x: f32,
    pub y: f32,
}

/// Tracks game character's current and max hit points.
#[derive(Component)]
pub struct Hp {
    pub value: f32,
    pub max: f32,
}

impl Hp {
    pub fn new(value: f32, max: f32) -> Self {
        Self { value, max }
    }
}

#[derive(Clone, Eq, PartialEq, Debug, Hash)]
pub enum PlayerState {
    Idle,
    Run,
    Die,
}

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_state(PlayerState::Idle)
            .add_system_set(SystemSet::on_enter(GameState::Playing).with_system(spawn_player))
            .add_system_set(
                SystemSet::on_update(GameState::Playing)
                    .with_system(player_movement)
                    .with_system(player_vs_items)
                    .with_system(player_health)
                    .with_system(animate_player_system),
            );
    }
}

fn spawn_player(
    mut commands: Commands,
    images: Res<ImageAssets>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    commands
        .spawn_bundle(SpriteSheetBundle {
            texture_atlas: images.marine_idle.clone(),
            transform: Transform {
                translation: Vec3::new(1216.0 + 16.0, 640.0, 2.0),
                scale: Vec3::splat(1.0),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(Player)
        .insert(Speed(150.0))
        .insert(Velocity { x: 0.0, y: 0. })
        .insert(Hp::new(200.0, 200.0))
        .insert(Timer::from_seconds(0.2, true));

    let texture_atlas =
        TextureAtlas::from_grid(images.health_bar.clone(), Vec2::new(64.0, 9.2), 1, 10);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);

    commands
        .spawn_bundle(SpriteSheetBundle {
            sprite: TextureAtlasSprite {
                index: 1,
                ..Default::default()
            },
            texture_atlas: texture_atlas_handle,
            transform: Transform {
                translation: Vec3::new(0.0, 32.0, 2.0),
                scale: Vec3::splat(1.0),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(HealthBar);
}

fn animate_player_system(
    time: Res<Time>,
    texture_atlases: Res<Assets<TextureAtlas>>,
    mut player_query: Query<
        (&mut Timer, &mut TextureAtlasSprite, &Handle<TextureAtlas>),
        With<Player>,
    >,
) {
    for (mut timer, mut sprite, texture_atlas_handle) in player_query.iter_mut() {
        timer.tick(time.delta());
        if timer.finished() {
            let texture_atlas = texture_atlases.get(texture_atlas_handle).unwrap();
            sprite.index = (sprite.index + 1) % texture_atlas.textures.len();
        }
    }
}

fn player_health(
    texture_atlases: Res<Assets<TextureAtlas>>,
    mut health_bar_query: Query<(&mut TextureAtlasSprite, &Handle<TextureAtlas>), With<HealthBar>>,
    player_query: Query<&Hp, With<Player>>,
) {
    let _player_hp = player_query.single();
    for (mut sprite, texture_atlas_handle) in health_bar_query.iter_mut() {
        let _texture_atlas = texture_atlases.get(texture_atlas_handle).unwrap();
        sprite.index = 0;
    }
}

fn player_movement(
    time: Res<Time>,
    actions: Res<Actions>,
    mut _app_state: ResMut<State<PlayerState>>,
    mut player_query: Query<(&mut Transform, &mut TextureAtlasSprite, &Speed), With<Player>>,
    mut health_bar_query: Query<&mut Transform, (With<HealthBar>, Without<Player>)>,
    tile_query: Query<(&Tile, &TilePos)>,
) {
    if actions.player_movement.is_none() {
        return;
    }

    for (mut transform, mut sprite, speed) in player_query.iter_mut() {
        if actions.player_movement.unwrap().x == -1.0 {
            sprite.flip_x = true;
        } else {
            sprite.flip_x = false;
        }

        let movement = Vec3::new(
            actions.player_movement.unwrap().x * speed.0 * time.delta_seconds(),
            0.0,
            0.0,
        );

        let player_translation = Vec3::new(
            transform.translation.x + movement.x,
            transform.translation.y + movement.y,
            0.0,
        );

        let player_size = Vec2::new(64.0, 64.0);
        for (tile, pos) in tile_query.iter() {
            match tile.texture_index {
                9 | 10 => {
                    if collide(
                        player_translation,
                        player_size,
                        Vec3::new( pos.0 as f32 * 32.0 + 16.0, pos.1 as f32 * 32.0 + 16.0, 0.0),
                        Vec2::new(32.0, 32.0),
                    )
                    .is_some()
                    {
                        return;
                    }
                }
                _ => continue,
            }
        }

        let mut health_bar_transform = health_bar_query.single_mut();

        transform.translation += movement;
        health_bar_transform.translation += movement;
    }
}

fn player_vs_items(
    mut commands: Commands,
    audio_assets: Res<AudioAssets>,
    audio: Res<Audio>,
    mut player_query: Query<&mut Transform, With<Player>>,
    mut item_query: Query<(Entity, &Transform, &O2), Without<Player>>,
) {
    for player_transform in player_query.iter_mut() {
        let player_size = Vec2::new(64.0, 64.0);

        for (entity, item, _o2) in item_query.iter_mut() {
            if collide(
                player_transform.translation,
                player_size,
                item.translation,
                Vec2::new(24.0, 32.0),
            )
            .is_some()
            {
                commands.entity(entity).despawn();
                audio.play(audio_assets.pick_up.clone());
            }
        }
    }
}
