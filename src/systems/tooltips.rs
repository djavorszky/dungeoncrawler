use crate::prelude::*;

#[system]
#[read_component(Point)]
#[read_component(Name)]
#[read_component(Health)]
pub fn tooltips(ecs: &SubWorld, #[resource] mouse_pos: &Point, #[resource] camera: &Camera) {
    let offset = Point::new(camera.left_x, camera.top_y);
    let map_pos = *mouse_pos + offset;
    let mut draw_batch = DrawBatch::new();
    draw_batch.target(2);

    <(Entity, &Point, &Name)>::query()
        .iter(ecs)
        .filter(|(_, pos, _)| **pos == map_pos)
        .for_each(|(entity, pos, name)| {
            let screen_pos = *mouse_pos * 4 + Point::new(-2, -3);

            draw_batch.print(screen_pos, &name.0);

            if let Ok(health) = ecs.entry_ref(*entity).unwrap().get_component::<Health>() {
                draw_batch.bar_horizontal(
                    screen_pos + Point::new(0, 1),
                    8,
                    health.current,
                    health.max,
                    ColorPair::new(RED, BLACK),
                );
            }
        });

    draw_batch.submit(10100).expect("Barch error");
}
