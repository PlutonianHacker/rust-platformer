use base_crawler::GamePlugin;
use bevy::{prelude::*, input::system};

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::BLACK))
        .add_plugins(DefaultPlugins)
        .add_plugin(GamePlugin)
        .add_system(system::exit_on_esc_system)
        .run()
}
