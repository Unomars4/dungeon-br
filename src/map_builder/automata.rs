use super::MapArchitect;
use crate::prelude::*;

pub struct AutomataArchitect {}

impl MapArchitect for AutomataArchitect {
    fn new(&mut self, rng: &mut RandomNumberGenerator) -> MapBuilder {
        let mut mb = MapBuilder {
            map: Map::new(),
            rooms: Vec::new(),
            player_start: Point::zero(),
            amulet_start: Point::zero(),
            monster_spawns: Vec::new(),
        };

        mb
    }

    fn random_noise(&mut self, rng: &mut RandomNumberGenerator, map: &mut Map) {
        map.tiles.iter_mut().for_each(|t| {
            let roll = rng.range(0, 100);
            if roll > 55 {
                *t = TileType::Floor
            } else {
                *t = TileType::Wall
            }
        });
    }
}
