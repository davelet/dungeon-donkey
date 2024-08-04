use crate::prelude::*;

#[system]
#[read_component(Player)]
#[read_component(Point)]
#[read_component(Enemy)]
#[write_component(Health)]
pub fn player_input(
    ecs: &mut SubWorld,
    commands: &mut CommandBuffer,
    #[resource] key: &Option<VirtualKeyCode>,
    #[resource] turn_state: &mut TurnState,
) {
    if let Some(key) = key {
        let delta = match key {
            VirtualKeyCode::Left => Point::new(-1, 0),
            VirtualKeyCode::Right => Point::new(1, 0),
            VirtualKeyCode::Up => Point::new(0, -1),
            VirtualKeyCode::Down => Point::new(0, 1),
            _ => Point::zero(),
        };
        let mut did = false;
        let mut players = <(Entity, &Point)>::query().filter(component::<Player>());
        let (player_entity, destination) = players
            .iter(ecs)
            .find_map(|(entity, pos)| Some((*entity, *pos + delta)))
            .unwrap();
        let mut enymies = <(Entity, &Point)>::query().filter(component::<Enemy>());
        let mut hit = false;
        if delta != Point::zero() {
            enymies
                .iter(ecs)
                .filter(|(_, pos)| *pos == &destination)
                .for_each(|(entity, _)| {
                    hit = true;
                    did = true;
                    commands.push((
                        (),
                        WantsToAttack {
                            entity: player_entity,
                            target: *entity,
                        },
                    ));
                });
            if !hit {
                did = true;
                commands.push((
                    (),
                    WantsToMove {
                        entity: player_entity,
                        destination,
                    },
                ));
            }

            /* players.iter_mut(ecs).for_each(|(entity, pos)| {
              let destination = *pos + delta;
              // if map.can_enter_tile(destination) {
              //     *pos = destination;
              //     camera.on_player_move(destination);
              //     *turn_state = TurnState::PlayerTurn;
              // }
              commands.push(((), WantsToMove { entity: *entity, destination }));
            });*/
        }
        if !did {
            if let Ok(health) = ecs.entry_mut(player_entity).unwrap().get_component_mut::<Health>() {
                health.current = i32::min(health.max, health.current + 1);
            }
        }
        *turn_state = TurnState::PlayerTurn;
    }
}
