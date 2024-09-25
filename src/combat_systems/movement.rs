/*
needs to add checking movement points and decrementing them when the entity moves!!! This is basically the last thing I think.
*/
use crate::prelude::*;
use hecs::*;
use macroquad::prelude::*;
///Game system that will process all movement MOIs
pub fn system(state: &mut GameState) {
    //make a vec to hold all the MOIs yr gonna be processing
    let mut moving_mois: Vec<MOIWantsToMove> = Vec::new();
    //make a command buffer so you can purge all the MOIs after extracting them (and other utilities idk yet)
    let mut cmd_buffer = CommandBuffer::new();
    //bool to track if the entity actually moved
    let mut has_moved = false;
    //query all of the movement MOIs
    for (id, moi) in state.ecs.query_mut::<&MOIWantsToMove>() {
        //transfer them to our outside buffer
        moving_mois.push(*moi);
        //add a despawn command for the MOI in our command buffer
        cmd_buffer.despawn(id);
    }
    //run the command buffer on the ECS to get rid of the MOIs
    cmd_buffer.run_on(&mut state.ecs);
    //now we can actually read through them and modify the state according to their context
    for moi in moving_mois.iter() {
        //deconstruct the current message of intent
        let (collision, entity, destination) = moi.get();
        //check if the entity has a pool of movement points (like all PCs and NPCs will) and make sure they have enough
        let mut movement_points: Option<MovementPoints> = None;
        if state.ecs.entity(entity).is_ok() {
            if state.ecs.entity(entity).unwrap().has::<&MovementPoints>() {
                for mp_comp in state.ecs.query_one_mut::<&MovementPoints>(entity) {
                    movement_points = Some(mp_comp.clone());
                }
            }
        }

        let mut can_move = true;
        //if the character has a movement point component then you need to check if they have enough
        if movement_points.is_some() {
            can_move = movement_points.unwrap().can_move();
            println!("entity has the movement points to move");
        }
        println!("can_move:{}", can_move);
        //only let them move if they actually can move
        if can_move {
            //check if the entity moving has collision (ex. characters do but reticules don't)
            if collision {
                //make a vec to hold all the possible points that the moving entity could collide with
                let mut collision_points: Vec<IVec2> = Vec::new();
                //collect all of the positions of the entities that can be collided with
                for (_id, collision_pos) in state.ecs.query_mut::<With<&IVec2, &Collideable>>() {
                    collision_points.push(*collision_pos);
                }
                //first check if the entity would be blocked by a map tile
                if state.map.can_enter_tile(destination) {
                    //variable to track if the moving entity would collide w/ any others
                    let mut collides = false;
                    //then iterate through the collection of positions of other entities the moving entity could possibly collide with
                    for pos in collision_points.iter() {
                        if &destination == pos {
                            collides = true;
                        }
                    }
                    //if the entity wouldn't be colliding with any other entity then it's okay to move them!
                    if !collides {
                        cmd_buffer.insert(entity, (destination,));
                        cmd_buffer.run_on(&mut state.ecs);
                        println!("tried to move entity!");
                        has_moved = true;
                    }
                }
            } else {
                //if the entity moving doesn't have collision to its movement you just need to check that whereever
                //it's moving to is in bounds of the map
                if state.map.in_bounds(destination) {
                    cmd_buffer.insert(entity, (destination,));
                    has_moved = true;
                }
            }
            //finally, if the character has moved a tile, then if applicable consume one of their movement points
            if has_moved && movement_points.is_some() {
                movement_points.unwrap().decrement();
                cmd_buffer.insert(entity, (movement_points.unwrap(),));
                cmd_buffer.run_on(&mut state.ecs);
                println!("decremented the movement points of the moving entity!");
                println!(
                    "movement points left: {}",
                    movement_points.unwrap().current()
                )
            }
            //then update the movement points component of the entity that's moving
        } else {
            println!("doesn't have enough movement points to move!");
        }
    }
}
