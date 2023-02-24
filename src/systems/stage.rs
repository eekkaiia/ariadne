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
const NUMBER_OF_VIEWS: usize = 2;

pub struct Stage {
    pub width: f32,
    pub depth: f32,
    pub tile_size: f32,
    pub perspective: usize,
    pub user_decided_to_exit: bool,
}
impl Stage {
    pub fn new(width: f32, depth: f32) -> Self {
        Self {
            width,
            depth,
            tile_size: 48.0,
            perspective: 0,
            user_decided_to_exit: false,
        }
    }
    /// interface() gets input from keyboard & mouse
    pub fn interface(&mut self, maze: &mut Amaze, theseus: &mut Theseus, paraphernalia: &mut Paraphernalia) -> (bool, usize) {
        let mut play_sound: bool = false;
        let mut sound_index: usize = 0;
        match self.perspective {
            0 => { // orthoview
                // keyboard input
                match get_last_key_pressed() {
                    Some(KeyCode::Down) => 
                    /*
                    
                    */
                    (),
                    Some(KeyCode::Up) => (),
                    Some(KeyCode::Left) => {
                        match theseus.direction {
                            3 => theseus.direction = 2,
                            0 => theseus.direction = 3,
                            1 => theseus.direction = 0,
                            2 => theseus.direction = 1,
                            _ => eprintln!("!!!Unexpected direction char: {:?}", &theseus.direction),
                        }
                    },
                    Some(KeyCode::Right) => {
                        match theseus.direction {
                            3 => theseus.direction = 0,
                            0 => theseus.direction = 1,
                            1 => theseus.direction = 2,
                            2 => theseus.direction = 3,
                            _ => eprintln!("!!!Unexpected direction char: {:?}", &theseus.direction),
                        }
                    },
                    Some(KeyCode::Space) => {
                        (play_sound, sound_index) = maze.theseus_move_forward(theseus);
                    },
                    Some(KeyCode::P) => {
                        self.perspective += 1;
                        self.perspective %= NUMBER_OF_VIEWS;
                    },
                    Some(KeyCode::Q) => self.user_decided_to_exit = true,
                    Some(KeyCode::S) => { // 'secret' key to go to next level
                        for mm in 0..maze.end.len() - 1 {
                            if theseus.chamber == maze.end[mm] {
                                theseus.chamber = maze.start[mm + 1];
                            }
                        }
                    },
                    Some(KeyCode::T) => { // 'secret' key to go back to previous level
                        for mm in 1..maze.start.len() {
                            if theseus.chamber == maze.start[mm] {
                                theseus.chamber = maze.end[mm - 1];
                            }
                        }
                    },
                    _ => (),
                }
                // mouse input
                if is_mouse_button_pressed(MouseButton::Right) || is_mouse_button_pressed(MouseButton::Left) {
                    let (mx, my) = mouse_position();
                    let select_tile_x: u8 = (mx / TILE).trunc() as u8;
                    let select_tile_y: u8 = (my / TILE).trunc() as u8;
                    let tile_index: u8 = (select_tile_y * 19) + select_tile_x;
                    // rewritten match
                    if paraphernalia.selected == None {
                        for ii in 0..paraphernalia.parapherna.len() {
                            if paraphernalia.parapherna[ii].tile == tile_index { paraphernalia.selected = Some(ii); };
                        }
                        if paraphernalia.selected == None {
                            match select_tile_x {
                                0..=3 => (),
                                4..=6 => {
                                    match theseus.direction {
                                        3 => theseus.direction = 2,
                                        0 => theseus.direction = 3,
                                        1 => theseus.direction = 0,
                                        2 => theseus.direction = 1,
                                        _ => eprintln!("!!!Unexpected direction char: {:?}", &theseus.direction),
                                    }
                                },
                                7..=11 => {
                                    match select_tile_y {
                                        0..=2 => (),
                                        3..=7 => (play_sound, sound_index) = maze.theseus_move_forward(theseus),
                                        8..=10 => (),
                                        _ => eprintln!("!!!Unexpected tile y coordinate: {:?}", &select_tile_y),
                                    }
                                },
                                12..=14 => {
                                    match theseus.direction {
                                        3 => theseus.direction = 0,
                                        0 => theseus.direction = 1,
                                        1 => theseus.direction = 2,
                                        2 => theseus.direction = 3,
                                        _ => eprintln!("!!!Unexpected direction char: {:?}", &theseus.direction),
                                    }
                                },
                                15..=18 => (),
                                _ => eprintln!("!!!Unexpected tile x coordinate: {:?}", &select_tile_x),
                            }
                        }
                    } else {
                        let index: usize = paraphernalia.selected.expect("!!!Unexpected None");
                        match tile_index {
                            21 => {
                                for ii in 0..paraphernalia.parapherna.len() {
                                    if paraphernalia.parapherna[ii].disposition == Disposition::OnHead {
                                        paraphernalia.parapherna[ii].disposition = Disposition::InPack;
                                    }
                                }
                                paraphernalia.parapherna[index].disposition = Disposition::OnHead;
                                paraphernalia.parapherna[index].tile = 21;
                                paraphernalia.selected = None;
                            },
                            24..=32 => {
                                paraphernalia.parapherna[index].disposition = Disposition::InMaze;
                                paraphernalia.selected = None;
                            },
                            39 => {
                                for ii in 0..paraphernalia.parapherna.len() {
                                    if paraphernalia.parapherna[ii].disposition == Disposition::InLeftHand {
                                        paraphernalia.parapherna[ii].disposition = Disposition::InPack;
                                    }
                                }
                                paraphernalia.parapherna[index].disposition = Disposition::InLeftHand;
                                paraphernalia.parapherna[index].tile = 39;
                                paraphernalia.selected = None;
                            },
                            40 => {
                                for ii in 0..paraphernalia.parapherna.len() {
                                    if paraphernalia.parapherna[ii].disposition == Disposition::OnNeck {
                                        paraphernalia.parapherna[ii].disposition = Disposition::InPack;
                                    }
                                }
                                paraphernalia.parapherna[index].disposition = Disposition::OnNeck;
                                paraphernalia.parapherna[index].tile = 40;
                                paraphernalia.selected = None;
                            },
                            41 => {
                                for ii in 0..paraphernalia.parapherna.len() {
                                    if paraphernalia.parapherna[ii].disposition == Disposition::InRightHand {
                                        paraphernalia.parapherna[ii].disposition = Disposition::InPack;
                                    }
                                }
                                paraphernalia.parapherna[index].disposition = Disposition::InRightHand;
                                paraphernalia.parapherna[index].tile = 41;
                                paraphernalia.selected = None;
                            },
                            43..=51 => {
                                paraphernalia.parapherna[index].disposition = Disposition::InMaze;
                                paraphernalia.selected = None;
                            },
                            62..=70 => {
                                paraphernalia.parapherna[index].disposition = Disposition::InMaze;
                                paraphernalia.selected = None;
                            },
                            77..=79 => {
                                paraphernalia.parapherna[index].disposition = Disposition::InPack;
                                paraphernalia.selected = None;
                            },
                            81..=89 => {
                                paraphernalia.parapherna[index].disposition = Disposition::InMaze;
                                paraphernalia.selected = None;
                            },
                            96..=98 => {
                                paraphernalia.parapherna[index].disposition = Disposition::InPack;
                                paraphernalia.selected = None;
                            },
                            100..=108 => {
                                paraphernalia.parapherna[index].disposition = Disposition::InMaze;
                                paraphernalia.selected = None;
                            },
                            115..=117 => {
                                paraphernalia.parapherna[index].disposition = Disposition::InPack;
                                paraphernalia.selected = None;
                            },
                            119..=127 => {
                                paraphernalia.parapherna[index].disposition = Disposition::InMaze;
                                paraphernalia.selected = None;
                            },
                            134..=136 => {
                                paraphernalia.parapherna[index].disposition = Disposition::InPack;
                                paraphernalia.selected = None;
                            },
                            138..=146 => {
                                paraphernalia.parapherna[index].disposition = Disposition::InMaze;
                                paraphernalia.selected = None;
                            },
                            153..=155 => {
                                paraphernalia.parapherna[index].disposition = Disposition::InPack;
                                paraphernalia.selected = None;
                            },
                            157..=165 => {
                                paraphernalia.parapherna[index].disposition = Disposition::InMaze;
                                paraphernalia.selected = None;
                            },
                            176..=184 => {
                                paraphernalia.parapherna[index].disposition = Disposition::InMaze;
                                paraphernalia.selected = None;
                            },
                            _ => paraphernalia.selected = None,
                        }
                    }
                }
            },
            1 => { // planview
                // keyboard input
                match get_last_key_pressed() {
                    Some(KeyCode::Down) => {
                        theseus.direction = 1;
                        (play_sound, sound_index) = maze.theseus_move_forward(theseus);
                    },
                    Some(KeyCode::Up) => {
                        theseus.direction = 3;
                        (play_sound, sound_index) = maze.theseus_move_forward(theseus);
                    },
                    Some(KeyCode::Left) => {
                        theseus.direction = 2;
                        (play_sound, sound_index) = maze.theseus_move_forward(theseus);
                    },
                    Some(KeyCode::Right) => {
                        theseus.direction = 0;
                        (play_sound, sound_index) = maze.theseus_move_forward(theseus);
                    },
                    Some(KeyCode::P) => {
                        self.perspective += 1;
                        self.perspective %= NUMBER_OF_VIEWS;
                    },
                    Some(KeyCode::Q) => self.user_decided_to_exit = true,
                    Some(KeyCode::S) => { // 'secret' key to go to next level
                        for mm in 0..maze.end.len() - 1 {
                            if theseus.chamber == maze.end[mm] {
                                theseus.chamber = maze.start[mm + 1];
                            }
                        }
                    },
                    Some(KeyCode::T) => { // 'secret' key to go back to previous level
                        for mm in 1..maze.start.len() {
                            if theseus.chamber == maze.start[mm] {
                                theseus.chamber = maze.end[mm - 1];
                            }
                        }
                    },
                    _ => (),
                }
            },
            _ => eprintln!("!!!Unexpected view command: {}", &self.perspective),
        }
        //paraphernalia.update(&mut theseus, &maze);
        (play_sound, sound_index)
    }
    /// update_stage() draw perspective
    pub fn update_stage(&self, info: &mut Info, maze: &Amaze, theseus: &Theseus, paraphernalia: &Paraphernalia) {
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
                self.draw_ortho(maze, theseus, paraphernalia);
                // self.guide_lines();
            },
            1 => {
                self.draw_planview(maze, theseus);
            },
            _ => eprintln!("!!!Unexpected view command: {}", &self.perspective),
        }
        
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
        
        let left_col =(self.width - (maze.texture_cell_sizes[1] * maze.width) as f32) / 2.0;
        let top_row = (self.depth - (maze.texture_cell_sizes[1] * maze.depth) as f32) / 2.0;
        
    
        let (col, row, _) = maze.idx_to_xyz(maze.start[lost_level]);
        let start_col: f32 = col as f32;
        let start_row: f32 = row as f32;
        let (col, row, _) = maze.idx_to_xyz(maze.end[lost_level]);
        let end_col: f32 = col as f32;
        let end_row: f32 = row as f32;
    
        draw_texture(
            maze.level_sheets[lost_level][1],
            left_col,
            top_row,
            Color::new(1.0, 1.0, 1.0, 1.0),
        );
        draw_circle_lines(
            left_col + (start_col + 0.5) * maze.texture_cell_sizes[1] as f32,
            top_row + (start_row + 0.5) * maze.texture_cell_sizes[1] as f32,
            0.4 * maze.texture_cell_sizes[1] as f32,
            2.0,
            RED,
        );
        draw_circle_lines(
            left_col + (end_col + 0.5) * maze.texture_cell_sizes[1] as f32,
            top_row + (end_row + 0.5) * maze.texture_cell_sizes[1] as f32,
            0.4 * maze.texture_cell_sizes[1] as f32,
            2.0,
            GREEN,
        );
        draw_circle(
            left_col + (lost_col + 0.5) * maze.texture_cell_sizes[1] as f32,
            top_row + (lost_row + 0.5) * maze.texture_cell_sizes[1] as f32,
            0.3 * maze.texture_cell_sizes[1] as f32,
            BLUE,
        );
        for ii in (maze.width * maze.depth * lost_level)..(maze.width * maze.depth * (lost_level + 1)) {
            if maze.visited[ii] {
                let (col, row, _) = maze.idx_to_xyz(ii);
                let visit_col: f32 = col as f32;
                let visit_row: f32 = row as f32;
                draw_circle(
                    left_col + (visit_col + 0.4) * maze.texture_cell_sizes[1] as f32,
                    top_row + (visit_row + 0.4) * maze.texture_cell_sizes[1] as f32,
                    0.1 * maze.texture_cell_sizes[1] as f32,
                    GOLD,
                );
            }
        }
        for jj in 0..maze.solutions[lost_level].len() {
            let (col, row) = maze.idx_to_xy(maze.solutions[lost_level][jj]);
            let solution_col: f32 = col as f32;
            let solution_row: f32 = row as f32;
            draw_circle(
                left_col + (solution_col + 0.6) * maze.texture_cell_sizes[1] as f32,
                top_row + (solution_row + 0.6) * maze.texture_cell_sizes[1] as f32,
                0.1 * maze.texture_cell_sizes[1] as f32,
                GREEN,
            );
        }
    }
    /// draw_ortho() displays maze from Theseus' perspective
    fn draw_ortho(&self, maze: &Amaze, theseus: &Theseus, paraphernalia: &Paraphernalia) {
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
            2 => self.draw_front_window(&maze, level as usize),
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
        self.draw_paraphernalia(&paraphernalia, &theseus);
        match paraphernalia.something_illuminated {
            0..=5 => self.draw_atmosphere(Color::new(0.0, 0.0 , 0.0, 0.97)),
            6..=10 => self.draw_atmosphere(Color::new(0.0, 0.0 , 0.0, 0.5)),
            11..=20 => self.draw_atmosphere(Color::new(1.0, 0.6 , 0.0, 0.1)),
            _ => self.draw_atmosphere(Color::new(0.0, 0.0 , 0.0, 1.0)),
        }
        let (mx, my) = mouse_position();
        let hover_tile_x: f32 = (mx / TILE).trunc();
        let hover_tile_y: f32 = (my / TILE).trunc();
        draw_rectangle_lines(hover_tile_x * TILE, hover_tile_y * TILE, TILE, TILE, 2.0, Color::new(0.5, 0.5, 0.5, 0.5));
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
    fn draw_paraphernalia(&self, paraphernalia: &Paraphernalia, theseus: &Theseus) {
        let colour: Color = Color::new(0.5, 0.5, 0.5, 0.5);
        // delineate body, rucksack, discard
        draw_line(
            TILE,
            3.5 * TILE,
            4.0 * TILE,
            3.5 * TILE,
            3.0,
            LIGHTGRAY,
        );
        draw_line(
            TILE,
            8.5 * TILE,
            4.0 * TILE,
            8.5 * TILE,
            3.0,
            LIGHTGRAY,
        );
        // draw head, neck, hands & discard tiles
        draw_circle_lines( // head
            2.5 * TILE,
            1.5 * TILE,
            0.5 * TILE,
            2.0,
            GRAY,
        );
        draw_rectangle_lines( // left hand
            1.0 * TILE,
            2.75 * TILE,
            0.5 * TILE,
            0.25 * TILE,
            2.0,
            GRAY,
        );
        draw_rectangle_lines( // left hand
            1.0 * TILE,
            2.5 * TILE,
            0.125 * TILE,
            0.25 * TILE,
            2.0,
            GRAY,
        );
        draw_rectangle_lines( // left hand
            1.125 * TILE,
            2.25 * TILE,
            0.125 * TILE,
            0.5 * TILE,
            2.0,
            GRAY,
        );
        draw_rectangle_lines( // left hand
            1.25 * TILE,
            2.125 * TILE,
            0.125 * TILE,
            0.625 * TILE,
            2.0,
            GRAY,
        );
        draw_rectangle_lines( // left hand
            1.375 * TILE,
            2.25 * TILE,
            0.125 * TILE,
            0.5 * TILE,
            2.0,
            GRAY,
        );
        draw_rectangle_lines( // left hand
            1.5 * TILE,
            2.875 * TILE,
            0.25 * TILE,
            0.125 * TILE,
            2.0,
            GRAY,
        );
        draw_line( // neck
            2.125 * TILE,
            2.125 * TILE,
            2.125 * TILE,
            2.875 * TILE,
            2.0,
            GRAY,
        );
        draw_line( // neck
            2.875 * TILE,
            2.125 * TILE,
            2.875 * TILE,
            2.875 * TILE,
            2.0,
            GRAY,
        );
        draw_rectangle_lines( // right hand
            3.5 * TILE,
            2.75 * TILE,
            0.5 * TILE,
            0.25 * TILE,
            2.0,
            GRAY,
        );
        draw_rectangle_lines( // right hand
            3.875 * TILE,
            2.5 * TILE,
            0.125 * TILE,
            0.25 * TILE,
            2.0,
            GRAY,
        );
        draw_rectangle_lines( // right hand
            3.75 * TILE,
            2.25 * TILE,
            0.125 * TILE,
            0.5 * TILE,
            2.0,
            GRAY,
        );
        draw_rectangle_lines( // right hand
            3.625 * TILE,
            2.125 * TILE,
            0.125 * TILE,
            0.625 * TILE,
            2.0,
            GRAY,
        );
        draw_rectangle_lines( // right hand
            3.5 * TILE,
            2.25 * TILE,
            0.125 * TILE,
            0.5 * TILE,
            2.0,
            GRAY,
        );
        draw_rectangle_lines( // right hand
            3.25 * TILE,
            2.875 * TILE,
            0.25 * TILE,
            0.125 * TILE,
            2.0,
            GRAY,
        );
        // draw_text(text: &str, x: f32, y: f32, font_size: f32, color: Color)
        draw_text("HEAD", 2.25 * TILE, 1.625 * TILE, 16.0, colour);
        //draw_text("LEFT", 1.125 * TILE, 2.375 * TILE, 16.0, colour);
        //draw_text("HAND", 1.125 * TILE, 2.75 * TILE, 16.0, colour);
        draw_text("NECK", 2.25 * TILE, 2.625 * TILE, 16.0, colour);
        //draw_text("RIGHT", 3.125 * TILE, 2.375 * TILE, 16.0, colour);
        //draw_text("HAND", 3.125 * TILE, 2.75 * TILE, 16.0, colour);
        draw_text("RUCKSACK", 1.875 * TILE, 3.875 * TILE, 16.0, LIGHTGRAY);
        for ii in 0..paraphernalia.parapherna.len() {
            if paraphernalia.parapherna[ii].chamber == theseus.chamber && paraphernalia.parapherna[ii].direction == theseus.direction {
                let (col, row) = self.tile_2_cr(paraphernalia.parapherna[ii].tile);
                match paraphernalia.parapherna[ii].name.as_str() {
                    "Lantern" => self.draw_lantern(paraphernalia.parapherna[ii].tile, paraphernalia.parapherna[ii].active),
                    "Canteen" => self.draw_canteen(paraphernalia.parapherna[ii].tile, paraphernalia.parapherna[ii].active),
                    "Fruitcake" => self.draw_fruitcake(paraphernalia.parapherna[ii].tile),
                    "Spectacles" => self.draw_spectacles(paraphernalia.parapherna[ii].tile),
                    "Ariadne's Thread" => self.draw_spool(paraphernalia.parapherna[ii].tile),
                    "Compass" => self.draw_compass(theseus, paraphernalia.parapherna[ii].tile, paraphernalia.parapherna[ii].active),
                    "Violet Glasses" => self.draw_violet_glasses(paraphernalia.parapherna[ii].tile),
                    "Correspondence" => self.draw_correspondence(paraphernalia.parapherna[ii].tile),
                    "Journal" => self.draw_journal(paraphernalia.parapherna[ii].tile),
                    _ => {
                        draw_rectangle(
                            (col + 0.125) * TILE,
                            (row + 0.125) * TILE,
                            0.75 * TILE,
                            0.75 * TILE,
                            paraphernalia.parapherna[ii].colour,
                        );
                    },
                }
                if paraphernalia.selected == Some(ii) {
                    draw_rectangle_lines(
                        col * TILE,
                        row * TILE,
                        TILE,
                        TILE,
                        2.0,
                        GOLD,
                    );
                    draw_text(&paraphernalia.parapherna[ii].name, 1.5 * TILE, 9.875 * TILE, 16.0, LIGHTGRAY);
                }
            }
        }
    }
    fn draw_lantern(&self, tile_index: u8, active: bool) {
        let (ulx, uly) = self.tile_2_xy(tile_index);
        draw_rectangle_lines(
            ulx + 12.0,
            uly + 6.0,
            24.0,
            20.0,
            4.0,
            GOLD,
        );
        draw_rectangle(
            ulx + 16.0,
            uly + 18.0,
            16.0,
            24.0,
            DARKGREEN,
        );
        if active {
            draw_circle(
                ulx + 24.0,
                uly + 30.0,
                7.0,
                YELLOW,
            );
        } else {
            draw_circle(
                ulx + 24.0,
                uly + 30.0,
                7.0,
                Color::new(0.5, 0.5, 0.0, 1.0),
            );
        }
        draw_rectangle_lines(
            ulx + 14.0,
            uly + 18.0,
            20.0,
            24.0,
            4.0,
            GOLD,
        );
        draw_line(
            ulx + 12.0,
            uly + 43.0,
            ulx + 36.0,
            uly + 43.0,
            4.0,
            GOLD,
        );
    }
    fn draw_canteen(&self, tile_index: u8, active: bool) {
        let (ulx, uly) = self.tile_2_xy(tile_index);
        draw_rectangle(
            ulx + 20.0,
            uly + 4.0,
            8.0,
            8.0,
            DARKGRAY,
        );
        draw_circle_lines(
            ulx + 28.0,
            uly + 8.0,
            4.0,
            1.0,
            DARKGRAY,
        );
        if active {
            draw_circle(
                ulx + 24.0,
                uly + 28.0,
                16.0,
                Color::new(0.0, 0.0, 0.7, 0.8),
            );
        } else {
            draw_circle(
                ulx + 24.0,
                uly + 28.0,
                16.0,
                DARKGREEN,
            );
        }
        draw_line(
            ulx + 8.0,
            uly + 24.0,
            ulx + 40.0,
            uly + 24.0,
            2.0,
            DARKGRAY,
        );
        draw_line(
            ulx + 8.0,
            uly + 32.0,
            ulx + 40.0,
            uly + 32.0,
            2.0,
            DARKGRAY,
        );
    }
    fn draw_fruitcake(&self, tile_index: u8) {
        let (ulx, uly) = self.tile_2_xy(tile_index);
        // draw_poly(x: f32, y: f32, sides: u8, radius: f32, rotation: f32, color: Color)
        draw_poly(
            ulx + 24.0,
            uly + 24.0,
            5,
            17.0,
            -20.0,
            YELLOW,
        );
        draw_poly_lines(
            ulx + 24.0,
            uly + 24.0,
            5,
            17.0,
            -20.0,
            2.0,
            BROWN,
        );
        draw_circle(
            ulx + 20.0,
            uly + 28.0,
            2.0,
            RED,
        );
        draw_circle(
            ulx + 23.0,
            uly + 15.0,
            2.0,
            GREEN,
        );
        draw_circle(
            ulx + 18.0,
            uly + 16.0,
            2.0,
            ORANGE,
        );
        draw_circle(
            ulx + 34.0,
            uly + 23.0,
            2.0,
            RED,
        );
        draw_circle(
            ulx + 28.0,
            uly + 31.0,
            2.0,
            GREEN,
        );
        draw_circle(
            ulx + 19.0,
            uly + 33.0,
            2.0,
            RED,
        );
        draw_circle(
            ulx + 33.0,
            uly + 15.0,
            2.0,
            GREEN,
        );
        draw_circle(
            ulx + 22.0,
            uly + 26.0,
            2.0,
            ORANGE,
        );
        draw_circle(
            ulx + 15.0,
            uly + 23.0,
            2.0,
            RED,
        );
        draw_circle(
            ulx + 28.0,
            uly + 25.0,
            2.0,
            GREEN,
        );
        
        draw_circle(
            ulx + 8.0,
            uly + 16.0,
            2.0,
            ORANGE,
        );
        draw_circle(
            ulx + 13.0,
            uly + 14.0,
            2.0,
            RED,
        );
        draw_circle(
            ulx + 16.0,
            uly + 11.0,
            2.0,
            GREEN,
        );
        
        draw_circle(
            ulx + 38.0,
            uly + 16.0,
            2.0,
            ORANGE,
        );
        draw_circle(
            ulx + 33.0,
            uly + 14.0,
            2.0,
            RED,
        );
        draw_circle(
            ulx + 30.0,
            uly + 11.0,
            2.0,
            GREEN,
        );
        draw_circle(
            ulx + 24.0,
            uly + 9.0,
            2.0,
            RED,
        );
    }
    fn draw_spectacles(&self, tile_index: u8) {
        let (ulx, uly) = self.tile_2_xy(tile_index);
        draw_line(
            ulx + 20.0,
            uly + 28.0,
            ulx + 36.0,
            uly + 22.0,
            1.0,
            GOLD,
        );
        draw_line(
            ulx + 28.0,
            uly + 12.0,
            ulx + 40.0,
            uly + 22.0,
            1.0,
            GOLD,
        );
        draw_line(
            ulx + 28.0,
            uly + 12.0,
            ulx + 24.0,
            uly + 18.0,
            1.0,
            GOLD,
        );
        draw_line(
            ulx + 6.0,
            uly + 24.0,
            ulx + 16.0,
            uly + 32.0,
            1.0,
            GOLD,
        );
        draw_line(
            ulx + 6.0,
            uly + 24.0,
            ulx + 2.0,
            uly + 28.0,
            1.0,
            GOLD,
        );
        // draw_poly(x: f32, y: f32, sides: u8, radius: f32, rotation: f32, color: Color)
        draw_circle(
            ulx + 20.0,
            uly + 32.0,
            7.0,
            LIGHTGRAY,
        );
        draw_circle(
            ulx + 37.0,
            uly + 26.0,
            7.0,
            LIGHTGRAY,
        );
        draw_circle_lines(
            ulx + 20.0,
            uly + 32.0,
            7.0,
            1.5,
            GOLD,
        );
        draw_circle_lines(
            ulx + 37.0,
            uly + 26.0,
            7.0,
            1.5,
            GOLD,
        );
    }
    fn draw_journal(&self, tile_index: u8) {
        let (ulx, uly) = self.tile_2_xy(tile_index);
        // draw_poly(x: f32, y: f32, sides: u8, radius: f32, rotation: f32, color: Color)
        draw_poly(
            ulx + 24.0,
            uly + 29.0,
            4,
            18.0,
            -22.5,
            BROWN,
        );
        draw_poly(
            ulx + 24.0,
            uly + 27.0,
            4,
            18.0,
            -22.5,
            WHITE,
        );
        draw_poly(
            ulx + 24.0,
            uly + 26.0,
            4,
            18.0,
            -22.5,
            LIGHTGRAY,
        );
        draw_poly(
            ulx + 24.0,
            uly + 25.0,
            4,
            18.0,
            -22.5,
            GRAY,
        );
        draw_poly(
            ulx + 24.0,
            uly + 24.0,
            4,
            18.0,
            -22.5,
            WHITE,
        );
        draw_poly(
            ulx + 24.0,
            uly + 23.0,
            4,
            18.0,
            -22.5,
            LIGHTGRAY,
        );
        draw_poly(
            ulx + 24.0,
            uly + 22.0,
            4,
            18.0,
            -22.5,
            WHITE,
        );
        draw_poly(
            ulx + 24.0,
            uly + 21.0,
            4,
            18.0,
            -22.5,
            BROWN,
        );
        draw_line(
            ulx + 9.0,
            uly + 28.0,
            ulx + 9.0,
            uly + 36.0,
            2.0,
            BROWN,
        );
        draw_line(
            ulx + 20.0,
            uly + 28.0,
            ulx + 30.0,
            uly + 18.0,
            4.0,
            BLACK,
        );
    }
    fn draw_correspondence(&self, tile_index: u8) {
        let (ulx, uly) = self.tile_2_xy(tile_index);
        // draw_poly(x: f32, y: f32, sides: u8, radius: f32, rotation: f32, color: Color)
        draw_poly(
            ulx + 24.0,
            uly + 32.0,
            4,
            18.0,
            22.5,
            GRAY,
        );
        draw_poly(
            ulx + 24.0,
            uly + 31.0,
            4,
            18.0,
            22.5,
            LIGHTGRAY,
        );
        draw_poly(
            ulx + 24.0,
            uly + 30.0,
            4,
            18.0,
            22.5,
            DARKGRAY,
        );
        draw_poly(
            ulx + 24.0,
            uly + 29.0,
            4,
            18.0,
            22.5,
            WHITE,
        );
        draw_poly(
            ulx + 24.0,
            uly + 28.0,
            4,
            18.0,
            22.5,
            LIGHTGRAY,
        );
        draw_poly(
            ulx + 24.0,
            uly + 27.0,
            4,
            18.0,
            22.5,
            WHITE,
        );
        draw_poly(
            ulx + 24.0,
            uly + 26.0,
            4,
            18.0,
            22.5,
            LIGHTGRAY,
        );
        draw_poly(
            ulx + 24.0,
            uly + 25.0,
            4,
            18.0,
            22.5,
            GRAY,
        );
        draw_poly(
            ulx + 24.0,
            uly + 24.0,
            4,
            18.0,
            22.5,
            WHITE,
        );
        draw_poly(
            ulx + 24.0,
            uly + 23.0,
            4,
            18.0,
            22.5,
            LIGHTGRAY,
        );
        draw_poly(
            ulx + 24.0,
            uly + 22.0,
            4,
            18.0,
            22.5,
            WHITE,
        );
        draw_line(
            ulx + 20.0,
            uly + 28.0,
            ulx + 30.0,
            uly + 18.0,
            4.0,
            RED,
        );
        draw_line(
            ulx + 20.0,
            uly + 6.0,
            ulx + 28.0,
            uly + 36.0,
            2.0,
            RED,
        );
        draw_line(
            ulx + 28.0,
            uly + 36.0,
            ulx + 28.0,
            uly + 44.0,
            2.0,
            RED,
        );
        draw_line(
            ulx + 12.0,
            uly + 26.0,
            ulx + 12.0,
            uly + 38.0,
            2.0,
            RED,
        );
        draw_line(
            ulx + 12.0,
            uly + 26.0,
            ulx + 36.0,
            uly + 18.0,
            2.0,
            RED,
        );
    }
    fn draw_spool(&self, tile_index: u8) {
        let (ulx, uly) = self.tile_2_xy(tile_index);
        draw_line(
            ulx + 6.0,
            uly + 6.0,
            ulx + 42.0,
            uly + 42.0,
            4.0,
            BROWN,
        );
        draw_line(
            ulx + 4.0,
            uly + 18.0,
            ulx + 18.0,
            uly + 4.0,
            4.0,
            BROWN,
        );
        draw_line(
            ulx + 44.0,
            uly + 30.0,
            ulx + 30.0,
            uly + 44.0,
            4.0,
            BROWN,
        );
        // draw_poly(x: f32, y: f32, sides: u8, radius: f32, rotation: f32, color: Color)
        draw_poly(
            ulx + 18.0,
            uly + 18.0,
            4,
            10.0,
            0.0,
            GOLD,
        );
        draw_poly(
            ulx + 30.0,
            uly + 30.0,
            4,
            10.0,
            0.0,
            GOLD,
        );
        draw_line(
            ulx + 20.0,
            uly + 28.0,
            ulx + 30.0,
            uly + 18.0,
            2.0,
            YELLOW,
        );
        draw_line(
            ulx + 30.0,
            uly + 18.0,
            ulx + 36.0,
            uly + 18.0,
            1.0,
            YELLOW,
        );
    }
    fn draw_compass(&self, theseus: &Theseus, tile_index: u8, active: bool) {
        let (ulx, uly) = self.tile_2_xy(tile_index);
        draw_circle_lines(
            ulx + 24.0,
            uly + 18.0,
            14.0,
            2.0,
            DARKGREEN,
        );
        draw_circle(
            ulx + 24.0,
            uly + 32.0,
            10.0,
            DARKGREEN,
        );
        draw_circle_lines(
            ulx + 24.0,
            uly + 32.0,
            10.0,
            2.0,
            GOLD,
        );
        draw_line(
            ulx + 14.0,
            uly + 32.0,
            ulx + 34.0,
            uly + 32.0,
            1.0,
            GOLD,
        );
        draw_line(
            ulx + 24.0,
            uly + 22.0,
            ulx + 24.0,
            uly + 42.0,
            1.0,
            GOLD,
        );
        draw_line(
            ulx + 18.0,
            uly + 35.0,
            ulx + 30.0,
            uly + 28.0,
            2.0,
            BLACK,
        );
        if active {
            match theseus.direction {
                0 => draw_text("< EAST >", 8.75 * TILE, 0.625 * TILE, 16.0, GRAY),
                1 => draw_text("< SOUTH >", 8.75 * TILE, 0.625 * TILE, 16.0, GRAY),
                2 => draw_text("< WEST >", 8.75 * TILE, 0.625 * TILE, 16.0, GRAY),
                3 => draw_text("< NORTH >", 8.75 * TILE, 0.625 * TILE, 16.0, GRAY),
               _ => eprintln!("!!!Unexpected direction: {:?}", &theseus.direction),
            }
        }
    }
    fn draw_violet_glasses(&self, tile_index: u8) {
        let (ulx, uly) = self.tile_2_xy(tile_index);
        draw_line(
            ulx + 20.0,
            uly + 28.0,
            ulx + 36.0,
            uly + 22.0,
            1.0,
            GOLD,
        );
        draw_line(
            ulx + 28.0,
            uly + 12.0,
            ulx + 40.0,
            uly + 22.0,
            1.0,
            GOLD,
        );
        draw_line(
            ulx + 28.0,
            uly + 12.0,
            ulx + 24.0,
            uly + 18.0,
            1.0,
            GOLD,
        );
        draw_line(
            ulx + 6.0,
            uly + 24.0,
            ulx + 16.0,
            uly + 32.0,
            1.0,
            GOLD,
        );
        draw_line(
            ulx + 6.0,
            uly + 24.0,
            ulx + 2.0,
            uly + 28.0,
            1.0,
            GOLD,
        );
        // draw_poly(x: f32, y: f32, sides: u8, radius: f32, rotation: f32, color: Color)
        draw_poly(
            ulx + 20.0,
            uly + 32.0,
            6,
            8.0,
            0.0,
            VIOLET,
        );
        draw_poly(
            ulx + 37.0,
            uly + 26.0,
            6,
            8.0,
            0.0,
            VIOLET,
        );
        draw_poly_lines(
            ulx + 20.0,
            uly + 32.0,
            6,
            8.0,
            0.0,
            1.0,
            GOLD,
        );
        draw_poly_lines(
            ulx + 37.0,
            uly + 26.0,
            6,
            8.0,
            0.0,
            1.0,
            GOLD,
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