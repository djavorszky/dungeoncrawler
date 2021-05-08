use crate::prelude::*;

#[system]
#[read_component(Player)]
#[write_component(Point)]
pub fn player_input(
    ecs: &mut SubWorld,
    #[resource] map: &Map,
    #[resource] key: &Option<VirtualKeyCode>,
    #[resource] camera: &mut Camera,
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

        let mut players = <&mut Point>::query().filter(component::<Player>());

        players.iter_mut(ecs).for_each(|pos| {
            let new_position = *pos + delta;

            if map.can_enter_tile(new_position) {
                *pos = new_position;
                camera.on_player_move(new_position);
            }
        });
    }
}
