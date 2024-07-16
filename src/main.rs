mod map;
mod player;
mod map_builder;

use bracket_lib::prelude::Point;
use bracket_lib::random::RandomNumberGenerator;
use bracket_lib::terminal::GameState;
use prelude::*;

mod prelude {
    pub(crate) use bracket_lib::prelude::{BError, BTerm, BTermBuilder, main_loop};

    pub(crate) const SCREEN_WIDTH: i32 = 80;
    pub(crate) const SCREEN_HEIGHT: i32 = 50;

    pub(crate) use crate::map::*;
    pub(crate) use crate::player::*;
    pub(crate) use crate::map_builder::*;
}


fn main() -> BError {
    let c = BTermBuilder::simple80x50()
        .with_fps_cap(30f32)
        .with_title("Dungeon Donkey").build()?;
    main_loop(c, State::new())
}

struct State {
    map: Map,
    player: Player,
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        ctx.cls();
        self.player.update(ctx, &self.map);
        self.map.render(ctx);
        self.player.render(ctx);
    }
}


impl State {
    fn new() -> Self {
        let mut rng = RandomNumberGenerator::new();
        let mb = MapBuilder::new(&mut rng);
        Self {
            map: mb.map,
            player: Player::new(mb.player_start),
        }
    }
}

