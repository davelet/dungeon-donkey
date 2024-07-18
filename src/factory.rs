use bracket_lib::prelude::{BLACK, ColorPair, Point, to_cp437, WHITE};
pub use crate::prelude::*;

pub fn new_player(ecs: &mut World, pos: Point) {
    ecs.push((Player, pos, Render {
        color: ColorPair::new(WHITE, BLACK),
        glyph: to_cp437('@')
    }));
}