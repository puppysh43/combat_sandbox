use super::CombatEncounter;
use crate::combat_action_type::*;
use crate::gamestate::*;
use crate::prelude::*;
use hecs::*;
use macroquad::prelude::*;

pub fn system(state: &mut GameState, combat_encounter: &mut CombatEncounter) {
    //first check with the combat encounter to see which entity/character is being currently controlled
    let active_entity = combat_encounter.next_turn().unwrap();
    match state.control_state {
        CombatActionType::None => {
            //choose to do a ranged attack
            if is_key_pressed(KeyCode::F) {
                state.control_state = CombatActionType::RangedAttack;
            }
            //reload
            if is_key_pressed(KeyCode::R) {
                state.ecs.spawn((MOIReload::new(active_entity),));
            }
        }
        CombatActionType::RangedAttack => {
            //ranged attack code goes here [move reticule MOI or send ranged attack MOI]
        }
        CombatActionType::MeleeAttack => {
            //melee attack code goes here
        }
        CombatActionType::Leadership => {
            //leadership check code IG
        }
        CombatActionType::Aiming => {
            //aiming code - move reticule and select then do aiming MOI
        }
        CombatActionType::ChangingStance => {
            //changing stance code, change stance moi w/ direction enum I guess
        }
        CombatActionType::Drawing => {
            //this one will be a pain in the ass will need a menu and different options and all that bullshit
        }
        CombatActionType::Movement => {
            //
        }
        CombatActionType::Grapple => {
            //this one is gonna be really complicated lmao
        }
        CombatActionType::Interact => {
            //this one will probably go unused for a while
        }
        CombatActionType::UseItem(useitemstate) => {
            match &useitemstate {
                UseItemState::Selecting => {
                    //
                }
                UseItemState::Using => {
                    //
                }
            }
        }
        CombatActionType::Look => {
            //player can move reticule, press enter to get a short description printed to the log, or use shift enter to get a detailed description
            //including identifying equipment but using 1AP in the process
        }
        CombatActionType::PickUp => {
            //this one also may not need to be its own control state we shall see
        }
    }
}
