use crate::prelude::*;

#[system]
#[read_component(Player)]
#[read_component(Health)]
#[read_component(AmuletOfYala)]
#[read_component(Point)]
pub fn end_turn(#[resource] turn_state: &mut TurnState, ecs: &SubWorld, #[resource] map: &Map) {
    let mut player_hp = <(&Health, &Point)>::query().filter(component::<Player>());
    let mut amulet = <&Point>::query().filter(component::<AmuletOfYala>());
    let amulet_default = Point::new(-1, -1);
    let amulet_pos = amulet.iter(ecs).nth(0).unwrap_or(&amulet_default);
    let current_state = turn_state.clone();
    let mut new_state = match turn_state {
        TurnState::AwaitingInput => return,
        TurnState::PlayerTurn => TurnState::MonsterTurn,
        TurnState::MonsterTurn => TurnState::AwaitingInput,
        _ => current_state,
    };

    player_hp.iter(ecs).for_each(|(hp, pos)| {
        let idx = map.point2d_to_index(*pos);

        if hp.current < 1 {
            new_state = TurnState::GameOver;
        }

        if pos == amulet_pos {
            new_state = TurnState::Victory;
        }

        if map.tiles[idx] == TileType::Exit {
            new_state = TurnState::NextLevel;
        }
    });

    *turn_state = new_state;
}
