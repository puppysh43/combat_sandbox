use crate::prelude::*;
use hecs::*;
use macroquad::prelude::*;

pub fn system(state: &mut GameState) {
    //for now just draw a line separating the game tiles from the UI
    //and then list the keybinds for the current control state.
    //do all the preamble math and
    let gamescreen_edge = (MAP_WIDTH * TILE_WIDTH) as f32;
    let divider_width: f32 = 10.0;
    let ui_x_start: f32 = gamescreen_edge + divider_width;
    let ui_top_y_offset: f32 = 5.0;
    let text_height = measure_text("A", None, 20, 1.0).height;
    // println!("text height: {}", text_height);
    draw_line(
        (gamescreen_edge + (divider_width / 2.0)),
        0.0,
        (gamescreen_edge + (divider_width / 2.0)),
        (TILE_HEIGHT * MAP_HEIGHT) as f32,
        divider_width,
        BLACK,
    );
    //then match the control state to see what help needs to be displayed onscreen
}
