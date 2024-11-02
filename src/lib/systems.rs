/*
This module contains all the helper functions for the game systems
*/
use crate::prelude::*;
use hecs::*;
use macroquad::math::*;
use macroquad::rand;

pub fn calculate_initiative(state: &mut GameState) {
    //in the future this will take in arguments either what entities will be included or what mapscreen will have the combat encounter in it or WHATEVER
    //gay bullshit I end up doing
    //HOWEVER right now this simply looks for all entities that have the necessary components to even do an initiative test
}

///Reset the amount of movement points an entity has so they can move more than once ever.
pub fn refresh_mp(state: &mut GameState, active_entity: Entity) {
    //the way I'm doing it rn feels really hacky and lazy but whatever
    for mp in state
        .ecs
        .query_one_mut::<&mut MovementPoints>(active_entity)
    {
        mp.reset();
        println!("resetting movement points.");
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum CheckDifficulty {
    Simple,
    Easy,
    Routine,
    Average,
    Difficult,
    VeryDifficult,
    Formidable,
    Impossible,
}
pub enum BoonOrBane {
    Boon,
    Bane,
}

pub enum RollResult {
    Success(i32),
    Failure(i32),
}
pub fn task_check(
    dm: i32,
    check_difficulty: CheckDifficulty,
    boon_or_bane: Option<BoonOrBane>,
) -> RollResult {
    let difficulty = match check_difficulty {
        CheckDifficulty::Simple => 2,
        CheckDifficulty::Easy => 4,
        CheckDifficulty::Routine => 6,
        CheckDifficulty::Average => 8,
        CheckDifficulty::Difficult => 10,
        CheckDifficulty::VeryDifficult => 12,
        CheckDifficulty::Formidable => 14,
        CheckDifficulty::Impossible => 16,
    };
    if boon_or_bane.is_some() {
        let effect = (roll_3d6(boon_or_bane.unwrap()) + dm) - difficulty;
        if effect >= 0 {
            RollResult::Success(effect)
        } else {
            RollResult::Failure(effect)
        }
    } else {
        let effect = (roll_2d6() + dm) - difficulty;
        if effect >= 0 {
            RollResult::Success(effect)
        } else {
            RollResult::Failure(effect)
        }
    }
}

///randomly generates the result of rolling 2d6s
pub fn roll_2d6() -> i32 {
    rand::srand(rand::rand() as u64);
    rand::gen_range(1, 6) + rand::gen_range(1, 6)
}
pub fn roll_3d6(boon_or_bane: BoonOrBane) -> i32 {
    let raw_roll: Vec<i32> = vec![
        rand::gen_range(1, 6),
        rand::gen_range(1, 6),
        rand::gen_range(1, 6),
    ];
    match boon_or_bane {
        BoonOrBane::Bane => raw_roll.iter().sum::<i32>() - raw_roll.iter().max().unwrap(),
        BoonOrBane::Boon => raw_roll.iter().sum::<i32>() - raw_roll.iter().min().unwrap(),
    }
}

///Implementation of Bresenham's Line Algorithm
pub fn get_line(a: IVec2, b: IVec2) -> Vec<IVec2> {
    let mut points = Vec::<IVec2>::new();
    let (mut x1, mut y1, mut x2, mut y2) = (a.x, a.y, b.x, b.y);
    let is_steep = (y2 - y1).abs() > (x2 - x1).abs();
    if is_steep {
        std::mem::swap(&mut x1, &mut y1);
        std::mem::swap(&mut x2, &mut y2);
    }
    let mut reversed = false;
    if x1 > x2 {
        std::mem::swap(&mut x1, &mut x2);
        std::mem::swap(&mut y1, &mut y2);
        reversed = true;
    }
    let dx = x2 - x1;
    let dy = (y2 - y1).abs();
    let mut err = dx / 2;
    let mut y = y1;
    let ystep: i32;
    if y1 < y2 {
        ystep = 1;
    } else {
        ystep = -1;
    }
    for x in x1..(x2 + 1) {
        if is_steep {
            points.push(IVec2::new(y, x));
        } else {
            points.push(IVec2::new(x, y));
        }
        err -= dy;
        if err < 0 {
            y += ystep;
            err += dx;
        }
    }
    if reversed {
        for i in 0..(points.len() / 2) {
            let end = points.len() - 1;
            points.swap(i, end - i);
        }
    }
    points
}
