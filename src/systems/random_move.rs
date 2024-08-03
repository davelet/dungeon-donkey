use crate::prelude::*;

#[system]
#[read_component(MovingRandomly)]
#[read_component(Point)]
pub fn random_move(ecs: &mut SubWorld, commands: &mut CommandBuffer) {
    let mut moves = <(Entity, &Point, &MovingRandomly)>::query();
    moves.iter_mut(ecs).for_each(|(entity, pos, _)| {
        let mut rng = RandomNumberGenerator::new();
        let destination = match rng.range(0, 4) {
            0 => Point::new(-1, 0),
            1 => Point::new(1, 0),
            2 => Point::new(0, -1),
            _ => Point::new(0, 1),
        } + *pos;
        // if map.can_enter_tile(dest) {
        //     *pos = dest;
        // }
        commands.push(((), WantsToMove { entity: *entity, destination }));
    });
}
