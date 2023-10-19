use bevy::prelude::*;
use rand::{ thread_rng, Rng };

use super::BoardOptions;

const SQUARE_COORD: [(i8, i8); 8] = [
    // Top Left
    (-1, 1),
    // Top
    (0, 1),
    // Top right
    (1, 1),
    // Left
    (-1, 0),
    // Right
    (1, 0),
    // Bottom left
    (-1, -1),
    // Bottom
    (0, -1),
    // Bottom right
    (1, -1),
];

#[derive(Resource, Debug, Default)]
pub struct Board {
    pub map: Vec<Vec<i8>>,
}

impl Board {
    pub fn reset(&mut self, board_options: &Res<BoardOptions>) {
        let width: u16 = board_options.width;
        let height = board_options.height;

        let area = width * height;
        let bomb_count = board_options.bomb_count;
        let mut rng = thread_rng();
        let mut map: Vec<i8>;
        if bomb_count < area / 2 {
            map = (0..height * width)
                .into_iter()
                .map(|i: u16| if i < area - bomb_count { 0 } else { -1 })
                .collect();
            for i in area - bomb_count..area {
                let idd = rng.gen_range(0..=i) as usize;
                let i: usize = i as usize;
                if map[idd] != -1 {
                    map[idd] = -1 as i8;
                    map[i] = 0 as i8;
                }
            }
        } else {
            map = (0..height * width)
                .into_iter()
                .map(|i| if i < bomb_count { -1 } else { 0 })
                .collect();
            for i in bomb_count..area {
                let idd = rng.gen_range(0..=i) as usize;
                let i = i as usize;
                if map[idd] != 0 {
                    map[idd] = 0 as i8;
                    map[i] = -1 as i8;
                }
            }
        }

        self.map = map
            .chunks(width as usize)
            .map(|k| k.iter().cloned().collect::<Vec<_>>())
            .collect();

        for y in 0..height {
            for x in 0..width {
                let coord = (x, y);
                if self.is_bomb_at(coord) {
                    continue;
                }
                self.map[y as usize][x as usize] = self.bomb_count_at(coord) as i8;
            }
        }
    }

    fn is_bomb_at(&self, coord: (u16, u16)) -> bool {
        let y = coord.1 as usize;
        let x = coord.0 as usize;
        if y >= self.map.len() {
            return false;
        }
        if x >= self.map[y].len() {
            return false;
        }
        self.map[y][x] == -1
    }

    fn bomb_count_at(&self, coord: (u16, u16)) -> u8 {
        if self.is_bomb_at(coord) {
            return 0;
        }
        let res = self
            .safe_square_at(coord)
            .filter(|coord| self.is_bomb_at(*coord))
            .count();
        res as u8
    }

    fn safe_square_at(&self, coord: (u16, u16)) -> impl Iterator<Item = (u16, u16)> {
        SQUARE_COORD.iter()
            .copied()
            .map(move |tuple| (
                (tuple.0 as i16) + (coord.0 as i16),
                (tuple.1 as i16) + (coord.1 as i16),
            ))
            .filter(|coord| coord.0 >= 0 && coord.1 >= 0)
            .map(|coord| (coord.0 as u16, coord.1 as u16))
    }

    pub fn console_output(&self) -> String {
        let separator: String = (0..=self.map.len() * 3)
            .into_iter()
            .map(|_| '-')
            .collect();
        format!(
            "{}\n{}\n{}",
            separator,
            self.map
                .iter()
                .map(|row|
                    format!(
                        "|{}|",
                        row
                            .iter()
                            .map(|column| format!("{:2}", column))
                            .collect::<Vec<_>>()
                            .join(" ")
                    )
                )
                .collect::<Vec<_>>()
                .join("\n"),
            separator
        )
    }
}