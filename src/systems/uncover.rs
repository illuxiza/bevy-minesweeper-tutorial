use bevy::prelude::*;

use crate::{
    events::{ TileUncoverEvent, TileCheckEvent },
    resources::Board,
    components::Coordinate,
};

pub fn uncover_tiles(
    mut board: ResMut<Board>,
    mut tiles: Query<(&mut TextureAtlasSprite, &Coordinate)>,
    mut tile_uncover_ev: EventReader<TileUncoverEvent>,
    mut tile_check_ev: EventWriter<TileCheckEvent>
) {
    for ev in tile_uncover_ev.iter() {
        let select = ev.0;
        if board.op_map[select.y as usize][select.x as usize] != 0 {
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
