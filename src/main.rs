mod map;

use bracket_lib::terminal::GameState;
use prelude::*;

mod prelude {
    pub(crate) use bracket_lib::prelude::{BError, BTerm, BTermBuilder, main_loop};

    pub(crate) const SCREEN_WIDTH: i32 = 80;
    pub(crate) const SCREEN_HEIGHT: i32 = 50;

    pub(crate) use crate::map::*;
}


fn main() -> BError {
    let c = BTermBuilder::simple80x50()
        .with_fps_cap(30f32)
        .with_title("Dungeon Donkey").build()?;
    main_loop(c, State::new())
}

struct State {
    map: Map,
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        ctx.cls();
        self.map.render(ctx)
    }
}


impl State {
    fn new() -> Self {
        Self {
            map: Map::new()
        }
    }
}

