mod entity_render;
mod map_render;
mod player_input;
// mod collisions;
mod end_turn;
mod fight;
mod hud;
mod movement;
mod random_move;
mod tooltip;
mod chasing;

use chasing::chasing_system;
// use collisions::collisions_system;
use end_turn::end_turn_system;
use entity_render::entity_render_system;
use random_move::random_move_system;

use crate::prelude::*;

pub fn build_input_scheduler() -> Schedule {
    Schedule::builder()
        .add_system(player_input::player_input_system())
        .flush()
        .add_system(map_render::map_render_system())
        .add_system(entity_render_system())
        .add_system(hud::hud_system())
        .add_system(tooltip::tooltip_system())
        .build()
}

pub fn build_player_scheduler() -> Schedule {
    Schedule::builder()
        .add_system(fight::fight_system())
        .flush()
        .add_system(movement::movement_system())
        .flush()
        // .add_system(collisions_system())
        .flush()
        .add_system(map_render::map_render_system())
        .add_system(entity_render_system())
        .add_system(hud::hud_system())
        .add_system(end_turn_system())
        .build()
}

pub fn build_enemy_scheduler() -> Schedule {
    Schedule::builder()
        .add_system(random_move_system())
        .add_system(chasing_system())
        .flush()
        .add_system(fight::fight_system())
        .flush()
        .add_system(movement::movement_system())
        .flush()
        // .add_system(collisions_system())
        .flush()
        .add_system(map_render::map_render_system())
        .add_system(entity_render_system())
        .add_system(hud::hud_system())
        .add_system(end_turn_system())
        .build()
}
