use crate::prelude::*;

#[system]
#[read_component(Point)]
#[read_component(ChasingPlayer)]
#[read_component(Health)]
#[read_component(Player)]
pub fn chasing(ecs: &mut SubWorld, commands: &mut CommandBuffer, #[resource] map: &Map) {
    let mut movers = <(Entity, &Point, &ChasingPlayer)>::query();
    let mut positions = <(Entity, &Point, &Health)>::query();
    let mut player = <&Point>::query().filter(component::<Player>());

    let player_pos = player.iter(ecs).next().unwrap();
    let player_idx = map_idx_point(*player_pos);

    let search_targets = vec![player_idx];

    let dijkstra_map = DijkstraMap::new(SCREEN_WIDTH, SCREEN_WIDTH, &search_targets, map, 1024.0);

    movers.iter(ecs).for_each(|(entity, pos, _)| {
        let idx = map_idx_point(*pos);
        if let Some(destination) = DijkstraMap::find_lowest_exit(&dijkstra_map, idx, map) {
            let distance = DistanceAlg::Pythagoras.distance2d(*pos, *player_pos);

            let destination = if distance > 1.2 {
                map.index_to_point2d(destination)
            } else {
                *player_pos
            };

            let mut attacked = false;
            positions
                .iter(ecs)
                .filter(|(_, target_pos, _)| **target_pos == destination)
                .for_each(|(victim, _, _)| {
                    if ecs
                        .entry_ref(*victim)
                        .unwrap()
                        .get_component::<Player>()
                        .is_ok()
                    {
                        commands.push((
                            (),
                            AttackIntent {
                                attacker: *entity,
                                victim: *victim,
                            },
                        ));

                        attacked = true;
                    }
                });

            if !attacked {
                commands.push((
                    (),
                    MoveIntent {
                        entity: *entity,
                        destination,
                    },
                ));
            }
        }
    });
}
