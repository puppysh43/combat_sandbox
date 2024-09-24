use crate::prelude::*;
use hecs::*;
use macroquad::prelude::*;

pub fn system(state: &mut GameState) {
    //first render the gamemap

    //then render the entities in proper order (the z ordering equivalent is just that they're drawn in the order of the draw_texture function)
}
fn render_map(state: &mut GameState) {
    clear_background(GRAY);
    //first render the game map
    for y in 0..MAP_HEIGHT {
        for x in 0..MAP_WIDTH {
            let pt = IVec2::new(x, y);
            let idx = map_idx(x, y);
            if state.map.in_bounds(pt) {
                match state.map.tiles[idx] {
                    TileType::Wall => {
                        draw_texture(
                            state.texture_atlas.get("wall").unwrap(),
                            (x * TILE_WIDTH) as f32,
                            (y * TILE_HEIGHT) as f32,
                            WHITE,
                        );
                    }
                    TileType::Floor => {
                        draw_texture(
                            state.texture_atlas.get("floor").unwrap(),
                            (x * TILE_WIDTH) as f32,
                            (y * TILE_HEIGHT) as f32,
                            WHITE,
                        );
                    }
                    TileType::LoadingSquare => {
                        draw_texture(
                            state.texture_atlas.get("cratespot").unwrap(),
                            (x * TILE_WIDTH) as f32,
                            (y * TILE_HEIGHT) as f32,
                            WHITE,
                        );
                    }
                }
            }
        }
    }
}
