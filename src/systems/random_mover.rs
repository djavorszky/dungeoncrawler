use crate::prelude::*;

#[system]
#[read_component(Point)]
#[read_component(MovingRandomly)]
pub fn random_movement(ecs: &mut SubWorld, commands: &mut CommandBuffer) {
    <(Entity, &Point)>::query()
        .filter(component::<MovingRandomly>())
        .iter(ecs)
        .for_each(|(entity, pos)| {
            let mut rng = RandomNumberGenerator::new();

            let delta = match rng.range(0, 4) {
                0 => Point::new(1, 0),
                1 => Point::new(-1, 0),
                2 => Point::new(0, 1),
                _ => Point::new(0, -1),
            };

            let destination = *pos + delta;

            commands.push((
                (),
                WantsToMove {
                    entity: *entity,
                    destination,
                },
            ));
        });
}
