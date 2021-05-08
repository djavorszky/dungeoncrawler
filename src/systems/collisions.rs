use crate::prelude::*;

#[system]
#[read_component(Point)]
#[read_component(Player)]
#[read_component(Enemy)]
pub fn collision(ecs: &SubWorld, commands: &mut CommandBuffer) {
    let player_pos = <&Point>::query()
        .filter(component::<Player>())
        .iter(ecs)
        .next()
        .expect("Unable to find player position");

    <(Entity, &Point)>::query()
        .filter(component::<Enemy>())
        .iter(ecs)
        .filter(|(_, pos)| *pos == player_pos)
        .for_each(|(entity, _)| {
            commands.remove(*entity);
        });
}
