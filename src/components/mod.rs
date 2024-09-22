mod rpg_components;
mod weapons;

use hecs::*;
//this is the big mod file for components that will expand as I add more components

///Component that allows an entity to be rendered, contains the hashmap key needed to retrieve
///the necessary Texture2D from the texture atlas
pub struct Renderable {
    sprite: String,
}
impl Renderable {
    pub fn get_sprite(&self) -> String {
        self.sprite.clone()
    }
}

///This enum will be used in this sandbox simply for determining which team someone is on but in the future
///will be used for determining who in combat is controlled by the player and who by AI
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum ControlType {
    PC,  //for hotseat mode synonymous w/ player 1
    NPC, //for hotseat mode synonymous w/ player 2
}

///Struct that defines who is in a combat encounter, and tracks all the information needed to
///process through them in iniatiative order
pub struct CombatEncounter {
    ///The actual order of the vec is what determines their initiative order, tracked by the character's
    ///Entity ID tag. it's paired with a boolean to track who has completed their turn
    combatants: Vec<(Entity, bool)>,
    ///The total number of turns the combat encounter took is tracked and recorded
    num_turns: i32,
}
impl CombatEncounter {
    pub fn new(combatants: Vec<Entity>) -> Self {
        //
    }
}

// pub struct ActionPoints(i32);
// impl ActionPoints {
// pub fn new()
// }
