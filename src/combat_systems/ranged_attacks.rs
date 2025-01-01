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

        //this should essentially "Recycle" the same moi over and over needlessly
        //this is /probably/ a really stupid way of dealing with this problem

        //check for an Attributes component which is the core that defines Characters from
        //any other kind of entity and is also directly relevant to if we deal damage
        if state.ecs.satisfies::<&Attributes>(target).unwrap() {
            //characters have the ability to make reactions in response to being attacked
            //so we need to push the moi back into the system and prompt the player
            state.ecs.spawn((moi.clone(),));
        } else {
            //if it's an object then query for the object health component and roll a damage roll for it
            //for now though just do nothing :3
        }
    }

    //now iterate through all the complete ranged attack MOIs that include a reaction
    let mut attacks_with_reactions: Vec<MOIRangedAttackWithReaction> = Vec::new();
    for (id, moi) in state.ecs.query_mut::<&MOIRangedAttackWithReaction>() {
        attacks_with_reactions.push(moi.clone());
        command_buffer.despawn(id);
    }
    //
    for moi in attacks_with_reactions.iter() {
        let (shooter, target, reaction) = (moi.shooter, moi.target, moi.reaction);
        let shooter_weapon = state.ecs.query_one_mut
        //here is where the actual attack roll gets fully processed

        //depending on the attack deplete the ammo a certain amount
        let mut shots_made = 1; //currently this is just 1 but later this will match
                                //according to attack type


        //get the attacker's equipped weapon to check for any attack roll

        //bonuses from attachments or weapon traits/relevant traits

        //get attacker's relevant weapon skill DM and relevant attribute DM for the attack roll

        //calculate the range modifier for the attack

        //check if the target is in cover/has the incover component and if so apply
        //the cover bonuses

        //check the stance of both shooter and target for DMs

        //check for any aimed at/aiming at components

        //add all the DMs together and apply them to the skill check of the relevant skill determined by
        //the weapon type

        //do the skillcheck w/ an average difficulty and get the effect back

        //if Effect > 0 then make a damage MOI w/ the damage of the weapon + Effects from skillcheck
    }
}
