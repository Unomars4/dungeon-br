use super::MapArchitect;
use crate::prelude::*;

pub struct DrunkardArchitect {}

const STAGGER_DISTANCE: usize = 400;
const NUM_TILES: usize = (SCREEN_WIDTH * SCREEN_WIDTH) as usize;
const DESIRED_FLOOR: usize = NUM_TILES / 3;

impl DrunkardArchitect {
    fn drunkard(&mut self, start: &Point, rng: &mut RandomNumberGenerator, map: &mut Map) {
        let mut drunkard_pos = start.clone();
        let mut distance_staggerd = 0;

        loop {
            let drunk_idx = map.point2d_to_index(drunkard_pos);
            map.tiles[drunk_idx] = TileType::Floor;

            match rng.range(0, 4) {
                0 => drunkard_pos.x += 1,
                1 => drunkard_pos.x -= 1,
                2 => drunkard_pos.y += 1,
                _ => drunkard_pos.y -= 1,
            }

            if !map.in_bounds(drunkard_pos) {
                break;
            }

            distance_staggerd += 1;
            if distance_staggerd > STAGGER_DISTANCE {
                break;
            }
        }
    }
}

impl MapArchitect for DrunkardArchitect {
    fn new(&mut self, rng: &mut RandomNumberGenerator) -> MapBuilder {
        let mut mb = MapBuilder {
            map: Map::new(),
            player_start: Point::zero(),
            amulet_start: Point::zero(),
            monster_spawns: Vec::new(),
            rooms: Vec::new(),
        };

        mb
    }
}
