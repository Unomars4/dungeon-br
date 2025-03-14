use legion::systems::CommandBuffer;

use crate::prelude::*;

#[system(for_each)]
#[read_component(Player)]
#[read_component(FieldOfView)]
pub fn movement(
    entity: &Entity,
    wants_move: &WantsToMove,
    #[resource] map: &mut Map,
    #[resource] camera: &mut Camera,
    ecs: &mut SubWorld,
    command: &mut CommandBuffer,
) {
    if map.can_enter_tile(wants_move.destination) {
        command.add_component(wants_move.entity, wants_move.destination);

        if let Ok(entry) = ecs.entry_ref(wants_move.entity) {
            if let Ok(fov) = entry.get_component::<FieldOfView>() {
                command.add_component(wants_move.entity, fov.clone_dirty());
                if entry.get_component::<Player>().is_ok() {
                    camera.on_player_move(wants_move.destination);
                    fov.visible_tiles
                        .iter()
                        .for_each(|pos| map.revealed_tiles[map_idx(pos.x, pos.y)] = true);
                }
            }
        }
    }

    command.remove(*entity);
}
