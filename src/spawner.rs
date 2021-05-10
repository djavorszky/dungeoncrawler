use crate::prelude::*;

pub fn spawn_player(ecs: &mut World, pos: Point) {
    ecs.push((
        Player,
        pos,
        Render {
            color: ColorPair::new(WHITE, BLACK),
            glyph: to_cp437('@'),
        },
        Health {
            current: 20,
            max: 20,
        },
    ));
}

pub fn spawn_monster(ecs: &mut World, rng: &mut RandomNumberGenerator, pos: Point) {
    let monster = match rng.roll_dice(1, 10) {
        1..=6 => goblin(),
        _ => orc(),
    };

    ecs.push((
        Enemy,
        MovingRandomly,
        pos,
        Render {
            color: ColorPair::new(WHITE, BLACK),
            glyph: monster.glyph,
        },
        Health {
            current: monster.hp,
            max: monster.hp,
        },
        Name(monster.name),
    ));
}

fn orc() -> Monster {
    Monster {
        hp: 2,
        name: "Orc".to_string(),
        glyph: to_cp437('o'),
    }
}
fn goblin() -> Monster {
    Monster {
        hp: 1,
        name: "Goblin".to_string(),
        glyph: to_cp437('g'),
    }
}

struct Monster {
    hp: i32,
    name: String,
    glyph: FontCharType,
}
