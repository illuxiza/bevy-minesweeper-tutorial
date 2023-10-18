mod resources;

use bevy::prelude::*;
use resources::BoardOptions;

fn main() {
    App::new()
        .init_resource::<BoardOptions>()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup_camera)
        .add_systems(Startup, print_options)
        .run();
}

fn print_options(options: Res<BoardOptions>) {
    println!("{:?}", options);
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}
