use crate::combat_action_type::*;
use crate::init_ecs::*;
use crate::prelude::*;
use crate::sound_atlas::*;
use crate::texture_atlas::*;
use hecs::*;
use macroquad::prelude::*;

pub struct GameState {
    pub texture_atlas: TextureAtlas,
    pub sound_atlas: SoundAtlas,
    pub ecs: World,
    pub turn_state: TurnState,
    pub control_state: CombatActionType,
    //map: Map,//temporary just for testing combat.
    //player: Entity //will be needed in full game
    pub log: Vec<String>,
    pub number_turns: i32,
    pub quitting: bool,
    //ui_state: UiState,
}

impl GameState {
    pub async fn default() -> Self {
        let log: Vec<String> = Vec::new();
        Self {
            texture_atlas: crate::texture_atlas::make().await,
            sound_atlas: crate::sound_atlas::make().await,
            ecs: crate::init_ecs::init_ecs(),
            turn_state: TurnState::PlayerOne,
            control_state: CombatActionType::None,
            log,
            number_turns: 0,
            quitting: false,
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum TurnState {
    PlayerOne,
    PlayerTwo,
}
