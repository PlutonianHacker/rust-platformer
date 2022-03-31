use bevy::prelude::*;

use crate::{util, GameState};

pub struct SplashPlugin;

impl Plugin for SplashPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(GameState::Splash).with_system(splash_setup))
            .add_system_set(
                SystemSet::on_update(GameState::Splash)
                    .with_system(countdown)
            )
            .add_system_set(
                SystemSet::on_exit(GameState::Splash)
                    .with_system(util::despawn_screen::<OnSplashScreen>),
            );
    }
}

#[derive(Component)]
struct OnSplashScreen;

fn splash_setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let logo = asset_server.load("images/logo.png");

    commands.spawn_bundle(UiCameraBundle::default());

    commands
        .spawn_bundle(ImageBundle {
            style: Style {
                margin: Rect::all(Val::Auto),
                size: Size::new(Val::Px(100.0), Val::Auto),
                ..Default::default()
            },
            image: UiImage(logo),
            ..Default::default()
        })
        .insert(OnSplashScreen);

    commands.insert_resource(Timer::from_seconds(1.0, false));
}

fn countdown(mut game_state: ResMut<State<GameState>>, time: Res<Time>, mut timer: ResMut<Timer>) {
    if timer.tick(time.delta()).finished() {
        game_state.set(GameState::Loading).unwrap();
    }
}
