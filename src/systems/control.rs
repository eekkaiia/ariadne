use macroquad::{
    //audio,
    //audio::Sound,
    color::Color,
    color::colors::*,
    input::*,
    math::*,
    shapes::*,
    text::*,
    //texture::*,
    //ui::root_ui,
    //window::*,
};

use crate::systems::*;
use crate::entities::*;

const TILE: f32 = 48.0;
const NUMBER_OF_VIEWS: usize = 4;

pub struct Control {
    pub name: Vec<String>,
    pub tile: Vec<u8>,
    pub active_controls: Vec<bool>,
    pub examine_paraphernalia: usize,
    pub examine_portalia: usize,
    pub page: usize,
    pub user_decided_to_exit: bool,
}
impl Control {
    pub fn new() -> Self {
        let name: Vec<String> = vec![
            "UP [UP ARROW]".to_string(),
            "DOWN [DOWN ARROW]".to_string(),
            "LEFT [LEFT ARROW]".to_string(),
            "RIGHT [RIGHT ARROW]".to_string(),
            "FORWARD [SPACEBAR]".to_string(),
            "EXAMINE [E]".to_string(),
            "UNDO [BACKSPACE]".to_string(),
            "CLOSE [Q]".to_string(),
        ];
        let tile: Vec<u8> = vec![73, 111, 91, 93, 92, 148, 149, 150];
        let active_controls: Vec<bool> = vec![false, false, false, false, false, false, false, false];
        Self {
            name,
            tile,
            active_controls,
            examine_paraphernalia: 0,
            examine_portalia: 0,
            page: 0,
            user_decided_to_exit: false,
        }
    }
    pub fn update(
        &mut self, 
        stage: &mut Stage, 
        maze: &mut Amaze, 
        theseus: &mut Theseus, 
        paraphernalia: &mut Paraphernalia,
        portalia: &mut Portalia,
    ) {
        match stage.perspective {
            0 => { // orthoview
                // keyboard input
                match get_last_key_pressed() {
                    Some(KeyCode::Down) => 
                    /*
                    ??? use up/down keys to 'look' up or down ???
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
                    Some(KeyCode::E) => {
                        match portalia.selected {
                            None => (),
                            Some(item) => {
                                self.examine_portalia = item;
                                stage.perspective = 2;
                            },
                        }
                        match paraphernalia.selected {
                            None => (),
                            Some(item) => {
                                self.examine_paraphernalia = item;
                                stage.perspective = 3;
                            },
                        }
                    },
                    Some(KeyCode::Space) => {
                        maze.theseus_move_forward(theseus);
                    },
                    Some(KeyCode::P) => {
                        stage.perspective += 1;
                        stage.perspective %= NUMBER_OF_VIEWS;
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
                    
                    match tile_index {
                        91 => {
                            match theseus.direction {
                                3 => theseus.direction = 2,
                                0 => theseus.direction = 3,
                                1 => theseus.direction = 0,
                                2 => theseus.direction = 1,
                                _ => eprintln!("!!!Unexpected direction char: {:?}", &theseus.direction),
                            }
                        },
                        92 => {
                            maze.theseus_move_forward(theseus);
                        },
                        93 => {
                            match theseus.direction {
                                3 => theseus.direction = 0,
                                0 => theseus.direction = 1,
                                1 => theseus.direction = 2,
                                2 => theseus.direction = 3,
                                _ => eprintln!("!!!Unexpected direction char: {:?}", &theseus.direction),
                            }
                        },
                        148 => {
                            match portalia.selected {
                                None => (),
                                Some(item) => {
                                    self.examine_portalia = item;
                                    stage.perspective = 2;
                                },
                            }
                            match paraphernalia.selected {
                                None => (),
                                Some(item) => {
                                    self.examine_paraphernalia = item;
                                    stage.perspective = 3;
                                },
                            }
                        },
                        150 => self.user_decided_to_exit = true,
                        _ => (),
                    }
                    
                    let mut new_paraphernalia_selected: Option<usize> = None;
                    let mut new_portalia_selected: Option<usize> = None;
                    for ii in 0..paraphernalia.parapherna.len() {
                        if paraphernalia.parapherna[ii].tile == tile_index
                            && paraphernalia.parapherna[ii].chamber == theseus.chamber
                            && paraphernalia.parapherna[ii].direction == theseus.direction
                        { new_paraphernalia_selected = Some(ii); };
                    }
                    for ii in 0..portalia.portals.len() {
                        for jj in 0..portalia.portals[ii].tiles.len() {
                            if portalia.portals[ii].tiles[jj] == tile_index
                                && portalia.portals[ii].chamber == theseus.chamber
                                && portalia.portals[ii].direction == theseus.direction
                            { new_portalia_selected = Some(ii); };
                        }
                    }
                    // rewritten match
                    if paraphernalia.selected == None && portalia.selected == None {
                        for ii in 0..paraphernalia.parapherna.len() {
                            if paraphernalia.parapherna[ii].tile == tile_index
                                && paraphernalia.parapherna[ii].chamber == theseus.chamber
                                && paraphernalia.parapherna[ii].direction == theseus.direction
                            { paraphernalia.selected = Some(ii); };
                        }
                        for ii in 0..portalia.portals.len() {
                            for jj in 0..portalia.portals[ii].tiles.len() {
                                if portalia.portals[ii].tiles[jj] == tile_index
                                    && portalia.portals[ii].chamber == theseus.chamber
                                    && portalia.portals[ii].direction == theseus.direction
                                { portalia.selected = Some(ii); };
                            }
                        }
                    } else if new_paraphernalia_selected != None && new_paraphernalia_selected != paraphernalia.selected {
                        paraphernalia.selected = new_paraphernalia_selected;
                        portalia.selected = None;
                    } else if new_portalia_selected != None && new_portalia_selected != portalia.selected {
                        portalia.selected = new_portalia_selected;
                        paraphernalia.selected = None;
                    } else if paraphernalia.selected != None {
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
                    } else {
                        portalia.selected = None;
                    }
                }
            },
            1 => { // planview
                // keyboard input
                match get_last_key_pressed() {
                    Some(KeyCode::Down) => {
                        theseus.direction = 1;
                        maze.theseus_move_forward(theseus);
                    },
                    Some(KeyCode::Up) => {
                        theseus.direction = 3;
                        maze.theseus_move_forward(theseus);
                    },
                    Some(KeyCode::Left) => {
                        theseus.direction = 2;
                        maze.theseus_move_forward(theseus);
                    },
                    Some(KeyCode::Right) => {
                        theseus.direction = 0;
                        maze.theseus_move_forward(theseus);
                    },
                    Some(KeyCode::P) => {
                        stage.perspective += 1;
                        stage.perspective %= NUMBER_OF_VIEWS;
                    },
                    Some(KeyCode::Q) => stage.perspective = 0,
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
                    match tile_index {
                        73 => {
                            theseus.direction = 3;
                            maze.theseus_move_forward(theseus);
                        },
                        91 => {
                            theseus.direction = 2;
                            maze.theseus_move_forward(theseus);
                        },
                        93 => {
                            theseus.direction = 0;
                            maze.theseus_move_forward(theseus);
                        },
                        111 => {
                            theseus.direction = 1;
                            maze.theseus_move_forward(theseus);
                        },
                        150 => stage.perspective = 0,
                        _ => (),
                    }
                }
            },
            2 => {
                // keyboard input
                match get_last_key_pressed() {
                    Some(KeyCode::P) => {
                        stage.perspective += 1;
                        stage.perspective %= NUMBER_OF_VIEWS;
                    },
                    Some(KeyCode::Q) => stage.perspective = 0,
                    _ => (),
                }
                // mouse input
                if is_mouse_button_pressed(MouseButton::Right) || is_mouse_button_pressed(MouseButton::Left) {
                    let (mx, my) = mouse_position();
                    let select_tile_x: u8 = (mx / TILE).trunc() as u8;
                    let select_tile_y: u8 = (my / TILE).trunc() as u8;
                    let tile_index: u8 = (select_tile_y * 19) + select_tile_x;
                    match tile_index {
                        150 => stage.perspective = 0,
                        _ => (),
                    }
                }
            },
            3 => {
                // keyboard input
                match get_last_key_pressed() {
                    Some(KeyCode::Left) => {
                        if self.page > 0 { self.page -= 1; };
                    },
                    Some(KeyCode::Right) => {
                        self.page += 1;
                    },
                    Some(KeyCode::P) => {
                        stage.perspective += 1;
                        stage.perspective %= NUMBER_OF_VIEWS;
                    },
                    Some(KeyCode::Q) => {
                        stage.perspective = 0;
                        self.page = 0;
                    },
                    _ => (),
                }
                // mouse input
                if is_mouse_button_pressed(MouseButton::Right) || is_mouse_button_pressed(MouseButton::Left) {
                    let (mx, my) = mouse_position();
                    let select_tile_x: u8 = (mx / TILE).trunc() as u8;
                    let select_tile_y: u8 = (my / TILE).trunc() as u8;
                    let tile_index: u8 = (select_tile_y * 19) + select_tile_x;
                    match tile_index {
                        91 => { // go back one page
                            if self.page > 0 { self.page -= 1; };
                        },
                        93 => { // go forward one page
                            self.page += 1;
                        },
                        150 => {
                            stage.perspective = 0;
                            self.page = 0;
                        },
                        _ => (),
                    }
                }
            },
            _ => eprintln!("!!!Unexpected view command: {}", &stage.perspective),
        }
    }
    pub fn draw_control(&mut self, perspective: u8) {
        match perspective {
            0 => self.active_controls = vec![false, false, true, true, true, true, false, true],
            1 => self.active_controls = vec![true, true, true, true, false, false, false, true],
            2 => self.active_controls = vec![false, false, false, false, false, false, false, true],
            3 => self.active_controls = vec![false, false, true, true, false, false, false, true],
            _ => eprintln!("!!!Unexpected perspective: {}", &perspective),
        }
        let (mx, my) = mouse_position();
        let hover_tile_x: u8 = (mx / TILE).trunc() as u8;
        let hover_tile_y: u8 = (my / TILE).trunc() as u8;
        let hover_tile_index: u8 = (hover_tile_y * 19) + hover_tile_x;
        for ii in 0..self.tile.len() {
            if hover_tile_index == self.tile[ii] {
                let mut colour: Color = DARKGRAY;
                if self.active_controls[ii] { colour = LIGHTGRAY };
                draw_text(&self.name[ii], 15.0 * TILE, 9.875 * TILE, 16.0, colour);
            }
            match self.tile[ii] {
                73 => self.draw_up(),
                91 => self.draw_left(),
                92 => self.draw_octagon(),
                93 => self.draw_right(),
                111 => self.draw_down(),
                148 => self.draw_examine(),
                149 => self.draw_undo(),
                150 => self.draw_exit(),
                _ => eprintln!("!!!Unexpected control tile: {}", &self.tile[ii]),
            }
        }
    }
    fn draw_up(&self) { // '^' button
        let (col, row) = self.tile_2_cr(self.tile[0]);
        let mut colour: Color = DARKGRAY;
        let mut thickness: f32 = 1.0;
        if self.active_controls[0] {
            colour = LIGHTGRAY;
            thickness = 3.0;
        }
        draw_rectangle_lines(
            (col + 0.125) * TILE,
            (row + 0.125) * TILE,
            0.75 * TILE,
            0.75 * TILE,
            thickness,
            colour,
        );
        draw_poly_lines(
            (col * TILE) + 24.0,
            (row * TILE) + 24.0,
            3,
            0.2 * TILE,
            30.0,
            thickness,
            colour,
        );
    }
    fn draw_down(&self) { // 'v' button
        let (col, row) = self.tile_2_cr(self.tile[1]);
        let mut colour: Color = DARKGRAY;
        let mut thickness: f32 = 1.0;
        if self.active_controls[1] {
            colour = LIGHTGRAY;
            thickness = 3.0;
        }
        draw_rectangle_lines(
            (col + 0.125) * TILE,
            (row + 0.125) * TILE,
            0.75 * TILE,
            0.75 * TILE,
            thickness,
            colour,
        );
        draw_poly_lines(
            (col * TILE) + 24.0,
            (row * TILE) + 24.0,
            3,
            0.2 * TILE,
            -30.0,
            thickness,
            colour,
        );
    }
    fn draw_left(&self) { // '<' button
        let (col, row) = self.tile_2_cr(self.tile[2]);
        let mut colour: Color = DARKGRAY;
        let mut thickness: f32 = 1.0;
        if self.active_controls[2] {
            colour = LIGHTGRAY;
            thickness = 3.0;
        }
        draw_rectangle_lines(
            (col + 0.125) * TILE,
            (row + 0.125) * TILE,
            0.75 * TILE,
            0.75 * TILE,
            thickness,
            colour,
        );
        draw_poly_lines(
            (col * TILE) + 24.0,
            (row * TILE) + 24.0,
            3,
            0.2 * TILE,
            -60.0,
            thickness,
            colour,
        );
    }
    fn draw_right(&self) { // '>' button
        let (col, row) = self.tile_2_cr(self.tile[3]);
        let mut colour: Color = DARKGRAY;
        let mut thickness: f32 = 1.0;
        if self.active_controls[3] {
            colour = LIGHTGRAY;
            thickness = 3.0;
        }
        draw_rectangle_lines(
            (col + 0.125) * TILE,
            (row + 0.125) * TILE,
            0.75 * TILE,
            0.75 * TILE,
            thickness,
            colour,
        );
        draw_poly_lines(
            (col * TILE) + 24.0,
            (row * TILE) + 24.0,
            3,
            0.2 * TILE,
            0.0,
            thickness,
            colour,
        );
    }
    fn draw_octagon(&self) { // 'Octagon' button
        let (col, row) = self.tile_2_cr(self.tile[4]);
        let mut colour: Color = DARKGRAY;
        let mut thickness: f32 = 1.0;
        if self.active_controls[4] {
            colour = LIGHTGRAY;
            thickness = 3.0;
        }
        draw_rectangle_lines(
            (col + 0.125) * TILE,
            (row + 0.125) * TILE,
            0.75 * TILE,
            0.75 * TILE,
            thickness,
            colour,
        );
        draw_poly_lines(
            (col * TILE) + 24.0,
            (row * TILE) + 24.0,
            8,
            0.2 * TILE,
            22.0,
            thickness,
            colour,
        );
    }
    fn draw_examine(&self) { // 'examine' button
        let (col, row) = self.tile_2_cr(self.tile[5]);
        let mut colour: Color = DARKGRAY;
        let mut thickness: f32 = 1.0;
        if self.active_controls[5] {
            colour = LIGHTGRAY;
            thickness = 3.0;
        }
        draw_rectangle_lines(
            (col + 0.125) * TILE,
            (row + 0.125) * TILE,
            0.75 * TILE,
            0.75 * TILE,
            thickness,
            colour,
        );
        draw_circle_lines(
            (col * TILE) + 26.0,
            (row * TILE) + 22.0,
            0.2 * TILE,
            thickness,
            colour,
        );
        draw_line(
            (col * TILE) + 12.0,
            (row * TILE) + 38.0,
            (col * TILE) + 20.0,
            (row * TILE) + 28.0,
            thickness,
            colour,
        );
    }
    fn draw_undo(&self) { // 'Undo' button
        let (col, row) = self.tile_2_cr(self.tile[6]);
        let mut colour: Color = DARKGRAY;
        let mut thickness: f32 = 1.0;
        if self.active_controls[6] {
            colour = LIGHTGRAY;
            thickness = 3.0;
        }
        draw_rectangle_lines(
            (col + 0.125) * TILE,
            (row + 0.125) * TILE,
            0.75 * TILE,
            0.75 * TILE,
            thickness,
            colour,
        );
        draw_circle_lines(
            (col * TILE) + 24.0,
            (row * TILE) + 24.0,
            0.2 * TILE,
            thickness,
            colour,
        );
    }
    fn draw_exit(&self) { // 'X' button
        let (col, row) = self.tile_2_cr(self.tile[7]);
        let mut colour: Color = DARKGRAY;
        let mut thickness: f32 = 1.0;
        if self.active_controls[7] {
            colour = LIGHTGRAY;
            thickness = 3.0;
        }
        draw_rectangle_lines(
            (col + 0.125) * TILE,
            (row + 0.125) * TILE,
            0.75 * TILE,
            0.75 * TILE,
            thickness,
            colour,
        );
        draw_line(
            (col * TILE) + 16.0,
            (row * TILE) + 16.0,
            (col * TILE) + 32.0,
            (row * TILE) + 32.0,
            thickness,
            colour,
        );
        draw_line(
            (col * TILE) + 16.0,
            (row * TILE) + 32.0,
            (col * TILE) + 32.0,
            (row * TILE) + 16.0,
            thickness,
            colour,
        );
    }
    fn tile_2_cr(&self, idx: u8) -> (f32, f32) {
        let row: f32 = (idx as f32 / 19.0).trunc();
        let col: f32 = (idx % 19) as f32;
        (col, row)
    }
}