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
pub enum Disposition {
    InLeftHand,
    InRightHand,
    OnNeck,
    OnHead,
    InPack,
    InMaze,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Parapherna {
    pub name: String,
    pub chamber: usize,
    pub direction: u8,
    pub disposition: Disposition,
    pub tile: u8,
    pub colour: Color,
    pub active: bool,
    pub examine_text: Option<Vec<Vec<String>>>,
    pub examine_image: Option<Texture2D>,
}
impl Parapherna {
    pub fn new(name: String, chamber: usize, disposition: Disposition, tile: u8, colour: Color) -> Self {
        Self {
            name,
            chamber,
            direction: 0,
            disposition,
            tile,
            colour,
            active: false,
            examine_text: None,
            examine_image: None,
        }
    }
    pub fn draw_examine_text_parapherna(&self, mut page: usize, glasses: bool) {
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
}

#[derive(Clone, Debug, PartialEq)]
pub struct Paraphernalia {
    pub parapherna: Vec<Parapherna>,
    pub selected: Option<usize>,
    pub something_illuminated: u8,
}
impl Paraphernalia {
    pub fn new() -> Self {
        let parapherna: Vec<Parapherna> = Vec::new();
        Self {
            parapherna,
            selected: None,
            something_illuminated: 0,
        }
    }
    pub fn fill_rucksack(&mut self, start: usize) {
        let stuff: Vec<(String, usize, Color)> = vec![ // limit 12 items
            ("Lantern".to_string(), start, YELLOW),
            ("Spectacles".to_string(), start, WHITE),
            ("Canteen".to_string(), start, GREEN),
            ("Fruitcake".to_string(), start, ORANGE),
            ("Bandana".to_string(), start, RED),
            ("Correspondence".to_string(), start, WHITE),
            ("Sketchbook".to_string(), start, GRAY),
        ];
        // knapsack vec![77, 78, 79, 96, 97, 98, 115, 116, 117, 134, 135, 136]
        for ii in 0..stuff.len() {
            self.parapherna.push(Parapherna::new(
                stuff[ii].0.clone(),
                stuff[ii].1,
                Disposition::InPack,
                0,
                stuff[ii].2
            ));
        }
        for jj in 0..self.parapherna.len() {
            if self.parapherna[jj].name == "Bandana".to_string() {
                self.parapherna[jj].disposition = Disposition::OnNeck;
                self.parapherna[jj].tile = 40;
                self.parapherna[jj].examine_text = Some(vec![vec![
                    "A silk bandana tied in a knot so".to_string(),
                    "it can be worn around the neck.".to_string(),
                    "Bright red color distracts bulls.".to_string(),
                ]]);
            }
            if self.parapherna[jj].name == "Correspondence".to_string() {
                self.parapherna[jj].examine_text = Some(vec![
                    vec![
                        "A bundle of letters, telegrams, postcards, etc.".to_string(),
                        "Use left/right arrow keys to read in no".to_string(),
                        "particular order.".to_string(),
                    ],
                    vec![
                        "TELEGRAM".to_string(),
                        "DO NOT BOTHER RETURNING NEXT SEMESTER STOP".to_string(),
                        "YOUR ACTIONS IN PARIS WERE ILL-CONSIDERED STOP".to_string(),
                        "WHEN THINGS QUIET DOWN WILL ADVOCATE THAT".to_string(),
                        "YOUR MASTERS BE AWARDED IN PASSING STOP".to_string(),
                        "FORGET ABOUT PHD STOP".to_string(),
                        "HOPE YOU KNOW WHAT YOU ARE DOING STOP".to_string(),
                    ],
                    vec![
                        "Dear X".to_string(),
                        "How mysterious. I approve and appreciate your manner and".to_string(),
                        "discretion in contacting me by post. For nearly half a".to_string(),
                        "century I have been bothered by people, some respectable,".to_string(),
                        "most not, with questions about my parents work. The fact".to_string(),
                        "that I am replying to your letter is indication that your".to_string(),
                        "conjecture is correct, the answer to the location of the".to_string(),
                        "labyrinth is really that simple. I will not be more".to_string(),
                        "explicit in writing.".to_string(),
                    ],
                    vec![
                        "As to your other questions, there is not much more I can".to_string(),
                        "add that has not appeared in my parents published material".to_string(),
                        "with a few exceptions. As to whether my parents made a map".to_string(),
                        "of the labyrinth: it would not be helpful. They did not".to_string(),
                        "publish this information for fear of being called quacks.".to_string(),
                        "My parents returned to the labyrinth on several occasions".to_string(),
                        "and each time the passages appeared to have changed. Marks".to_string(),
                        "that had been left on the wall during their previous".to_string(),
                        "excursions were missing or were misoriented. The general".to_string(),
                        "layout was always the same, a series of chambers, some".to_string(),
                        "with doorways to adjoining chambers, all at right angles".to_string(),
                        "and apparently oriented with the cardinal directions. As".to_string(),
                        "they described it to me there are many chambers, all".to_string(),
                        "similar in appearance with few outside reference points,".to_string(),
                        "so it was easy to lose one's way.".to_string(),
                    ],
                    vec![
                        "A piece of information that my parents did not publish".to_string(),
                        "and can only benefit someone who finds the entrance to".to_string(),
                        "the labyrinth is the existence of a spool of golden thread.".to_string(),
                        "My parents found it in a chamber near the entrance. Father".to_string(),
                        "wanted to take it with them when they finally sealed up the".to_string(),
                        "labyrinth, but mother insisted that it be left for anyone".to_string(),
                        "who would come later. She called it Ariadne's thread and".to_string(),
                        "it served well in that role - better than the twine and".to_string(),
                        "markings my parents first used.".to_string(),
                    ],
                     
                ]);
            }
            if self.parapherna[jj].name == "Lantern".to_string() {
                self.parapherna[jj].examine_text = Some(vec![vec![
                    "A kerosene lantern with a carrying handle.".to_string(),
                    "Its not advisable to light while stored in a backpack.".to_string(),
                ]]);
            }
            if self.parapherna[jj].name == "Canteen".to_string() {
                self.parapherna[jj].examine_text = Some(vec![vec![
                    "A resealable vessel filled with refreshing water.".to_string(),
                ]]);
            }
            if self.parapherna[jj].name == "Fruitcake".to_string() {
                self.parapherna[jj].examine_text = Some(vec![vec![
                    "A tin of homebaked fruitcake from Grandma.".to_string(),
                ]]);
            }
            if self.parapherna[jj].name == "Spectacles".to_string() {
                self.parapherna[jj].examine_text = Some(vec![vec![
                    "Prescription wire-rim eyeglasses.".to_string(),
                    "Essential for reading correspondence, inscriptions, etc.".to_string(),
                ]]);
            }
            if self.parapherna[jj].name == "Sketchbook".to_string() {
                self.parapherna[jj].examine_text = Some(vec![vec![
                    "Blank paper and pen ready to record your observations.".to_string(),
                ]]);
            }
        }
    }
    pub fn fill_maze(&mut self, maze: &Amaze) { //  alcove center = 104
        let stuff: Vec<(String, usize, Color)> = vec![
            ("Ariadne's Thread".to_string(), maze.solutions[0][0], GOLD),
            ("Compass".to_string(), maze.solutions[0][3], GREEN),
            ("Violet Glasses".to_string(), maze.solutions[0][6], VIOLET),
            ("Astrolabe".to_string(), maze.solutions[0][9], GOLD),
            ("Flask".to_string(), maze.solutions[0][12], BLUE),
        ];
        for ii in 0..stuff.len() {
            self.parapherna.push(Parapherna::new(
                stuff[ii].0.clone(), 
                stuff[ii].1, 
                Disposition::InMaze, 
                104, 
                stuff[ii].2
            ));
        }
        for jj in 0..self.parapherna.len() {
            if self.parapherna[jj].name == "Ariadne's Thread".to_string() {
                self.parapherna[jj].examine_text = Some(vec![vec![
                    "Golden thread wound on a wooden spool.".to_string(),
                ]]);
            }
            if self.parapherna[jj].name == "Compass".to_string() {
                self.parapherna[jj].examine_text = Some(vec![vec![
                    "An ordinary magnetic compass with a lanyard".to_string(),
                    "allowing it to be worn around the neck.".to_string(),
                ]]);
            }
            if self.parapherna[jj].name == "Violet Glasses".to_string() {
                self.parapherna[jj].examine_text = Some(vec![vec![
                    "A pair of glasses with purple lenses.".to_string(),
                ]]);
            }
            if self.parapherna[jj].name == "Astrolabe".to_string() {
                self.parapherna[jj].examine_text = Some(vec![vec![
                    "A brass instrument for measuring angles.".to_string(),
                ]]);
            }
            if self.parapherna[jj].name == "Flask".to_string() {
                self.parapherna[jj].examine_text = Some(vec![vec![
                    "A glass flask filled with a dark blue viscous fluid.".to_string(),
                ]]);
            }
        }
    }
    pub fn update(&mut self, theseus: &mut Theseus, maze: &Amaze) {
        let compartment: Vec<u8> = vec![77, 78, 79, 96, 97, 98, 115, 116, 117, 134, 135, 136];
        let mut compartment_index: usize = 0;
        let alcove_compartment: Vec<u8> = vec![122, 123, 124, 105, 104, 103, 85, 84, 86, 161, 160, 162, 159, 163, 158, 164];
        let mut alcove_index: usize = 0;
        self.something_illuminated = 0;
        theseus.ariadne = false;
        if maze.has_window(theseus.chamber) {self.something_illuminated = 9};
        for ii in 0..self.parapherna.len() {
            self.parapherna[ii].active = false;
            if self.parapherna[ii].name == "Ariadne's Thread".to_string()
                && (self.parapherna[ii].disposition != Disposition::InLeftHand
                && self.parapherna[ii].disposition != Disposition::InRightHand)
                    
            {
                self.parapherna[ii].disposition = Disposition::InMaze;
            }
            if self.parapherna[ii].disposition == Disposition::OnHead
                || self.parapherna[ii].disposition == Disposition::OnNeck
                || self.parapherna[ii].disposition == Disposition::InLeftHand
                || self.parapherna[ii].disposition == Disposition::InRightHand
                || (self.parapherna[ii].disposition == Disposition::InMaze && self.parapherna[ii].name == "Lantern".to_string())
            {
                self.parapherna[ii].active = true;
            }
            if self.parapherna[ii].active {
                if self.parapherna[ii].name == "Lantern".to_string() && self.parapherna[ii].chamber == theseus.chamber {
                    if self.something_illuminated < 12 { self.something_illuminated = 12; };
                }
            }
            if self.parapherna[ii].disposition != Disposition::InMaze {
                self.parapherna[ii].chamber = theseus.chamber;
                self.parapherna[ii].direction = theseus.direction;
                if self.parapherna[ii].name == "Ariadne's Thread".to_string() {
                    self.parapherna[ii].active = true;
                    theseus.ariadne = true;
                }
            }
            if self.parapherna[ii].disposition == Disposition::InPack {
                self.parapherna[ii].tile = compartment[compartment_index];
                compartment_index += 1;
                compartment_index %= compartment.len();
            }
            if self.parapherna[ii].disposition == Disposition::InMaze {
                for jj in 0..6 {
                    if maze.rooms[self.parapherna[ii].chamber][jj] == 2 { self.parapherna[ii].direction = jj as u8; };
                }
                for kk in 0..6 {
                    if maze.rooms[self.parapherna[ii].chamber][kk] == 3 { self.parapherna[ii].direction = kk as u8; };
                }
                self.parapherna[ii].tile = alcove_compartment[alcove_index];
                alcove_index += 1;
                alcove_index %= alcove_compartment.len();
            }
        }
    }
    pub fn draw_paraphernalia(&self, theseus: &Theseus) {
        //let colour: Color = Color::new(0.5, 0.5, 0.5, 0.5);
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
            1.0,
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
            2.25 * TILE,
            2.125 * TILE,
            2.125 * TILE,
            2.5 * TILE,
            1.0,
            GRAY,
        );
        draw_line( // neck
            2.75 * TILE,
            2.125 * TILE,
            2.875 * TILE,
            2.5 * TILE,
            1.0,
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
        draw_text("RUCKSACK", 1.875 * TILE, 3.875 * TILE, 16.0, LIGHTGRAY);
        let (mx, my) = mouse_position();
        let hover_tile_x: u8 = (mx / TILE).trunc() as u8;
        let hover_tile_y: u8 = (my / TILE).trunc() as u8;
        let hover_tile_index: u8 = (hover_tile_y * 19) + hover_tile_x;
        for ii in 0..self.parapherna.len() {
            if self.parapherna[ii].chamber == theseus.chamber
                && self.parapherna[ii].direction == theseus.direction
                && self.selected == None
                && hover_tile_index == self.parapherna[ii].tile
            {
                draw_text(&self.parapherna[ii].name, 1.0 * TILE, 10.125 * TILE, 16.0, GRAY);
            }
            if self.parapherna[ii].chamber == theseus.chamber && self.parapherna[ii].direction == theseus.direction {
                let (col, row) = self.tile_2_cr(self.parapherna[ii].tile);
                match self.parapherna[ii].name.as_str() {
                    "Lantern" => self.draw_lantern(self.parapherna[ii].tile, self.parapherna[ii].active),
                    "Canteen" => self.draw_canteen(self.parapherna[ii].tile, self.parapherna[ii].active),
                    "Fruitcake" => self.draw_fruitcake(self.parapherna[ii].tile),
                    "Spectacles" => self.draw_spectacles(self.parapherna[ii].tile),
                    "Ariadne's Thread" => self.draw_spool(self.parapherna[ii].tile),
                    "Compass" => self.draw_compass(theseus, self.parapherna[ii].tile, self.parapherna[ii].active),
                    "Violet Glasses" => self.draw_violet_glasses(self.parapherna[ii].tile),
                    "Correspondence" => self.draw_correspondence(self.parapherna[ii].tile),
                    "Sketchbook" => self.draw_sketchbook(self.parapherna[ii].tile),
                    "Bandana" => self.draw_bandana(self.parapherna[ii].tile),
                    _ => {
                        draw_rectangle(
                            (col + 0.125) * TILE,
                            (row + 0.125) * TILE,
                            0.75 * TILE,
                            0.75 * TILE,
                            self.parapherna[ii].colour,
                        );
                    },
                }
                if self.selected == Some(ii) {
                    draw_rectangle_lines(
                        col * TILE,
                        row * TILE,
                        TILE,
                        TILE,
                        2.0,
                        GOLD,
                    );
                    draw_text(&self.parapherna[ii].name, 1.0 * TILE, 10.125 * TILE, 16.0, GOLD);
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
    fn draw_sketchbook(&self, tile_index: u8) {
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
    fn draw_bandana(&self, tile_index: u8) {
        let (ulx, uly) = self.tile_2_xy(tile_index);
        draw_circle_lines(
            ulx + 24.0,
            uly + 18.0,
            14.0,
            4.0,
            RED,
        );
        draw_circle(
            ulx + 24.0,
            uly + 5.0,
            4.0,
            RED,
        );
        let v1: Vec2 = Vec2::new(ulx + 7.0, uly + 18.0);
        let v2: Vec2 = Vec2::new(ulx + 24.0, uly + 24.0);
        let v3: Vec2 = Vec2::new(ulx + 20.0, uly + 44.0);
        draw_triangle(v1, v2, v3, RED);
        let v1: Vec2 = Vec2::new(ulx + 41.0, uly + 18.0);
        let v2: Vec2 = Vec2::new(ulx + 24.0, uly + 24.0);
        let v3: Vec2 = Vec2::new(ulx + 20.0, uly + 44.0);
        draw_triangle(v1, v2, v3, RED);
        draw_poly(
            ulx + 24.0,
            uly + 32.0,
            3,
            12.0,
            -15.0,
            RED,
        );
        draw_line(
            ulx + 24.0,
            uly + 6.0,
            ulx + 36.0,
            uly + 3.0,
            2.0,
            RED,
        );
        draw_line(
            ulx + 24.0,
            uly + 6.0,
            ulx + 14.0,
            uly + 3.0,
            2.0,
            RED,
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
            RED,
        );
        if active {
            match theseus.direction {
                0 => draw_text("< EAST >", TILE + 44.0, 0.625 * TILE, 16.0, GRAY),
                1 => draw_text("< SOUTH >", TILE + 40.0, 0.625 * TILE, 16.0, GRAY),
                2 => draw_text("< WEST >", TILE + 44.0, 0.625 * TILE, 16.0, GRAY),
                3 => draw_text("< NORTH >", TILE + 40.0, 0.625 * TILE, 16.0, GRAY),
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
        let y: f32 = (idx as f32 / 19.0).trunc() * TILE;
        let x: f32 = (idx % 19) as f32  * TILE;
        (x, y)
    }
}