mod map;

mod prelude {
    pub use bracket_lib::prelude::*;
    pub const SCREEN_WIDTH: i32 = 80;
    pub const SCREEN_HEIGHT: i32 = 80;
    pub use crate::map::*;
}

use prelude::*;

struct State {
    map: Map,
}

fn main() {
    println!("Let's begin...😛");
    hello_map();
}
