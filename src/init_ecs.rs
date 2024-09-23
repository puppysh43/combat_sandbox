use hecs::*;

///RN just have this setup for a basic fight but in the future this can be passed an identifier for deciding which
///scene to launch in the combat sandbox.
pub fn init_ecs() -> World {
    //first add the decorative entities (furniture, cover, whatever)
    //then spawn in the various characters in the combat scene
    //then define the combat scene
    //that should be everything??
    World::new()
}
