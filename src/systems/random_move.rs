use crate::prelude::*;

#[system]
#[read_component(MovingRandomly)]
#[write_component(Point)]
pub fn random_move(ecs: &mut SubWorld, #[resource] map: &Map) {
    let mut moves = <(&mut Point, &MovingRandomly)>::query();
    moves.iter_mut(ecs).for_each(|(pos, _)| {
        let mut rng = RandomNumberGenerator::new();
        let dest = match rng.range(0, 4) {
            0 => Point::new(-1, 0),
            1 => Point::new(1, 0),
            2 => Point::new(0, -1),
            _ => Point::new(0, 1),
        } + *pos;
        if map.can_enter_tile(dest) {
            *pos = dest;
        }
    });
}
