use bevy::prelude::*;

use crate::{ resources::Board, events::{GameOverEvent, GameWinEvent}, components::Coordinate, GameState };

pub fn game_over(
    board: Res<Board>,
    mut tiles: Query<(&mut TextureAtlasSprite, &Coordinate)>,
    mut game_over_ev: EventReader<GameOverEvent>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    for ev in game_over_ev.read() {
        next_state.set(GameState::Over);
        let select = ev.0;
        for (mut sprite, coord) in &mut tiles {
            let num = board.map[coord.y as usize][coord.x as usize];
            let op: i8 = board.op_map[coord.y as usize][coord.x as usize];
            if num == -1 && op == 0 {
                sprite.index = 10;
            }
            if op == 2 && num != -1 {
                sprite.index = 12;
            }
            if *coord == select {
                println!("Over Tile: {:?}", select);
                sprite.index = 11;
            }
        }
    }
}

pub fn game_win(
    board: Res<Board>,
    mut tiles: Query<(&mut TextureAtlasSprite, &Coordinate)>,
    mut game_win_ev: EventReader<GameWinEvent>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    for _ in game_win_ev.read() {
        println!("Win!");
        next_state.set(GameState::Over);
        for (mut sprite, coord) in &mut tiles {
            let num = board.map[coord.y as usize][coord.x as usize];
            let op: i8 = board.op_map[coord.y as usize][coord.x as usize];
            if op == 0 && num == -1 {
                sprite.index = 13;
            }
        }
    }
}