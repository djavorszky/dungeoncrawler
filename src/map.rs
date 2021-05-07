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

    pub fn render(&self, ctx: &mut BTerm, camera: &Camera) {
        ctx.set_active_console(0);
        for y in camera.top_y..camera.bottom_y {
            for x in camera.left_x..camera.right_x {
                if self.in_bounds(x, y) {
                    let idx = map_idx(x, y);

                    match self.tiles[idx] {
                        TileType::Floor => ctx.set(
                            x - camera.left_x,
                            y - camera.top_y,
                            WHITE,
                            BLACK,
                            to_cp437('.'),
                        ),
                        TileType::Wall => ctx.set(
                            x - camera.left_x,
                            y - camera.top_y,
                            WHITE,
                            BLACK,
                            to_cp437('#'),
                        ),
                    }
                }
            }
        }
    }

    pub fn in_bounds_point(&self, point: Point) -> bool {
        self.in_bounds(point.x, point.y)
    }

    pub fn in_bounds(&self, x: i32, y: i32) -> bool {
        x >= 0 && x < SCREEN_WIDTH && y >= 0 && y < SCREEN_HEIGHT
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
