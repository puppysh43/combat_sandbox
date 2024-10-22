use crate::prelude::*;
use hecs::*;
use macroquad::prelude::*;

pub fn system(state: &mut GameState, combat_encounter: &mut CombatEncounter) {
    //this will process requests to end the turn of the current active entity
    //create a bool to store if there is an moi
    let mut is_moi = false;
    let mut cmd_buffer = CommandBuffer::new();
    for (id, _moi) in state.ecs.query_mut::<&MOIEndTurn>() {
        //if there's an MOI pulled up by the query then mark the bool as such
        is_moi = true;
        cmd_buffer.despawn(id);
    }
    cmd_buffer.run_on(&mut state.ecs);
    //if there is a request to end the turn of the currently active entity
    //then do so
    if is_moi {
        state
            .ecs
            .spawn((DebugLogMessage::new(String::from("completing entity turn")),));
        for (_id, combat_encounter) in state.ecs.query_mut::<&mut CombatEncounter>() {
            combat_encounter.complete_turn();
        }
    }
}
/*
                cmd_buffer.spawn((GameLogMessage::new(format!(
                    "current movement points: {}",
                    movement_points.current()
                )),));
*/
