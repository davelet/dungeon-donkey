use bracket_lib::prelude::{BError, BTerm, BTermBuilder, main_loop};
use bracket_lib::terminal::GameState;

const SCREEN_WIDTH: i32 = 80;
const SCREEN_HEIGHT: i32 = 50;
const FRAME_DURATION: f32 = 75.0;

fn main() -> BError {
    let c = BTermBuilder::simple80x50().with_title("Dungeon Donkey").build()?;
    main_loop(c, State::new())
}

struct State {}

impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        ctx.cls();
        ctx.print(0, 0, "helll");
    }
}


impl State {
    fn new() -> Self {
        Self {}
    }
}

