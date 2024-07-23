use bracket_lib::prelude::{BLACK, ColorPair, Point, RandomNumberGenerator, to_cp437, WHITE};
pub use crate::prelude::*;

pub fn new_player(ecs: &mut World, pos: Point) {
    ecs.push((Player, pos, Render {
        color: ColorPair::new(WHITE, BLACK),
        glyph: to_cp437('@'),
    }));
}

pub fn new_monster(ecs: &mut World, rng: &mut RandomNumberGenerator, pos: Point) {
    ecs.push((Enemy, pos, Render {
        color: ColorPair::new(WHITE, BLACK),
        glyph: match rng.range(0, 4) {
            1 => to_cp437('E'),
            2 => to_cp437('O'),
            3 => to_cp437('o'),
            _ => to_cp437('g')
        },
    }));
}