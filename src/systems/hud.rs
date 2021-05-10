use crate::prelude::*;

#[system]
#[read_component(Health)]
#[read_component(Player)]
pub fn hud(ecs: &SubWorld) {
    let player_health = <&Health>::query()
        .filter(component::<Player>())
        .iter(ecs)
        .next()
        .unwrap();

    let mut draw_batch = DrawBatch::new();
    draw_batch.target(2);
    draw_batch.print_centered(1, "Explore the dungeon. Cursor keys to move.");

    draw_batch.bar_horizontal(
        Point::zero(),
        SCREEN_WIDTH * 2,
        player_health.current,
        player_health.max,
        ColorPair::new(RED, BLACK),
    );
    draw_batch.print_color_centered(
        0,
        format!("{} / {}", player_health.current, player_health.max),
        ColorPair::new(WHITE, RED),
    );

    draw_batch.submit(10000).expect("Batch error");
}