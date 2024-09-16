use macroquad::prelude::*;

#[derive(Clone, Debug, PartialEq)]
pub enum CombatActionType {
    None,
    RangedAttack,
    MeleeAttack,
    Leadership,
    Aiming,
    ChangingStance,
    Drawing,
    Reloading,
    Movement,
    Grapple,
    Interact,
    UseItem(UseItemState),
    Look,
    PickUp,
}

#[derive(Clone, Debug, PartialEq)]
pub enum UseItemState {
    Selecting,
    Using,
}
