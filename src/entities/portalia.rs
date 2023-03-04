use macroquad::{
    //audio,
    color::Color,
    color::colors::*,
    input::*,
    math::*,
    shapes::*,
    text::*,
    texture::*,
    //ui::root_ui,
    //window::*,
};

use crate::entities::*;
use crate::systems::*;

const TILE: f32 = 48.0;

#[derive(Clone, Debug, PartialEq)]
pub struct Portal {
    pub name: String,
    pub chamber: usize,
    pub direction: u8,
    pub tiles: Vec<u8>,
    pub colour: Color,
    pub active: bool,
    pub examine_text: Option<Vec<Vec<String>>>,
    pub examine_image: Option<Texture2D>,
}
impl Portal {
    pub fn new(
        name: String, 
        chamber: usize, 
        direction: u8, 
        tiles: Vec<u8>, 
        colour: Color,
        examine_image: Option<Texture2D>
    ) -> Self {
        Self {
            name,
            chamber,
            direction,
            tiles,
            colour,
            active: false,
            examine_text: None,
            examine_image,
        }
    }
    pub fn draw_examine_text_portal(&self, mut page: usize, glasses: bool) {
        match &self.examine_text {
            None => (),
            Some(pages) => {
                page %= pages.len();
                let left_col = 5.0 * TILE;
                let top_row = TILE;
                draw_rectangle(
                    left_col,
                    top_row,
                    9.0 * TILE,
                    20.0 * pages[page].len() as f32 + 18.0,
                    WHITE,
                );
                for ii in 0..pages[page].len() {
                    if glasses || self.name != "Correspondence".to_string() {
                        draw_text(
                            &pages[page][ii],
                            left_col + 12.0,
                            top_row + (20.0 * (ii + 1) as f32),
                            16.0,
                            DARKGRAY,
                        );
                    } else {
                        for jj in 0..7 {
                            draw_text(
                                &pages[page][ii],
                                left_col + 12.0 + jj as f32,
                                top_row + (20.0 * (ii + 1) as f32),
                                16.0,
                                DARKGRAY,
                            );
                        }
                    }
                }
                let ptext = format!("Page {} of {}", &page + 1, &pages.len());
                draw_text(&ptext, 8.75 * TILE, 0.625 * TILE, 16.0, GRAY);
            },
        }
    }
    pub fn draw_examine_image_portal(&self, theseus: &Theseus, maze: &Amaze) {
        let (col, row, level) = maze.idx_to_xyz(theseus.chamber);
        let lost_col: f32 = col as f32;
        let lost_row: f32 = row as f32; 
        let lost_level: usize = level as usize;
        let left_col = 5.5 * TILE;
        let top_row = 1.5 * TILE;
        let (col, row, _) = maze.idx_to_xyz(maze.start[lost_level]);
        let start_col: f32 = col as f32;
        let start_row: f32 = row as f32;
        let (col, row, _) = maze.idx_to_xyz(maze.end[lost_level]);
        let end_col: f32 = col as f32;
        let end_row: f32 = row as f32;
        match self.examine_image {
            None => (),
            Some(image) => {
                draw_texture(
                    image,
                    left_col,
                    top_row,
                    Color::new(1.0, 1.0, 1.0, 1.0),
                );
            }
        }
        // draw_poly(x: f32, y: f32, sides: u8, radius: f32, rotation: f32, color: Color)
        draw_poly_lines(
            left_col + (start_col + 0.5) * 15.0,
            top_row + (start_row + 0.5) * 15.0,
            3,
            0.3 * 15.0,
            30.0,
            2.0,
            LIGHTGRAY,
        );
        draw_poly_lines(
            left_col + (end_col + 0.5) * 15.0,
            top_row + (end_row + 0.5) * 15.0,
            3,
            0.3 * 15.0,
            -30.0,
            2.0,
            LIGHTGRAY,
        );
        draw_circle_lines(
            left_col + (lost_col + 0.5) * 15.0,
            top_row + (lost_row + 0.5) * 15.0,
            0.3 * 15.0,
            1.0,
            LIGHTGRAY,
        );
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Portalia {
    pub portals: Vec<Portal>,
    pub selected: Option<usize>,
}
impl Portalia {
    pub fn new() -> Self {
        let portals: Vec<Portal> = Vec::new();
        Self {
            portals,
            selected: None,
        }
    }
    pub fn fill_maze(&mut self, maze: &Amaze) { //  alcove center = 104
        for ii in 0..maze.rooms.len() { 
            for jj in 0..6 { // [E, S, W, N, U, D] with 0 = plain wall, 1 = portal, 2 = window, 3 = alcove, tbd
                match maze.rooms[ii][jj] {
                    /*
                    1 => {
                        self.portals.push(Portal::new(
                            "Portal".to_string(),
                            ii,
                            jj.try_into().unwrap(),
                            vec![64, 65, 66, 67, 68, 83, 84, 85, 86, 87, 102, 103, 104, 105, 106, 121, 122, 123, 124, 125, 140, 141, 142, 143, 144],
                            DARKGRAY,
                        ));
                    },
                    */
                    2 => {
                        let (_x, _y, level) = maze.idx_to_xyz(ii);
                        self.portals.push(Portal::new(
                            "Window".to_string(),
                            ii,
                            jj.try_into().unwrap(),
                            vec![84, 85, 86, 103, 104, 105, 122, 123, 124],
                            BLACK,
                            Some(maze.level_sheets[level as usize][1]),
                        ));
                    },
                    /*
                    3 => {
                        self.portals.push(Portal::new(
                            "Alcove".to_string(),
                            ii,
                            jj.try_into().unwrap(),
                            vec![84, 85, 86, 103, 104, 105, 122, 123, 124],
                            DARKGRAY,
                        ));
                    },
                    */
                    _ => (),
                }
            }
        }
    }
    pub fn update(&self) {
        
    }
    pub fn draw_portalia(&self, theseus: &Theseus, maze: &Amaze) {
        let (_x, _y, level) = maze.idx_to_xyz(theseus.chamber);
        let (mx, my) = mouse_position();
        let hover_tile_x: u8 = (mx / TILE).trunc() as u8;
        let hover_tile_y: u8 = (my / TILE).trunc() as u8;
        let hover_tile_index: u8 = (hover_tile_y * 19) + hover_tile_x;
        for ii in 0..self.portals.len() {
            if self.portals[ii].chamber == theseus.chamber
                && self.portals[ii].direction == theseus.direction
            {
                for jj in 0..self.portals[ii].tiles.len() {
                    if self.portals[ii].tiles[jj] == hover_tile_index {
                        draw_text(&self.portals[ii].name, 1.0 * TILE, 10.625 * TILE, 16.0, GRAY);
                    }
                }
                
                if self.portals[ii].name == "Window".to_string() {
                    self.draw_front_window(maze, level as usize);
                }
            }
            if self.selected == Some(ii) {
                for jj in 0..self.portals[ii].tiles.len() {
                    let (col, row) = self.tile_2_cr(self.portals[ii].tiles[jj]);
                    draw_rectangle_lines(
                        col * TILE,
                        row * TILE,
                        TILE,
                        TILE,
                        2.0,
                        GOLD,
                    );
                }
                draw_text(&self.portals[ii].name, 1.0 * TILE, 10.625 * TILE, 16.0, GOLD);
            }
        }
        
    }
    fn draw_front_window(&self, maze: &Amaze, level: usize) {
        draw_rectangle(
            8.0 * TILE,
            4.0 * TILE,
            3.0 * TILE,
            3.0 * TILE,
            Color::new(0.2, 0.2, 0.2, 1.0),
        );
        draw_line( // vanishing line
            8.0 * TILE,
            4.0 * TILE,
            11.0 * TILE,
            7.0 * TILE,
            1.0,
            BLACK,
        );
        draw_line( // vanishing line
            8.0 * TILE,
            7.0 * TILE,
            11.0 * TILE,
            4.0 * TILE,
            1.0,
            BLACK,
        );
        draw_rectangle(
            8.0 * TILE + 9.0,
            4.0 * TILE + 9.0,
            maze.level_sheets[level][0].width(),
            maze.level_sheets[level][0].height(),
            BLACK,
        );
        draw_texture(
            maze.level_sheets[level][0],
            8.0 * TILE + 9.0,
            4.0 * TILE + 9.0,
            Color::new(1.0, 1.0, 1.0, 1.0),
        );
        draw_rectangle_lines( // outer corner
            8.0 * TILE,
            4.0 * TILE,
            3.0 * TILE,
            3.0 * TILE,
            2.0,
            BLACK,
        );
    }
    pub fn tile_2_cr(&self, idx: u8) -> (f32, f32) {
        let row: f32 = (idx as f32 / 19.0).trunc();
        let col: f32 = (idx % 19) as f32;
        (col, row)
    }
    pub fn tile_2_xy(&self, idx: u8) -> (f32, f32) {
        let y: f32 = (idx as f32 / 19.0).trunc() * TILE;
        let x: f32 = (idx % 19) as f32  * TILE;
        (x, y)
    }
}