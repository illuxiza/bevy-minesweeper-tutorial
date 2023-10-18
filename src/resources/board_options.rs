use bevy::prelude::*;

#[derive(Resource, Debug)]
pub struct BoardOptions {
    pub width: u16,
    pub height: u16,
    pub bomb_count: u16,
}

impl Default for BoardOptions {
    fn default() -> Self {
        Self { width: 9, height: 9, bomb_count: 10 }
    }
}
