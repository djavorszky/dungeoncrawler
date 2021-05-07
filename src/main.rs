mod map;

mod prelude {
    pub use crate::map::*;
    pub use bracket_lib::prelude::*;

    pub const SCREEN_WIDTH: i32 = 80;
    pub const SCREEN_HEIGHT: i32 = 50;
}

use prelude::*;

struct State {
    map: Map,
}

impl State {
    fn new() -> Self {
        State { map: Map::new() }
    }
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        ctx.cls();
        self.map.render(ctx);
        ctx.print_color(1, 1, WHITE, BLACK, "Hello terminal!");
    }
}

fn main() -> BError {
    let ctx = BTermBuilder::simple80x50()
        .with_title("Rusty Dungeon")
        .with_fps_cap(30.0)
        .build()?;

    main_loop(ctx, State::new())
}
