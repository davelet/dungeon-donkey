use bracket_lib::random::RandomNumberGenerator;
use bracket_lib::terminal::GameState;

use prelude::*;

mod map;
mod player;
mod map_builder;
mod camera;

mod prelude {
    pub(crate) use bracket_lib::prelude::{BError, BTerm, BTermBuilder, main_loop};

    pub(crate) use crate::camera::*;
    pub(crate) use crate::map::*;
    pub(crate) use crate::map_builder::*;
    pub(crate) use crate::player::*;

    pub(crate) const SCREEN_WIDTH: i32 = 80;
    pub(crate) const SCREEN_HEIGHT: i32 = 50;
}

pub const DISPLAY_WIDTH: i32 = SCREEN_WIDTH / 2;
pub const DISPLAY_HEIGHT:i32 = SCREEN_HEIGHT / 2;
fn main() -> BError {
    // let c = BTermBuilder::simple80x50()
    //     .with_fps_cap(30f32)
    //     .with_title("Dungeon Donkey").build()?;
    let c = BTermBuilder::new()
        .with_title("Dungeon Donkey")
        .with_fps_cap(30f32)
        .with_dimensions(DISPLAY_WIDTH, DISPLAY_HEIGHT)
        .with_resource_path("resources/")
        .with_font("font.png", 32, 32)
        .with_simple_console(DISPLAY_WIDTH, DISPLAY_HEIGHT, "font.png")
        .with_simple_console_no_bg(DISPLAY_WIDTH, DISPLAY_HEIGHT, "font.png")
        .build()?;
    main_loop(c, State::new())
}

struct State {
    map: Map,
    player: Player,
    camera: Camera
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        ctx.set_active_console(0);
        ctx.cls();
        ctx.set_active_console(1);
        ctx.cls();
        self.player.update(ctx, &self.map, &mut self.camera);
        self.map.render(ctx, &self.camera);
        self.player.render(ctx, &self.camera);
    }
}


impl State {
    fn new() -> Self {
        let mut rng = RandomNumberGenerator::new();
        let mb = MapBuilder::new(&mut rng);
        Self {
            map: mb.map,
            player: Player::new(mb.player_start),
            camera: Camera::new(mb.player_start)
        }
    }
}

