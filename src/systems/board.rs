use bevy::prelude::*;

use crate::{resources::{BoardOptions, Board}, components::Coordinate};

pub fn setup_board(
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
