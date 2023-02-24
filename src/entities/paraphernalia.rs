use macroquad::{
    //audio,
    color::Color,
    color::colors::*,
    //input::*,
    //math::*,
    //shapes::*,
    //text::*,
    //texture::*,
    //ui::root_ui,
    //window::*,
};

use crate::entities::*;
use crate::systems::*;

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
            ("Canteen".to_string(), start, BLUE),
            ("Fruitcake".to_string(), start, RED),
            ("Correspondence".to_string(), start, WHITE),
            ("Journal".to_string(), start, GRAY),
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
        
    }
    pub fn fill_maze(&mut self, start: usize) { //  alcove center = 104
        let stuff: Vec<(String, usize, Color)> = vec![
            ("Ariadne's Thread".to_string(), start, GOLD),
            ("Compass".to_string(), start, GREEN),
            ("Violet Glasses".to_string(), start, VIOLET),
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
    }
    pub fn update(&mut self, theseus: &mut Theseus, maze: &Amaze) {
        let compartment: Vec<u8> = vec![77, 78, 79, 96, 97, 98, 115, 116, 117, 134, 135, 136];
        let mut compartment_index: usize = 0;
        let alcove_compartment: Vec<u8> = vec![122, 123, 124, 105, 104, 103, 85, 84, 86];
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
                    if maze.rooms[self.parapherna[ii].chamber][jj] == 3 { self.parapherna[ii].direction = jj as u8; };
                }
                self.parapherna[ii].tile = alcove_compartment[alcove_index];
                alcove_index += 1;
                alcove_index %= alcove_compartment.len();
            }
        }
    }
}