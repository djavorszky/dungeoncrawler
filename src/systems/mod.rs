mod collisions;
mod end_turn;
mod entity_renderer;
mod map_render;
mod movement;
mod player_input;
mod random_mover;

use crate::prelude::*;

pub fn build_input_scheduler() -> Schedule {
    Schedule::builder()
        .add_system(player_input::player_input_system())
        .flush()
        .add_system(map_render::map_render_system())
        .add_system(entity_renderer::entity_render_system())
        .build()
}

pub fn build_player_scheduler() -> Schedule {
    Schedule::builder()
        .add_system(movement::movement_system())
        .flush()
        .add_system(collisions::collision_system())
        .flush()
        .add_system(map_render::map_render_system())
        .add_system(entity_renderer::entity_render_system())
        .add_system(end_turn::end_turn_system())
        .build()
}

pub fn build_monster_scheduler() -> Schedule {
    Schedule::builder()
        .add_system(random_mover::random_movement_system())
        .flush()
        .add_system(movement::movement_system())
        .flush()
        .add_system(collisions::collision_system())
        .flush()
        .add_system(map_render::map_render_system())
        .add_system(entity_renderer::entity_render_system())
        .add_system(end_turn::end_turn_system())
        .build()
}
