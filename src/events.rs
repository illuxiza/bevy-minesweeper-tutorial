use bevy::prelude::Event;

use crate::components::Coordinate;

#[derive(Debug, Copy, Clone, Event)]
pub struct TileUncoverEvent(pub Coordinate);

#[derive(Debug, Copy, Clone, Event)]
pub struct TileCheckEvent(pub Coordinate);
