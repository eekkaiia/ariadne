use macroquad::{
    //audio,
    //audio::Sound,
    color::Color,
    color::colors::*,
    input::*,
    math::*,
    shapes::*,
    text::*,
    texture::*,
    ui::root_ui,
    window::*,
};

use crate::systems::*;
use crate::entities::*;

const TILE: f32 = 48.0;
//const NUMBER_OF_VIEWS: usize = 4;

pub struct Stage {
    pub width: f32,
    pub depth: f32,
    pub tile_size: f32,
    pub perspective: usize,
}
impl Stage {
    pub fn new(width: f32, depth: f32) -> Self {
        Self {
            width,
            depth,
            tile_size: 48.0,
            perspective: 0,
        }
    }
    /// update_stage() draw perspective
    pub fn update(
        &self, 
        info: &mut Info, 
        control: &mut Control, 
        maze: &Amaze, 
        theseus: &Theseus, 
        paraphernalia: &Paraphernalia,
        portalia: &Portalia,
    ) {
        clear_background(info.background);
        // update macroquad parameters
        info.update();
        // display an icon in top left corner that toggles panel visibilities
        if root_ui().button(None, "<>") {
            info.visible = !info.visible;
        }
        // if visible draw info panel
        if info.visible {
            draw_rectangle(25.0, 0.0, 190.0, 19.0, info.background);
            draw_text(&info.context, 27.0, 13.0, 16.0, BLUE);
            info.draw_panel();
        }
        match self.perspective {
            0 => {
                self.draw_ortho(theseus, maze);
                portalia.draw_portalia(theseus, maze);
                paraphernalia.draw_paraphernalia(theseus);
                control.draw_control(0);
                match paraphernalia.something_illuminated {
                    0..=5 => self.draw_atmosphere(Color::new(0.0, 0.0 , 0.0, 0.97)),
                    6..=10 => self.draw_atmosphere(Color::new(0.0, 0.0 , 0.0, 0.5)),
                    11..=20 => self.draw_atmosphere(Color::new(1.0, 0.6 , 0.0, 0.1)),
                    _ => self.draw_atmosphere(Color::new(0.0, 0.0 , 0.0, 1.0)),
                }
            },
            1 => {
                self.draw_planview(maze, theseus);
                control.draw_control(1);
            },
            2 => {
                portalia.portals[control.examine_portalia].draw_examine_image_portal(theseus, maze);
                //paraphernalia.draw_paraphernalia(theseus);
                control.draw_control(2);
            },
            3 => {
                let mut glasses: bool = false;
                for ii in 0..paraphernalia.parapherna.len() {
                    if paraphernalia.parapherna[ii].name == "Spectacles".to_string() || paraphernalia.parapherna[ii].name == "Violet Glasses".to_string() {
                        if paraphernalia.parapherna[ii].disposition == Disposition::OnHead { glasses = true; };
                    }
                }
                paraphernalia.parapherna[control.examine_paraphernalia].draw_examine_text_parapherna(control.page, glasses);
                control.draw_control(3);
            },
            _ => eprintln!("!!!Unexpected view command: {}", &self.perspective),
        }
        draw_rectangle_lines(0.0, 0.0, self.width, self.depth, 2.0, Color::new(0.5, 0.5, 0.5, 0.5));
        let (mx, my) = mouse_position();
        let hover_tile_x: f32 = (mx / TILE).trunc();
        let hover_tile_y: f32 = (my / TILE).trunc();
        draw_rectangle_lines(hover_tile_x * TILE, hover_tile_y * TILE, TILE, TILE, 2.0, Color::new(0.5, 0.5, 0.5, 0.5));
    }
    /// draw guide lines
    fn guide_lines(&self) {
        // draw tiles
        for jj in 0..11 {
            for ii in 0..19 {
                draw_rectangle_lines(ii as f32 * TILE, jj as f32 * TILE, TILE, TILE, 1.0, DARKGRAY);
            }
        }
        // draw wall lines
        draw_line(5.0 * TILE, 1.0 * TILE, 14.0 * TILE, 10.0 * TILE, 0.5, GRAY);
        draw_line(5.0 * TILE, 10.0 * TILE, 14.0 * TILE, 1.0 * TILE, 0.5, GRAY);
        draw_rectangle_lines(5.0 * TILE, 1.0 * TILE, 9.0 * TILE, 9.0 * TILE, 2.0, GRAY);
        draw_rectangle_lines(7.0 * TILE, 3.0 * TILE, 5.0 * TILE, 5.0 * TILE, 2.0, GRAY);
    }
    /// draw_planview() displays aerial maze
    fn draw_planview(&self, maze: &Amaze, theseus: &Theseus) {
        let (col, row, level) = maze.idx_to_xyz(theseus.chamber);
        let lost_col: f32 = col as f32;
        let lost_row: f32 = row as f32; 
        let lost_level: usize = level as usize;
        let left_col =(self.width - (maze.texture_cell_sizes[2] * maze.width) as f32) / 2.0;
        let top_row = (self.depth - (maze.texture_cell_sizes[2] * maze.depth) as f32) / 2.0;
        let (col, row, _) = maze.idx_to_xyz(maze.start[lost_level]);
        let start_col: f32 = col as f32;
        let start_row: f32 = row as f32;
        let (col, row, _) = maze.idx_to_xyz(maze.end[lost_level]);
        let end_col: f32 = col as f32;
        let end_row: f32 = row as f32;
    
        draw_texture(
            maze.level_sheets[lost_level][2],
            left_col,
            top_row,
            Color::new(1.0, 1.0, 1.0, 1.0),
        );
        draw_circle_lines(
            left_col + (start_col + 0.5) * maze.texture_cell_sizes[2] as f32,
            top_row + (start_row + 0.5) * maze.texture_cell_sizes[2] as f32,
            0.4 * maze.texture_cell_sizes[2] as f32,
            2.0,
            RED,
        );
        draw_circle_lines(
            left_col + (end_col + 0.5) * maze.texture_cell_sizes[2] as f32,
            top_row + (end_row + 0.5) * maze.texture_cell_sizes[2] as f32,
            0.4 * maze.texture_cell_sizes[2] as f32,
            2.0,
            GREEN,
        );
        draw_circle(
            left_col + (lost_col + 0.5) * maze.texture_cell_sizes[2] as f32,
            top_row + (lost_row + 0.5) * maze.texture_cell_sizes[2] as f32,
            0.3 * maze.texture_cell_sizes[2] as f32,
            BLUE,
        );
        for ii in (maze.width * maze.depth * lost_level)..(maze.width * maze.depth * (lost_level + 1)) {
            if maze.visited[ii] {
                let (col, row, _) = maze.idx_to_xyz(ii);
                let visit_col: f32 = col as f32;
                let visit_row: f32 = row as f32;
                draw_circle(
                    left_col + (visit_col + 0.4) * maze.texture_cell_sizes[2] as f32,
                    top_row + (visit_row + 0.4) * maze.texture_cell_sizes[2] as f32,
                    0.1 * maze.texture_cell_sizes[2] as f32,
                    GOLD,
                );
            }
        }
        for jj in 0..maze.solutions[lost_level].len() {
            let (col, row) = maze.idx_to_xy(maze.solutions[lost_level][jj]);
            let solution_col: f32 = col as f32;
            let solution_row: f32 = row as f32;
            draw_circle(
                left_col + (solution_col + 0.6) * maze.texture_cell_sizes[2] as f32,
                top_row + (solution_row + 0.6) * maze.texture_cell_sizes[2] as f32,
                0.1 * maze.texture_cell_sizes[2] as f32,
                GREEN,
            );
        }
    }
    /// draw_ortho() displays maze from Theseus' perspective
    fn draw_ortho(&self, theseus: &Theseus, maze: &Amaze) {
        let (_, _, level) = maze.idx_to_xyz(theseus.chamber);
        let mut colour: Color = DARKGRAY;
        if theseus.chamber == maze.end[level as usize] {
            colour = GREEN;
        }
        if theseus.chamber == maze.start[level as usize] {
            colour = RED;
        }
        let mut left: usize = 0;
        let mut right: usize = 0;
        match theseus.direction {
            0 => {
                left = 3;
                right = 1;
            },
            1 => {
                left = 0;
                right = 2;
            },
            2 => {
                left = 1;
                right = 3;
            },
            3 => {
                left = 2;
                right = 0;
            },
            _ => eprintln!("!!!Unexpected direction: {:?}", &theseus.direction),
        }
        self.draw_front_wall(colour);
        if theseus.chamber == maze.start[level as usize] {
            self.draw_up_portal();
        }
        if theseus.chamber == maze.end[level as usize] {
            self.draw_down_portal();
        }
        match maze.rooms[theseus.chamber][theseus.direction as usize] {
            0 => (),
            1 => {
                self.draw_front_portal();
                for ii in 0..maze.thread.len() {
                    match theseus.direction {
                        0 => {
                            if maze.thread[ii] == theseus.chamber + 1 {
                                self.draw_ariadne();
                            }
                        },
                        1 => {
                            if maze.thread[ii] == theseus.chamber + maze.width {
                                self.draw_ariadne();
                            }
                        },
                        2 => {
                            if maze.thread[ii] == theseus.chamber - 1 {
                                self.draw_ariadne();
                            }
                        },
                        3 => {
                            if maze.thread[ii] == theseus.chamber - maze.width {
                                self.draw_ariadne();
                            }
                        },
                        _ => eprintln!("!!!Unexpected direction: {:?}", &theseus.direction),
                    }
                }
            },
            2 => self.draw_front_alcove(),
            3 => self.draw_front_alcove(),
            _ => eprintln!("!!!Unexpected room type: {:?}", &maze.rooms[theseus.chamber][theseus.direction as usize]),
        }
        match maze.rooms[theseus.chamber][left] {
            0 => (), // draw_left_wall - not necessary yet 
            1 => {
                self.draw_left_portal();
                for ii in 0..maze.thread.len() {
                    match theseus.direction {
                        0 => {
                            if maze.thread[ii] == theseus.chamber - maze.width {
                                self.draw_left_ariadne();
                            }
                        },
                        1 => {
                            if maze.thread[ii] == theseus.chamber + 1 {
                                self.draw_left_ariadne();
                            }
                        },
                        2 => {
                            if maze.thread[ii] == theseus.chamber + maze.width {
                                self.draw_left_ariadne();
                            }
                        },
                        3 => {
                            if maze.thread[ii] == theseus.chamber - 1 {
                                self.draw_left_ariadne();
                            }
                        },
                        _ => eprintln!("!!!Unexpected direction: {:?}", &theseus.direction),
                    }
                }
            },
            2 => self.draw_left_window(),
            3 => self.draw_left_window(), // same as window
            _ => eprintln!("!!!Unexpected room type: {:?}", &maze.rooms[theseus.chamber][left]),
        }
        match maze.rooms[theseus.chamber][right] {
            0 => (), //draw_right_wall(), not necessary, yet
            1 => {
                self.draw_right_portal();
                for ii in 0..maze.thread.len() {
                    match theseus.direction {
                        0 => {
                            if maze.thread[ii] == theseus.chamber + maze.width {
                                self.draw_right_ariadne();
                            }
                        },
                        1 => {
                            if maze.thread[ii] == theseus.chamber - 1 {
                                self.draw_right_ariadne();
                            }
                        },
                        2 => {
                            if maze.thread[ii] == theseus.chamber - maze.width {
                                self.draw_right_ariadne();
                            }
                        },
                        3 => {
                            if maze.thread[ii] == theseus.chamber + 1 {
                                self.draw_right_ariadne();
                            }
                        },
                        _ => eprintln!("!!!Unexpected direction: {:?}", &theseus.direction),
                    }
                }
            },
            2 => self.draw_right_window(),
            3 => self.draw_right_window(), // same as window
            _ => eprintln!("!!!Unexpected room type: {:?}", &maze.rooms[theseus.chamber][right]),
        }
    }
    fn draw_atmosphere(&self, colour: Color) {
        draw_rectangle(
            5.0 * TILE,
            1.0 * TILE,
            9.0 * TILE,
            9.0 * TILE,
            colour,
        );
    }
    fn draw_ariadne(&self) {
        draw_line(
            9.5 * TILE,
            7.5 * TILE,
            9.0 * TILE + 30.0,
            8.0 * TILE,
            1.5,
            GOLD,
        );
        draw_line(
            9.0 * TILE + 30.0,
            8.0 * TILE,
            9.0 * TILE + 18.0,
            9.0 * TILE,
            1.7,
            YELLOW,
        );
        draw_line(
            9.0 * TILE + 18.0,
            9.0 * TILE,
            9.5 * TILE,
            10.0 * TILE,
            2.0,
            YELLOW,
        );
    }
    fn draw_left_ariadne(&self) {
        draw_line(
            5.0 * TILE,
            8.0 * TILE,
            5.25 * TILE,
            8.25 * TILE,
            1.0,
            GOLD,
        );
        draw_line(
            5.25 * TILE,
            8.25 * TILE,
            5.5 * TILE,
            9.75 * TILE,
            1.0,
            YELLOW,
        );
        draw_line(
            5.5 * TILE,
            9.75 * TILE,
            7.5 * TILE,
            10.0 * TILE,
            1.0,
            YELLOW,
        );
    }
    fn draw_right_ariadne(&self) {
        draw_line(
            14.0 * TILE,
            8.0 * TILE,
            13.75 * TILE,
            8.25 * TILE,
            1.0,
            GOLD,
        );
        draw_line(
            13.75 * TILE,
            8.25 * TILE,
            13.5 * TILE,
            9.75 * TILE,
            1.0,
            YELLOW,
        );
        draw_line(
            13.5 * TILE,
            9.75 * TILE,
            11.5 * TILE,
            10.0 * TILE,
            1.0,
            YELLOW,
        );
    }
    fn draw_front_wall(&self, colour: Color) {
        draw_rectangle(
            5.0 * TILE,
            1.0 * TILE,
            9.0 * TILE,
            9.0 * TILE,
            colour,
        );
        draw_rectangle_lines(
            5.0 * TILE,
            1.0 * TILE,
            9.0 * TILE,
            9.0 * TILE,
            2.0,
            DARKGRAY,
        );
        self.converging_lines();
    }
    fn draw_front_portal(&self) {
        draw_poly(
            9.5 * TILE,     // x: f32,
            5.5 * TILE,     // y: f32,
            8,              // sides: u8,
            2.75 * TILE,    // radius: f32,
            22.5,           // rotation: f32,
            Color::new(0.2, 0.2, 0.2, 1.0),
        );
        draw_poly(
            9.5 * TILE,     // x: f32,
            5.5 * TILE,     // y: f32,
            8,              // sides: u8,
            2.25 * TILE,    // radius: f32,
            22.5,           // rotation: f32,
            BLACK,          // color: Color
        );
        draw_line(
            406.0,
            142.0,
            506.0,
            384.0,
            1.0,
            BLACK,
        );
        draw_line(
            506.0,
            142.0,
            406.0,
            384.0,
            1.0,
            BLACK,
        );
        draw_line(
            334.0,
            214.0,
            578.0,
            314.0,
            1.0,
            BLACK,
        );
        draw_line(
            334.0,
            316.0,
            578.0,
            214.0,
            1.0,
            BLACK,
        );
        draw_poly_lines(
            9.5 * TILE,     // x: f32,
            5.5 * TILE,     // y: f32,
            8,              // sides: u8,
            2.75 * TILE,    // radius: f32,
            22.5,           // rotation: f32,
            1.0,            // thickness: f32,
            BLACK,
        );
        
    }
    fn draw_front_alcove(&self) {
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
            8.125 * TILE,
            4.125 * TILE,
            2.75 * TILE,
            2.75 * TILE,
            Color::new(0.1, 0.1, 0.1, 1.0),
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
    fn draw_left_portal(&self) {
        draw_rectangle(
            5.0 * TILE,
            4.0 * TILE,
            1.0 * TILE,
            3.0 * TILE,
            Color::new(0.2, 0.2, 0.2, 1.0),
        );
        draw_rectangle(
            5.0 * TILE,
            3.5 * TILE,
            0.25 * TILE,
            4.0 * TILE,
            BLACK,
        );
        let v1: Vec2 = Vec2::new(5.0 * TILE, 4.0 * TILE);
        let v2: Vec2 = Vec2::new(6.0 * TILE, 4.0 * TILE);
        let v3: Vec2 = Vec2::new(5.0 * TILE, 2.5 * TILE);
        draw_triangle(v1, v2, v3, Color::new(0.2, 0.2, 0.2, 1.0));
        let v1: Vec2 = Vec2::new(5.0 * TILE, 4.0 * TILE);
        let v2: Vec2 = Vec2::new(5.25 * TILE, 4.0 * TILE);
        let v3: Vec2 = Vec2::new(5.0 * TILE, (3.0 * TILE) + 32.0);
        draw_triangle(v1, v2, v3, BLACK);
        let v1: Vec2 = Vec2::new(5.0 * TILE, 7.0 * TILE);
        let v2: Vec2 = Vec2::new(6.0 * TILE, 7.0 * TILE);
        let v3: Vec2 = Vec2::new(5.0 * TILE, 8.5 * TILE);
        draw_triangle(v1, v2, v3, Color::new(0.2, 0.2, 0.2, 1.0));
        let v1: Vec2 = Vec2::new(5.0 * TILE, 7.0 * TILE);
        let v2: Vec2 = Vec2::new(5.25 * TILE, 7.0 * TILE);
        let v3: Vec2 = Vec2::new(5.0 * TILE, (7.0 * TILE) + 12.0);
        draw_triangle(v1, v2, v3, BLACK);
        draw_line(
            6.0 * TILE,
            4.0 * TILE,
            5.25 * TILE,
            4.0 * TILE,
            1.0,
            BLACK,
        );
        draw_line(
            6.0 * TILE,
            7.0 * TILE,
            5.25 * TILE,
            7.0 * TILE,
            1.0,
            BLACK,
        );
        draw_line(
            6.0 * TILE,
            4.0 * TILE,
            6.0 * TILE,
            7.0 * TILE,
            1.0,
            BLACK,
        );
        draw_line(
            6.0 * TILE,
            7.0 * TILE,
            5.0 * TILE,
            8.5 * TILE,
            1.0,
            BLACK,
        );
        draw_line(
            6.0 * TILE,
            4.0 * TILE,
            5.0 * TILE,
            2.5 * TILE,
            1.0,
            BLACK,
        );
    }
    fn draw_right_portal(&self) {
        draw_rectangle(
            13.0 * TILE,
            4.0 * TILE,
            1.0 * TILE,
            3.0 * TILE,
            Color::new(0.2, 0.2, 0.2, 1.0),
        );
        draw_rectangle(
            13.75 * TILE,
            3.5 * TILE,
            0.25 * TILE,
            4.0 * TILE,
            BLACK,
        );
        let v1: Vec2 = Vec2::new(13.0 * TILE, 4.0 * TILE);
        let v2: Vec2 = Vec2::new(14.0 * TILE, 4.0 * TILE);
        let v3: Vec2 = Vec2::new(14.0 * TILE, 2.5 * TILE);
        draw_triangle(v1, v2, v3, Color::new(0.2, 0.2, 0.2, 1.0));
        let v1: Vec2 = Vec2::new(13.75 * TILE, 4.0 * TILE);
        let v2: Vec2 = Vec2::new(14.0 * TILE, 4.0 * TILE);
        let v3: Vec2 = Vec2::new(14.0 * TILE, (3.0 * TILE) + 32.0);
        draw_triangle(v1, v2, v3, BLACK);
        let v1: Vec2 = Vec2::new(13.0 * TILE, 7.0 * TILE);
        let v2: Vec2 = Vec2::new(14.0 * TILE, 7.0 * TILE);
        let v3: Vec2 = Vec2::new(14.0 * TILE, 8.5 * TILE);
        draw_triangle(v1, v2, v3, Color::new(0.2, 0.2, 0.2, 1.0));
        let v1: Vec2 = Vec2::new(13.75 * TILE, 7.0 * TILE);
        let v2: Vec2 = Vec2::new(14.0 * TILE, 7.0 * TILE);
        let v3: Vec2 = Vec2::new(14.0 * TILE, (7.0 * TILE) + 12.0);
        draw_triangle(v1, v2, v3, BLACK);
        draw_line(
            13.0 * TILE,
            4.0 * TILE,
            13.75 * TILE,
            4.0 * TILE,
            1.0,
            BLACK,
        );
        draw_line(
            13.0 * TILE,
            7.0 * TILE,
            13.75 * TILE,
            7.0 * TILE,
            1.0,
            BLACK,
        );
        draw_line(
            13.0 * TILE,
            4.0 * TILE,
            13.0 * TILE,
            7.0 * TILE,
            1.0,
            BLACK,
        );
        draw_line(
            13.0 * TILE,
            7.0 * TILE,
            14.0 * TILE,
            8.5 * TILE,
            1.0,
            BLACK,
        );
        draw_line(
            13.0 * TILE,
            4.0 * TILE,
            14.0 * TILE,
            2.5 * TILE,
            1.0,
            BLACK,
        );
    }
    fn draw_up_portal(&self) {
        draw_line(
            8.0 * TILE,
            2.0 * TILE,
            11.0 * TILE,
            2.0 * TILE,
            1.0,
            BLACK,
        );
        draw_line(
            6.5 * TILE,
            1.0 * TILE,
            8.0 * TILE,
            2.0 * TILE,
            1.0,
            BLACK,
        );
        draw_line(
            11.0 * TILE,
            2.0 * TILE,
            12.5 * TILE,
            1.0 * TILE,
            1.0,
            BLACK,
        );
    }
    fn draw_down_portal(&self) {
        draw_line(
            8.0 * TILE,
            9.0 * TILE,
            11.0 * TILE,
            9.0 * TILE,
            1.0,
            BLACK,
        );
        draw_line(
            6.5 * TILE,
            10.0 * TILE,
            8.0 * TILE,
            9.0 * TILE,
            1.0,
            BLACK,
        );
        draw_line(
            11.0 * TILE,
            9.0 * TILE,
            12.5 * TILE,
            10.0 * TILE,
            1.0,
            BLACK,
        );
    }
    fn draw_left_window(&self) {
        draw_rectangle(
            5.0 * TILE,
            3.5 * TILE,
            0.25 * TILE,
            4.0 * TILE,
            Color::new(0.2, 0.2, 0.2, 1.0),
        );
        let v1: Vec2 = Vec2::new(5.0 * TILE, 3.5 * TILE);
        let v2: Vec2 = Vec2::new(5.25 * TILE, 3.5 * TILE);
        let v3: Vec2 = Vec2::new(5.0 * TILE, 3.25 * TILE);
        draw_triangle(v1, v2, v3, Color::new(0.2, 0.2, 0.2, 1.0));
        let v1: Vec2 = Vec2::new(5.0 * TILE, 7.5 * TILE);
        let v2: Vec2 = Vec2::new(5.25 * TILE, 7.5 * TILE);
        let v3: Vec2 = Vec2::new(5.0 * TILE, 7.75 * TILE);
        draw_triangle(v1, v2, v3, Color::new(0.2, 0.2, 0.2, 1.0));
        draw_line(
            5.0 * TILE,
            3.25 * TILE,
            5.25 * TILE,
            3.5 * TILE,
            1.0,
            BLACK,
        );
        draw_line(
            5.0 * TILE,
            7.75 * TILE,
            5.25 * TILE,
            7.5 * TILE,
            1.0,
            BLACK,
        );
        draw_rectangle_lines(
            5.0 * TILE,
            3.5 * TILE,
            0.25 * TILE,
            4.0 * TILE,
            1.5,
            BLACK,
        );
    }
    fn draw_right_window(&self) {
        draw_rectangle(
            13.75 * TILE,
            3.5 * TILE,
            0.25 * TILE,
            4.0 * TILE,
            Color::new(0.2, 0.2, 0.2, 1.0),
        );
        let v1: Vec2 = Vec2::new(13.75 * TILE, 3.5 * TILE);
        let v2: Vec2 = Vec2::new(14.0 * TILE, 3.5 * TILE);
        let v3: Vec2 = Vec2::new(14.0 * TILE, 3.25 * TILE);
        draw_triangle(v1, v2, v3, Color::new(0.2, 0.2, 0.2, 1.0));
        let v1: Vec2 = Vec2::new(13.75 * TILE, 7.5 * TILE);
        let v2: Vec2 = Vec2::new(14.0 * TILE, 7.5 * TILE);
        let v3: Vec2 = Vec2::new(14.0 * TILE, 7.75 * TILE);
        draw_triangle(v1, v2, v3, Color::new(0.2, 0.2, 0.2, 1.0));
        draw_line(
            13.75 * TILE,
            3.5 * TILE,
            14.0 * TILE,
            3.25 * TILE,
            1.0,
            BLACK,
        );
        draw_line(
            13.75 * TILE,
            7.5 * TILE,
            14.0 * TILE,
            7.75 * TILE,
            1.0,
            BLACK,
        );
        draw_rectangle_lines(
            13.75 * TILE,
            3.5 * TILE,
            0.25 * TILE,
            4.0 * TILE,
            1.5,
            BLACK,
        );
    }
    fn converging_lines(&self) {
        draw_rectangle_lines(
            6.5 * TILE,
            2.5 * TILE,
            6.0 * TILE,
            6.0 * TILE,
            1.5,
            BLACK,
        );
        draw_line(
            5.0 * TILE,
            1.0 * TILE,
            6.5 * TILE,
            2.5 * TILE,
            1.0,
            BLACK,
        );
        draw_line(
            12.5 * TILE,
            2.5 * TILE,
            14.0 * TILE,
            1.0 * TILE,
            1.0,
            BLACK,
        );
        draw_line(
            5.0 * TILE,
            10.0 * TILE,
            6.5 * TILE,
            8.5 * TILE,
            1.0,
            BLACK,
        );
        draw_line(
            12.5 * TILE,
            8.5 * TILE,
            14.0 * TILE,
            10.0 * TILE,
            1.0,
            BLACK,
        );
    }
    pub fn tile_2_cr(&self, idx: u8) -> (f32, f32) {
        let row: f32 = (idx as f32 / 19.0).trunc();
        let col: f32 = (idx % 19) as f32;
        (col, row)
    }
    pub fn tile_2_xy(&self, idx: u8) -> (f32, f32) {
        let y: f32 = (idx as f32 / 19.0).trunc() * self.tile_size;
        let x: f32 = (idx % 19) as f32  * self.tile_size;
        (x, y)
    }
}