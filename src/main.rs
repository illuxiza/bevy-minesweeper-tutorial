mod components;
mod resources;
mod events;
mod systems;

use bevy::{ prelude::*, render::camera::ScalingMode };
use events::{ TileUncoverEvent, TileCheckEvent, TileMarkEvent, GameOverEvent, GameWinEvent };
use resources::BoardOptions;

use crate::resources::Board;

#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
pub enum GameState {
    #[default]
    InGame,
    Over,
}

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
        .add_systems(OnEnter(GameState::InGame), systems::board::setup_board)
        .add_systems(
            Update,
            (
                systems::input::input_handler,
                systems::tiles::uncover_tiles,
                systems::tiles::check_tiles,
                systems::tiles::mark_tiles,
                systems::over::game_over,
                systems::over::game_win,
            ).run_if(in_state(GameState::InGame))
        )
        .add_state::<GameState>()
        .add_event::<TileUncoverEvent>()
        .add_event::<TileCheckEvent>()
        .add_event::<TileMarkEvent>()
        .add_event::<GameOverEvent>()
        .add_event::<GameWinEvent>()
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
