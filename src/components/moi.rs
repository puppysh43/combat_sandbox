//this file will contain all of the messages of intent used by the game
use hecs::*;

///Simple MOI component that contains the ID of the entity that is doing a reload action.
pub struct MOIReload(Entity);
impl MOIReload {
    pub fn new(entity: Entity) -> Self {
        Self(entity)
    }
    pub fn entity(&self) -> Entity {
        self.0
    }
}
