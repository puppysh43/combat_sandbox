use super::CombatEncounter;
use crate::combat_action_type::*;
use crate::gamestate::*;
use crate::prelude::*;
use hecs::*;
use macroquad::prelude::*;

pub fn system(state: &mut GameState, combat_encounter: &mut CombatEncounter) {
    //first check with the combat encounter to see which entity/character is being currently controlled
    let active_entity = combat_encounter.next_turn().unwrap();
    //make an option to hold the queried action points
    let mut action_points_query: Option<ActionPoints> = None;
    for ap in state.ecs.query_one_mut::<&ActionPoints>(active_entity) {
        action_points_query = Some(*ap);
    }
    let mut action_points = action_points_query.unwrap();

    match state.control_state {
        CombatActionType::None => {
            //choose to do a ranged attack
            if is_key_pressed(KeyCode::F) {
                match action_points.significant_action() {
                    Ok(ap_left) => {
                        state.control_state = CombatActionType::RangedAttack;
                        println!(
                            "[Entity Name] has decided to attack and now has {} AP left!",
                            ap_left
                        );
                    }
                    Err(ap_left) => {
                        println!("[Entity Name] doesn't have enough action points to make a ranged attack, only has {} AP!", ap_left);
                    }
                }
            }
            //reload
            if is_key_pressed(KeyCode::R) {
                match action_points.minor_action() {
                    Ok(ap_left) => {
                        state.ecs.spawn((MOIReload::new(active_entity),));
                    }
                    Err(ap_left) => {
                        //
                    }
                }
            }
            //choose to aim at an enemy
            if is_key_pressed(KeyCode::A) {
                match action_points.minor_action() {
                    Ok(ap_left) => {
                        state.control_state = CombatActionType::Aiming;
                        //spawn in a reticule
                    }
                    Err(ap_left) => {
                        //
                    }
                }
            }
            //choose to start moving
            if is_key_pressed(KeyCode::S) {
                state.control_state = CombatActionType::Movement;
                println!("character has decided to move");
            }
            if is_key_down(KeyCode::LeftShift) && is_key_pressed(KeyCode::Q)
                || is_key_down(KeyCode::RightShift) && is_key_pressed(KeyCode::Q)
            {
                state.quitting = true;
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
            let delta = get_delta();
            if delta.is_some() {
                let mut pos: Option<IVec2> = None;
                for query_pos in state.ecs.query_one_mut::<&IVec2>(active_entity) {
                    pos = Some(*query_pos);
                }
                state.ecs.spawn((MOIWantsToMove::new(
                    true,
                    active_entity,
                    IVec2::new(
                        pos.unwrap().x + delta.unwrap().x,
                        pos.unwrap().y + delta.unwrap().y,
                    ),
                ),));
            }
            //esc is used to stop moving when the user is done
            if is_key_pressed(KeyCode::Escape) {
                state.control_state = CombatActionType::None;
            }
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

///generic function for getting a delta from user keypresses, either for selecting a direction
///to move or to select something in a direction around an entity
fn get_delta() -> Option<IVec2> {
    if is_key_pressed(KeyCode::Left) {
        //move the entity west
        Some(IVec2::new(-1, 0))
    } else if is_key_pressed(KeyCode::Up) {
        //move the entity north
        Some(IVec2::new(0, -1))
    } else if is_key_pressed(KeyCode::Right) {
        //move the entity east
        Some(IVec2::new(1, 0))
    } else if is_key_pressed(KeyCode::Down) {
        //move the entity south
        Some(IVec2::new(0, 1))
    } else if is_key_pressed(KeyCode::Kp4) {
        //move the entity west
        Some(IVec2::new(-1, 0))
    } else if is_key_pressed(KeyCode::Kp7) {
        //move the entity northwest
        Some(IVec2::new(-1, -1))
    } else if is_key_pressed(KeyCode::Kp8) {
        //move the entity north
        Some(IVec2::new(0, -1))
    } else if is_key_pressed(KeyCode::Kp9) {
        //move the entity northeast
        Some(IVec2::new(1, -1))
    } else if is_key_pressed(KeyCode::Kp6) {
        //move the entity east
        Some(IVec2::new(1, 0))
    } else if is_key_pressed(KeyCode::Kp3) {
        //move the entity southeast
        Some(IVec2::new(1, 1))
    } else if is_key_pressed(KeyCode::Kp2) {
        //move the entity south
        Some(IVec2::new(0, 1))
    } else if is_key_pressed(KeyCode::Kp1) {
        //move the entity southwest
        Some(IVec2::new(-1, 1))
    } else {
        None
    }
}

fn move_with_collision(state: &mut GameState, entity_to_move: Entity) {}
fn move_without_collision(state: &mut GameState, entity_to_move: Entity) {}
