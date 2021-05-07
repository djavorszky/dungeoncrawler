mod map;
mod map_builder;
mod player;

mod prelude {
    pub use crate::map::*;
    pub use crate::map_builder::*;
    pub use crate::player::*;

    pub use bracket_lib::prelude::*;

    pub const SCREEN_WIDTH: i32 = 80;
    pub const SCREEN_HEIGHT: i32 = 50;
}

use prelude::*;

struct State {
    map: Map,
    player: Player,
}

impl State {
    fn new() -> Self {
        let mut rng = RandomNumberGenerator::new();
        let mb = MapBuilder::new(&mut rng);

        State {
            map: mb.map,
            player: Player::new(mb.player_start),
        }
    }
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        ctx.cls();
        self.player.update(ctx, &self.map);
        self.map.render(ctx);
        self.player.render(ctx);
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
