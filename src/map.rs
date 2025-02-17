//This file contains all the logic for the map functionality.
use crate::prelude::*;

const NUM_TILES: usize = (SCREEN_WIDTH * SCREEN_HEIGHT) as usize;

#[derive(Clone, Copy, PartialEq)]
pub enum TileType {
    Wall,
    Floor,
}

pub struct Map {
    pub tiles: Vec<TileType>,
}

impl Map {
    pub fn new() -> Self {
        Self {
            tiles: vec![TileType::Floor; NUM_TILES],
        }
    }
}

pub fn hello_map() {
    println!("Hello map 🗺️");
    println!("This is the total number of tiles needed: {} 🚀", NUM_TILES);
}
