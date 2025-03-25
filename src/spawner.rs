//Module handles spawning entities
pub use crate::prelude::*;

pub fn spawn_entity(ecs: &mut World, rng: &mut RandomNumberGenerator, pos: Point) {
    let roll = rng.roll_dice(1, 6);
    match roll {
        1 => spawn_healing_potion(ecs, pos),
        2 => spawn_dungeon_scroll(ecs, pos),
        _ => spawn_monster(ecs, pos, rng),
    }
}

pub fn spawn_healing_potion(ecs: &mut World, pos: Point) {
    ecs.push((
        Item,
        pos,
        Render {
            color: ColorPair::new(BLACK, WHITE),
            glyph: to_cp437('!'),
        },
        ProvidesHealing { amount: 15 },
        Name("Healing Potion".to_string()),
    ));
}

pub fn spawn_dungeon_scroll(ecs: &mut World, pos: Point) {
    ecs.push((
        Item,
        Name("Dungeon Map Scroll".to_string()),
        pos,
        Render {
            color: ColorPair::new(BLACK, WHITE),
            glyph: to_cp437('{'),
        },
        ProvidesDungeonMap {},
    ));
}

pub fn spawn_amulet_of_yala(ecs: &mut World, pos: Point) {
    ecs.push((
        Item,
        AmuletOfYala,
        pos,
        Render {
            color: ColorPair::new(WHITE, BLACK),
            glyph: to_cp437('/'),
        },
        Name("Amulet of Yala".to_string()),
    ));
}

pub fn spawn_player(ecs: &mut World, pos: Point) {
    ecs.push((
        Player { map_level: 0 },
        pos,
        Render {
            color: ColorPair::new(WHITE, BLACK),
            glyph: to_cp437('@'),
        },
        Health {
            current: 20,
            max: 20,
        },
        FieldOfView::new(8),
    ));
}

fn goblin() -> (i32, String, FontCharType) {
    (1, "Goblin".to_string(), to_cp437('g'))
}

fn orc() -> (i32, String, FontCharType) {
    (2, "Orc".to_string(), to_cp437('o'))
}

pub fn spawn_monster(ecs: &mut World, pos: Point, rng: &mut RandomNumberGenerator) {
    let (hp, name, glyph) = match rng.roll_dice(1, 10) {
        1..8 => goblin(),
        _ => orc(),
    };
    ecs.push((
        Enemy,
        pos,
        Render {
            color: ColorPair::new(WHITE, BLACK),
            glyph,
        },
        ChasingPlayer {},
        Health {
            current: hp,
            max: hp,
        },
        Name(name),
    ));
}
