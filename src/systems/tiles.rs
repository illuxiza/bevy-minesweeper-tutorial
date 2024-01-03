use bevy::prelude::*;

use crate::{
    events::{ TileUncoverEvent, TileCheckEvent, TileMarkEvent, GameOverEvent, GameWinEvent },
    resources::Board,
    components::Coordinate,
};

pub fn uncover_tiles(
    mut board: ResMut<Board>,
    mut tiles: Query<(&mut TextureAtlasSprite, &Coordinate)>,
    mut tile_uncover_ev: EventReader<TileUncoverEvent>,
    mut tile_check_ev: EventWriter<TileCheckEvent>
) {
    for ev in tile_uncover_ev.read() {
        let select: Coordinate = ev.0;
        let auto = ev.1;
        let op = board.op_map[select.y as usize][select.x as usize];
        // 已经掀开的不会再判断
        if op == 1 {
            continue;
        }
        // 点击事件时，标志的也不会再判断，自动扩展时，依然会翻开
        if !auto && op == 2 {
            continue;
        }
        // 找到对应坐标实体的精灵，修改index为实际单元格
        for (mut sprite, coord) in &mut tiles {
            if *coord == select {
                let column = board.map[select.y as usize][select.x as usize];
                let index = match column {
                    1..=8 => { column as usize }
                    -1 => { 10 }
                    _ => { 0 }
                };
                sprite.index = index;
                board.op_map[select.y as usize][select.x as usize] = 1;
                tile_check_ev.send(TileCheckEvent(select));
                break;
            }
        }
    }
}

pub fn check_tiles(
    board: Res<Board>,
    mut tile_uncover_ev: EventWriter<TileUncoverEvent>,
    mut tile_check_ev: EventReader<TileCheckEvent>,
    mut game_over_ev: EventWriter<GameOverEvent>,
    mut game_win_ev: EventWriter<GameWinEvent>,
) {
    for ev in tile_check_ev.read() {
        let select = ev.0;
        println!("Check Tile: {:?}", select);
        let num = board.map[select.y as usize][select.x as usize];
        // 只有当格子是空白格时，触发连锁
        if num == 0 {
            board
                .adust_cover_around((select.x, select.y))
                .into_iter()
                .for_each(|coord|
                    tile_uncover_ev.send(TileUncoverEvent(Coordinate::new(coord.0, coord.1), true))
                );
        }
        // 当格子是地雷时，输出爆炸
        if num == -1 {
            game_over_ev.send(GameOverEvent(select));
            break;
        }
        if board.is_complete() {
            game_win_ev.send(GameWinEvent);
        }
    }
}

pub fn mark_tiles(
    mut board: ResMut<Board>,
    mut tiles: Query<(&mut TextureAtlasSprite, &Coordinate)>,
    mut tile_mark_ev: EventReader<TileMarkEvent>
) {
    for ev in tile_mark_ev.read() {
        let select = ev.0;
        let op = board.op_map[select.y as usize][select.x as usize];
        if op == 1 {
            continue;
        }
        for (mut sprite, coord) in &mut tiles {
            if *coord == select {
                if op == 0 {
                    println!("Mark Tile: {:?}", select);
                    sprite.index = 13;
                    board.op_map[select.y as usize][select.x as usize] = 2;
                    break;
                } else if op == 2 {
                    println!("Unmark Tile: {:?}", select);
                    sprite.index = 9;
                    board.op_map[select.y as usize][select.x as usize] = 0;
                    break;
                }
            }
        }
    }
}