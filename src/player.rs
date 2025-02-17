use crate::prelude::*;

struct Player {
    pos: Point,
}

impl Player {
    pub fn new() -> Self {
        Self {
            pos: Point::new(SCREEN_WIDTH / 2, SCREEN_HEIGHT / 2),
        }
    }

    pub fn render(&self, ctx: &mut BTerm) {
        ctx.set(self.pos.x, self.pos.y, RED, BLACK, to_cp437('@'));
    }
}
