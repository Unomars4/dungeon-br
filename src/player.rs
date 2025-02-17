use crate::prelude::*;

pub struct Player {
    pos: Point,
}

impl Player {
    pub fn new() -> Self {
        Self {
            pos: Point::new(SCREEN_WIDTH / 2, SCREEN_HEIGHT / 2),
        }
    }

    pub fn update(&mut self, ctx: &mut BTerm, map: &Map) {
        if let Some(key) = ctx.key {
            let change_in_pos = match key {
                VirtualKeyCode::Left => Point::new(-1, 0),
                VirtualKeyCode::Right => Point::new(1, 0),
                VirtualKeyCode::Up => Point::new(0, -1),
                VirtualKeyCode::Down => Point::new(0, 1),
                _ => Point::zero(),
            };
            let new_pos = self.pos + change_in_pos;
            if map.can_enter_tile(self.pos) {
                self.pos = new_pos;
            }
        }
    }

    pub fn render(&self, ctx: &mut BTerm) {
        ctx.set(self.pos.x, self.pos.y, RED, BLACK, to_cp437('@'));
    }
}
