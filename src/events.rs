use bevy::prelude::Event;

use crate::components::Coordinate;

#[derive(Debug, Copy, Clone, Event)]
pub struct TileUncoverEvent(pub Coordinate, pub bool);

#[derive(Debug, Copy, Clone, Event)]
pub struct TileCheckEvent(pub Coordinate);

#[derive(Debug, Copy, Clone, Event)]
pub struct TileMarkEvent(pub Coordinate);
