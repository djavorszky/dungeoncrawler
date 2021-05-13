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

    pub fn valid_exit(&self, pos: Point, delta: Point) -> Option<usize> {
        let destination = pos + delta;

        if self.in_bounds_point(destination) && self.can_enter_tile(destination) {
            Some(self.point2d_to_index(destination))
        } else {
            None
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

impl BaseMap for Map {
    fn get_available_exits(&self, idx: usize) -> SmallVec<[(usize, f32); 10]> {
        let mut exits = SmallVec::new();
        let location = self.index_to_point2d(idx);

        if let Some(idx) = self.valid_exit(location, Point::new(-1, 0)) {
            exits.push((idx, 1.0));
        }
        if let Some(idx) = self.valid_exit(location, Point::new(1, 0)) {
            exits.push((idx, 1.0));
        }
        if let Some(idx) = self.valid_exit(location, Point::new(0, -1)) {
            exits.push((idx, 1.0));
        }
        if let Some(idx) = self.valid_exit(location, Point::new(0, 1)) {
            exits.push((idx, 1.0));
        }

        exits
    }

    fn get_pathing_distance(&self, idx1: usize, idx2: usize) -> f32 {
        DistanceAlg::Pythagoras.distance2d(self.index_to_point2d(idx1), self.index_to_point2d(idx2))
    }
}

impl Algorithm2D for Map {
    fn dimensions(&self) -> Point {
        Point::new(SCREEN_WIDTH, SCREEN_WIDTH)
    }

    fn in_bounds(&self, pos: Point) -> bool {
        self.in_bounds_point(pos)
    }
}
