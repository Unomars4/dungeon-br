//This file contains all the logic for the map functionality.
use crate::prelude::*;

const NUM_TILES: usize = (SCREEN_WIDTH * SCREEN_HEIGHT) as usize;

pub fn hello_map() {
    println!("Hello map 🗺️");
    println!("This is the total number of tiles needed: {} 🚀", NUM_TILES);
}
