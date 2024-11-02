use crate::prelude::*;
use hecs::*;

///This function handles ALL game logic related to ranged attacks being made, but not the visual/audio elements
pub fn system(state: &mut GameState) {
    //standard system boilerplate, grab all mois and purge them w/ a command buffer
    let mut ranged_attack_mois: Vec<MOIRangedAttack> = Vec::new();
    let mut command_buffer = CommandBuffer::new();
    for (id, moi) in state.ecs.query_mut::<&MOIRangedAttack>() {
        ranged_attack_mois.push(moi.clone());
        command_buffer.despawn(id);
    }
    command_buffer.run_on(&mut state.ecs);
    //then we run through all the MOIs
    for moi in ranged_attack_mois.iter() {
        let (shooter, target) = (moi.shooter, moi.target);
        //now we need to see if the target is an object or a character
        //we're gonna do this by querying for the presence of attributes
        //if they have attributes then we need to put the MOI BACK into the system and set it up so that the targeted entity
        //can decide what their reaction is gonna be either w/ an AI subroutine or manual user input
        //otherwise damage them through the item damaging system from traveller
    }
}
