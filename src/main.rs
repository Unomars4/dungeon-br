mod map;
mod map_builder;
mod player;

mod prelude {
    pub use bracket_lib::prelude::*;
    pub const SCREEN_WIDTH: i32 = 80;
    pub const SCREEN_HEIGHT: i32 = 50;
    pub const DISPLAY_WIDTH: i32 = SCREEN_WIDTH / 2;
    pub const DISPLAY_HEIGHT: i32 = SCREEN_HEIGHT / 2;
    pub use crate::map::*;
    pub use crate::map_builder::*;
    pub use crate::player::*;
}

use prelude::*;

struct State {
    map: Map,
    player: Player,
}

impl State {
    fn new() -> Self {
        let mut rng: RandomNumberGenerator = RandomNumberGenerator::new();
        let mb: MapBuilder = MapBuilder::new(&mut rng);
        Self {
            map: mb.map,
            player: Player::new(mb.player_start),
        }
    }
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        ctx.cls();
        self.player.update(ctx, &self.map);
        self.map.render(ctx);
        self.player.render(ctx);
    }
}

fn main() -> BError {
    println!("Launching...😛🚀");

    let context = BTermBuilder::simple80x50()
        .with_title("Dungeon 🫣")
        .with_fps_cap(30.0)
        .build()?;

    main_loop(context, State::new())
}
