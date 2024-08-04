use crate::prelude::*;


#[system]
#[read_component(WantsToAttack)]
#[write_component(Health)]
pub fn fight(ecs: &mut SubWorld, commands: &mut CommandBuffer) {
    let mut attacks = <(Entity, &WantsToAttack)>::query();
    let targets: Vec<(Entity, Entity)> = attacks.iter(ecs).map(|(entity, attack)| (*entity, attack.target)).collect();
    targets.iter().for_each(|(message, target)| {
        if let Ok(health) = ecs.entry_mut(*target).unwrap().get_component_mut::<Health>() {
            health.current -= 1;
            if health.current < 1 {
                commands.remove(*target);
            }
        }
        commands.remove(*message);
    });
}