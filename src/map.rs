use crate::prelude::*;

const NUM_TILES: usize = (SCREEN_WIDTH * SCREEN_HEIGHT) as usize;

#[derive(Copy, Clone, PartialEq)]
pub enum TileType {
    Wall,
    Floor,
}

pub struct Map {
    pub tiles: Vec<TileType>,
}

impl Map {
    pub fn new() -> Self {
        Self {
            tiles: vec![TileType::Floor; NUM_TILES],
        }
    }

    pub fn in_bounds_point(&self, point: Point) -> bool {
        self.in_bounds(point.x, point.y)
    }

    pub fn in_bounds(&self, x: i32, y: i32) -> bool {
        (0..SCREEN_WIDTH).contains(&x) && (0..SCREEN_HEIGHT).contains(&y)
    }

    pub fn can_enter_tile(&self, point: Point) -> bool {
        self.in_bounds_point(point) && self.tiles[map_idx_point(point)] == TileType::Floor
    }

    pub fn try_idx(&self, x: i32, y: i32) -> Option<usize> {
        self.in_bounds(x, y).then(|| map_idx(x, y))
    }
}

pub fn map_idx(x: i32, y: i32) -> usize {
    ((y * SCREEN_WIDTH) + x) as usize
}

#[inline]
pub fn map_idx_point(point: Point) -> usize {
    map_idx(point.x, point.y)
}
