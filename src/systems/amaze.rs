/*  Iterative implementation of depth-first search maze generation
    Choose the initial cell, mark it as visited and push it to the stack
    While the stack is not empty
        Pop a cell from the stack and make it a current cell
        If the current cell has any neighbours which have not been visited
            Push the current cell to the stack
            Choose one of the unvisited neighbours
            Remove the panel between the current cell and the chosen cell
            Mark the chosen cell as visited and push it to the stack
*/

use macroquad::{
    // audio,
    color::Color,
    color::colors::*,
    math::*,
    rand::*,
    // shapes::*,
    // text::*,
    texture::*,
    time::*,
    // window::*,
    // ui::root_ui,
};

use crate::entities::Theseus;

/// 3D maze 
#[derive(Clone, Debug, PartialEq)]
pub struct Amaze {
    pub width: usize,
    pub depth: usize,
    pub level: usize,
    pub start: Vec<usize>,  // starting cell for each level
    pub end: Vec<usize>,    // end of maze for each level
    pub rooms: Vec<[u8; 6]>, // [E, S, W, N, U, D] with 0 = plain wall, 1 = portal, 2 = window, 3 = alcove, tbd
    pub visited: Vec<bool>,
    pub solutions: Vec<Vec<usize>>,
    pub thread: Vec<usize>, // thread is a stack for tracking ariadne's method of returning to starting point
    pub level_sheets: Vec<Vec<Texture2D>>, // maze image for each level at different sizes
    pub texture_cell_sizes: Vec<usize>,
}
impl Amaze {
    pub fn new(width: usize, depth: usize, level: usize) -> Self {
        let start: Vec<usize> = Vec::new();
        let end: Vec<usize> = Vec::new();
        let rooms: Vec<[u8; 6]> = Vec::new(); // [E, S, W, N, U, D] with 0 = plain wall, 1 = portal, 2 = window, 3 = alcove, tbd
        let mut visited: Vec<bool> = Vec::new();
        for _ in 0..width * depth * level {
            visited.push(false);
        }
        let solutions: Vec<Vec<usize>> = Vec::new();
        let thread: Vec<usize> = Vec::new();
        let level_sheets: Vec<Vec<Texture2D>> = Vec::new();        
        let texture_cell_sizes: Vec<usize> = vec![5, 15, 20];
        Self {
            width,
            depth,
            level,
            start,
            end,
            rooms,
            visited,
            solutions,
            thread,
            level_sheets,
            texture_cell_sizes,
        }
    }
    /// create_maze() generates a 3D maze of cells
    pub fn create_maze(&mut self, starting: usize) {
        srand((get_time() * 64556.0).trunc() as u64);
        let mut start_cell = starting % (self.width * self.depth); // ensures starting point is on level 0 - change later?
        self.thread.push(99_999);
        for kk in 0..self.level {
            let (h_panels, v_panels, l_end) = self.generate_panels(start_cell, kk);
            // convert amaze 'panels' to maze 'walls' 0 = plain wall, 1 = portal, 2 = window, 3 = alcove, tbd
            for jj in 0..self.width * self.depth {
                let mut room: [u8; 6] = [0; 6];
                let mut empty_walls: Vec<usize> = Vec::new();
                let (col, row) = self.idx_to_xy(jj);
                // East
                if col as usize == self.width - 1  {
                    room[0] = 2;
                } else if !v_panels[jj + 1] {
                    room[0] = 1;
                } else {
                    empty_walls.push(0);
                }
                // South
                if row as usize == self.depth - 1 {
                    room[1] = 2;
                } else if !h_panels[jj + self.width] {
                    room[1] = 1;
                } else {
                    empty_walls.push(1);
                }
                // West
                if col as usize == 0 {
                    room[2] = 2;
                } else if !v_panels[jj] {
                    room[2] = 1;
                } else {
                    empty_walls.push(2);
                };
                // North
                if row as usize == 0 {
                    room[3] = 2;
                } else if !h_panels[jj] {
                    room[3] = 1
                } else {
                    empty_walls.push(3);
                }
                if empty_walls.len() > 0 {
                    let random_wall = gen_range::<usize>(0, empty_walls.len());
                    room[empty_walls[random_wall]] = 3;
                }
                self.rooms.push(room);
            }
            self.level_sheets.push(self.paint(h_panels, v_panels));
            start_cell = l_end;
        }
    }
    /// generate_panels() removes adjoining walls to create maze
    fn generate_panels(&mut self, start_cell: usize, l_level: usize) -> (Vec<bool>, Vec<bool>, usize) {
        let mut h_panels: Vec<bool> = Vec::new();
        let mut v_panels: Vec<bool> = Vec::new();
        let mut l_stack: Vec<usize> = Vec::new();
        let mut l_solution: Vec<usize> = Vec::new();
        let mut l_visited: Vec<bool> = Vec::new();
        for _ in 0..self.depth {
            for _ in 0..self.width {
                h_panels.push(true);
                v_panels.push(true);
                l_visited.push(false);
            }
        }
        let mut l_end: usize = 0;
        srand((get_time() * 64556.0).trunc() as u64);
        l_visited[start_cell] = true;
        l_stack.push(start_cell);
        let mut deepest: usize = 0;
        while !l_stack.is_empty() {
            let current = l_stack.pop().expect("stack is None");
            let good_neighbors = self.check_neighbors(current, &mut l_visited);
            // if good_neighbors != 0 push current to stack
            if !good_neighbors.is_empty() {
                l_stack.push(current);
                // randomly select one good neighbor
                let random_neighbor = gen_range::<usize>(0, good_neighbors.len());
                // remove panel between current and new
                match good_neighbors[random_neighbor].1 {
                    'N' => h_panels[current] = false,
                    'E' => v_panels[current + 1] = false,
                    'S' => h_panels[current + self.width] = false,
                    'W' => v_panels[current] = false,
                    _ => eprintln!("!!!Unexpected char: {:?}", &good_neighbors[random_neighbor].1),
                }
                // mark the chosen cell as visited and push it to the stack
                l_visited[good_neighbors[random_neighbor].0] = true;
                l_stack.push(good_neighbors[random_neighbor].0);
            }
            if l_stack.len() > deepest {
                l_solution = l_stack.clone();
                l_end = l_solution[l_solution.len() - 1];
                deepest = l_stack.len();
            }
        }
        self.start.push((self.width * self.depth * l_level) + start_cell);
        self.end.push((self.width * self.depth * l_level) + l_end);
        self.solutions.push(l_solution.clone());
        
        (h_panels, v_panels, l_end)
    }
    /// check_neighbors() validates that adjoining cell exists & is unvisited
    fn check_neighbors(&self, idx: usize, l_visited: &mut Vec<bool>) -> Vec<(usize, char)> {
        let mut neighbors: Vec<(usize, char)> = Vec::new();
        //north
        if idx as i32 - (self.width as i32) >= 0 {
            if !l_visited[idx - self.width] {
                neighbors.push((idx - self.width, 'N'));
            }
        }
        // east
        if (idx + 1) % self.width != 0 && !l_visited[idx + 1] {
            neighbors.push((idx + 1, 'E'));
        }
        // south
        if idx + self.width < self.width * self.depth && !l_visited[idx + self.width] {
            neighbors.push((idx + self.width, 'S'));
        }
        // west
        if idx % self.width != 0 && !l_visited[idx - 1] {
            neighbors.push((idx - 1, 'W'));
        }
        neighbors
    }
    /// paint() a macroquad texture2D from maze struct
    fn paint(&self, h_panels: Vec<bool>, v_panels: Vec<bool>) -> Vec<Texture2D> {
        let mut colour: Color = GRAY;
        let mut pictures: Vec<Texture2D> = Vec::new();
        for nn in 0..self.texture_cell_sizes.len() {
            let size = self.texture_cell_sizes[nn];
            if size < 8 { colour =  Color::new(0.5, 0.5, 0.5, 0.5); };
            let image_width = (size * self.width) + 1;
            let image_depth = (size * self.depth) + 1;
            let mut maze_painting = Image::gen_image_color(
                image_width.try_into().expect("usize to big"),
                image_depth.try_into().expect("usize to big"),
                BLANK);
            // draw maze
            for ii in 0..h_panels.len() {
                if h_panels[ii] {
                    let (col, row) = self.idx_to_xy(ii);
                    for jj in 0..size {
                        maze_painting.set_pixel((col * size as u32) + jj as u32, row * size as u32, colour);
                    }
                }
            }
            for ii in 0..v_panels.len() {
                if v_panels[ii] {
                    let (col, row) = self.idx_to_xy(ii);
                    for jj in 0..size {
                        maze_painting.set_pixel(col * size as u32, (row * size as u32) + jj as u32, colour);
                    }
                }
            }
            // draw border around maze
            for kk in 0..image_width {
                maze_painting.set_pixel(kk as u32, 0, colour);
                maze_painting.set_pixel(kk as u32, image_depth as u32 - 1, colour);
            }
            for ll in 0..image_depth {
                maze_painting.set_pixel(0, ll as u32, colour);
                maze_painting.set_pixel(image_width as u32 - 1, ll as u32, colour);
            }
            // convert from Image to Texture2D
            pictures.push(Texture2D::from_image(&maze_painting));
        }
        pictures
    }
    /// theseus_move_forward() evalutates a move within maze
    pub fn theseus_move_forward(&mut self, theseus: &mut Theseus) {
        match self.rooms[theseus.chamber][theseus.direction as usize] { // [E, S, W, N, U, D] with 0 = plain wall, 1 = portal, 2 = window, tbd
            0 => (),
            1 => {
                self.visited[theseus.chamber] = true;
                match theseus.direction {
                    0 => {
                        if theseus.ariadne {
                            if theseus.chamber + 1 == self.thread[self.thread.len() - 1] {
                                _ = self.thread.pop();
                            } else {
                                self.thread.push(theseus.chamber);
                            }
                        }
                        theseus.chamber += 1;
                    },
                    1 => {
                        if theseus.ariadne && !self.thread.is_empty() {
                            if theseus.chamber + self.width == self.thread[self.thread.len() - 1] {
                                _ = self.thread.pop();
                            } else {
                                self.thread.push(theseus.chamber);
                            }
                        }
                        theseus.chamber += self.width
                    },
                    2 => {
                        if theseus.ariadne && !self.thread.is_empty() {
                            if theseus.chamber - 1 == self.thread[self.thread.len() - 1] {
                                _ = self.thread.pop();
                            } else {
                                self.thread.push(theseus.chamber);
                            }
                        }
                        theseus.chamber -= 1
                    },
                    3 => {
                        if theseus.ariadne && !self.thread.is_empty() {
                            if theseus.chamber - self.width == self.thread[self.thread.len() - 1] {
                                _ = self.thread.pop();
                            } else {
                                self.thread.push(theseus.chamber);
                            }
                        }
                        theseus.chamber -= self.width
                    },
                    _ => eprintln!("!!!Unexpected direction: {:?}", &theseus.direction),
                }
            },
            2 => (),
            3 => (),
            _ => eprintln!("!!!Unexpected direction: {:?} or chamber: {:?}", &theseus.direction, &theseus.chamber),
        }
    }
    /// idx_to_xyz() converts cell vector index into (x, y, z) coordinates
    pub fn idx_to_xyz(&self, idx: usize) -> (u32, u32, u32) {
        let z = (idx as f32 / (self.width * self.depth) as f32).trunc() as u32;
        let y = ((idx - (z as usize * self.width * self.depth)) as f32 / self.width as f32).trunc() as u32;
        let x = (idx % self.width) as u32;
        (x, y, z)
    }
    /// idx_to_xy() converts cell vector index into (x, y) coordinates
    pub fn idx_to_xy(&self, idx: usize) -> (u32, u32) {
        let y = (idx as f32 / self.width as f32).trunc() as u32;
        let x = (idx % self.width) as u32;
        (x, y)
    }
    pub fn has_window(&self, idx: usize) -> bool {
        let mut pane: bool = false;
        for ii in 0..6 {
            if self.rooms[idx][ii] == 2 { pane = true; };
        }
        pane
    }
}