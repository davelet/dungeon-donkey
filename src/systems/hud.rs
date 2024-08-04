use crate::prelude::*;

#[system]
#[read_component(Player)]
#[read_component(Health)]
pub fn hud(ecs: &SubWorld) {
    let mut health_query = <&Health>::query().filter(component::<Player>());
    let health = health_query.iter(ecs).nth(0).unwrap();

    let mut draw_batch = DrawBatch::new();
    draw_batch.target(2);
    draw_batch.print_centered(1, "Move around your character with directional keys.");
    draw_batch.bar_horizontal(Point::zero(), SCREEN_WIDTH * 2, health.current, health.max, ColorPair::new(RED, BLACK));
    draw_batch.print_color_centered(0, format!("Health: {} / {}", health.current, health.max), ColorPair::new(WHITE, RED));
    draw_batch.submit(10000).unwrap()

}