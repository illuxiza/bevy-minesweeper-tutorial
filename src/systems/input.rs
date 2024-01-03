use bevy::{ prelude::*, input::{ mouse::MouseButtonInput, ButtonState } };

use crate::{ components::Coordinate, events::{TileUncoverEvent, TileMarkEvent} };

pub fn input_handler(
    windows: Query<&mut Window>,
    camera: Query<(&Camera, &GlobalTransform)>,
    tiles: Query<(&Coordinate, &GlobalTransform)>,
    mut button_evr: EventReader<MouseButtonInput>,
    mut tile_uncover_ev: EventWriter<TileUncoverEvent>,
    mut tile_mark_ev: EventWriter<TileMarkEvent>
) {
    let window = windows.single();
    let (camera, camera_transform) = camera.single();

    for event in button_evr.read() {
        // 捕获松开鼠标的事件
        if event.state == ButtonState::Released {
            // 将鼠标点击的位置转换为相机中的实际位置
            let position = window
                .cursor_position()
                .and_then(|cursor| camera.viewport_to_world_2d(camera_transform, cursor))
                .unwrap();
            for (coord, transform) in tiles.iter() {
                // 判断点击所在的实体
                if in_transform(transform, position) {
                    println!("Mouse button release: {:?}", coord);
                    match event.button {
                        // 当点击左键时，发送一个翻开的事件
                        MouseButton::Left => {
                            tile_uncover_ev.send(TileUncoverEvent(coord.clone(), false));
                        }
                        MouseButton::Right => {
                            tile_mark_ev.send(TileMarkEvent(coord.clone()));
                        }
                        _ => {}
                    }
                }
            }
        }
    }
}

/// 判断点击位置是否在Entity位置中
fn in_transform(transform: &GlobalTransform, position: Vec2) -> bool {
    let inx =
        transform.translation().x - 8.0 < position.x &&
        transform.translation().x + 8.0 > position.x;
    let iny =
        transform.translation().y - 8.0 < position.y &&
        transform.translation().y + 8.0 > position.y;
    inx && iny
}
