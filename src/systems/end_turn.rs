use crate::prelude::*;

#[system]
#[read_component(Player)]
#[read_component(Health)]
pub fn end_turn(#[resource] turn_state: &mut TurnState, ecs: &SubWorld) {
    let mut player_hp = <&Health>::query().filter(component::<Player>());
    let current_state = turn_state.clone();
    let mut new_state = match turn_state {
        TurnState::AwaitingInput => return,
        TurnState::PlayerTurn => TurnState::MonsterTurn,
        TurnState::MonsterTurn => TurnState::PlayerTurn,
        _ => current_state,
    };

    player_hp.iter(ecs).for_each(|hp| {
        if hp.current < 1 {
            new_state = TurnState::GameOver;
        }
    });

    *turn_state = new_state;
}
