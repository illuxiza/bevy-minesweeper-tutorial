mod components;
mod resources;
mod events;
mod systems;

use bevy::{ prelude::*, render::camera::ScalingMode };
use events::{TileUncoverEvent, TileCheckEvent};
use resources::BoardOptions;

use crate::{ resources::Board, components::Coordinate };

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
        .add_systems(Startup, setup_board)
        .add_systems(Update, (
            systems::input::input_handler,
            systems::uncover::uncover_tiles,
            systems::check::check_tiles,
        ))
        .add_event::<TileUncoverEvent>()
        .add_event::<TileCheckEvent>()
        .run();
}

fn setup_board(
    mut commands: Commands,
    options: Res<BoardOptions>,
    mut board: ResMut<Board>,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>
) {
    println!("{:?}", options);
    board.reset(&options);
    println!("{}", board.console_output());

    let texture_handle = asset_server.load("texture.png");
    let texture_atlas = TextureAtlas::from_grid(
        texture_handle,
        Vec2::new(16.0, 16.0),
        4,
        4,
        None,
        None
    );
    let texture_atlas_handle: Handle<TextureAtlas> = texture_atlases.add(texture_atlas);

    let tile_size = 16.0;
    let board_size = Vec2::new(
        (options.width as f32) * tile_size,
        (options.height as f32) * tile_size
    );
    let offset = (board_size - tile_size) / 2.0;
    commands
        .spawn((
            SpriteBundle {
                sprite: Sprite {
                    color: Color::WHITE,
                    custom_size: Some(board_size),
                    ..default()
                },
                transform: Transform::from_xyz(0.0, 0.0, 1.0),
                ..default()
            },
        ))
        .with_children(|commands| {
            for y in 0..options.height {
                for x in 0..options.width {
                    commands.spawn((
                        SpriteSheetBundle {
                            texture_atlas: texture_atlas_handle.clone(),
                            sprite: TextureAtlasSprite {
                                index: 9,
                                ..Default::default()
                            },
                            transform: Transform::from_xyz(
                                tile_size * (x as f32) - offset.x,
                                offset.y - tile_size * (y as f32),
                                2.0
                            ),
                            ..default()
                        },
                        Coordinate::new(x, y),
                    ));
                }
            }
        });
}

fn setup_camera(mut commands: Commands, options: Res<BoardOptions>) {
    let mut camera = Camera2dBundle::default();
    camera.projection.scaling_mode = ScalingMode::AutoMin {
        min_width: (options.width as f32) * 16.0,
        min_height: (options.height as f32) * 16.0,
    };
    commands.spawn(camera);
}
