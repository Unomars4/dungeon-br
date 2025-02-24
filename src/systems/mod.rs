mod player_input;

use crate::prelude::*;
use player_input::*;

pub fn build_scheduler() -> Schedule {
    Schedule::builder().build()
}
