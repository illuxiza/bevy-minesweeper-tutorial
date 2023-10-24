mod components;
mod resources;
mod events;
mod systems;

use bevy::{ prelude::*, render::camera::ScalingMode };
use events::{ TileUncoverEvent, TileCheckEvent, TileMarkEvent };
use resources::BoardOptions;

use crate::resources::Board;

fn main() {
    App::new()
        .init_resource::<BoardOptions>()
        .init_resource::<Board>()
        .add_plugins(
            DefaultPlugins.set(ImagePlugin::default_nearest()).set(WindowPlugin {
                primary_window: Some(Window {
                    title: "MineSweeper".into(),
                    resolution: (320.0, 320.0).into(),
                    ..Default::default()
                }),
                ..default()
            })
        )
        .add_systems(Startup, setup_camera)
        .add_systems(Startup, systems::board::setup_board)
        .add_systems(Update, (
            systems::input::input_handler,
            systems::tiles::uncover_tiles,
            systems::tiles::check_tiles,
            systems::tiles::mark_tiles,
        ))
        .add_event::<TileUncoverEvent>()
        .add_event::<TileCheckEvent>()
        .add_event::<TileMarkEvent>()
        .run();
}

fn setup_camera(mut commands: Commands, options: Res<BoardOptions>) {
    let mut camera = Camera2dBundle::default();
    camera.projection.scaling_mode = ScalingMode::AutoMin {
        min_width: (options.width as f32) * 16.0,
        min_height: (options.height as f32) * 16.0,
    };
    commands.spawn(camera);
}
