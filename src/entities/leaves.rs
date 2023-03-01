use macroquad::{
    //audio,
    //color::Color,
    color::colors::*,
    //input::*,
    //math::*,
    shapes::*,
    text::*,
    //texture::*,
    //ui::root_ui,
    //window::*,
};

use crate::entities::*;
use crate::systems::*;

const TILE: f32 = 48.0;

#[derive(Clone, Debug, PartialEq)]
pub struct Leaves {
    pub name: String,
    pub leaf: Vec<String>,
    pub page: usize,
}
impl Leaves {
    pub fn new(name: String, leaf: Vec<String>) -> Self {
        Self {
            name,
            leaf,
            page: 0,
        }
    }
    pub fn draw_leaves(&self) {
        let left_col = 5.0 * TILE;
        let top_row = TILE;
        draw_rectangle(
            left_col,
            top_row,
            9.0 * TILE,
            9.0 * TILE,
            WHITE,
        );
        draw_text(
            &self.leaf[self.page],
            left_col + 4.0,
            top_row + 20.0,
            16.0,
            DARKGRAY,
        );
    }
}