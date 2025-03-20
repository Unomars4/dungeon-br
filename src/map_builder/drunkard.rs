use super::MapArchitect;
use crate::prelude::*;

pub struct DrunkardArchitect {}

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
