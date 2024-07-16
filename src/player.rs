use bracket_lib::color::{BLACK, WHITE};
use bracket_lib::prelude::{Point, to_cp437, VirtualKeyCode};
use crate::map::Map;
use crate::prelude::BTerm;

pub struct Player {
    pub position: Point,
}

impl Player {
    pub fn new(point: Point) -> Self {
        Self { position: point }
    }

    pub fn render(&self, ctx: &mut BTerm) {
        ctx.set(self.position.x,
                self.position.y,
                WHITE,
                BLACK,
                to_cp437('@'))
    }

    pub fn update(&mut self, ctx: &mut BTerm, map: &Map) {
        if let Some(key) = ctx.key {
            let delta = match key {
                VirtualKeyCode::Left => Point::new(-1, 0),
                VirtualKeyCode::Right => Point::new(1, 0),
                VirtualKeyCode::Up => Point::new(0, -1),
                VirtualKeyCode::Down => Point::new(0, 1),
                _ => Point::zero()
            };
            let new_position = self.position + delta;
            if map.can_enter_tile(new_position) {
                self.position = new_position
            }
        }
    }
}