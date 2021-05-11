use crate::prelude::*;

#[system]
#[read_component(Point)]
#[read_component(Player)]
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

        if delta.x == 0 && delta.y == 0 {
            return;
        }

        let mut players = <(Entity, &Point)>::query().filter(component::<Player>());

        let (player_entity, destination) = players
            .iter(ecs)
            .find_map(|(entity, position)| Some((*entity, *position + delta)))
            .unwrap();

        if let Some((entity, _)) = <(Entity, &Point)>::query()
            .filter(component::<Enemy>())
            .iter(ecs)
            .find(|(_, pos)| **pos == destination)
        {
            commands.push((
                (),
                AttackIntent {
                    attacker: player_entity,
                    victim: *entity,
                },
            ));
        } else {
            commands.push((
                (),
                MoveIntent {
                    entity: player_entity,
                    destination,
                },
            ));
        }
        *turn_state = TurnState::PlayerTurn;
    }
}
