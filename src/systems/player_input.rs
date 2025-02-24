use crate::prelude::*;

#[system]
#[write_component(Point)]
#[read_component(Player)]
fn player_input(
    &mut ecs: World,
    #[resource] map: Map,
    #[resource] key: &Option<VirtualKeyCode>,
    #[resource] camera: Camera,
) {
}
