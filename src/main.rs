use macroquad::{
    //audio,
    //audio::Sound,
    //color::Color,
    //color::colors::*,
    //input::*,
    //math::*,
    //shapes::*,
    //text::*,
    //texture::*,
    //ui::root_ui,
    window::*,
};

mod entities;
mod components;
mod systems;

use crate::systems::*;
use crate::entities::*;

const MAZE_WIDTH: usize = 25;
const MAZE_DEPTH: usize = 25;
const MAZE_LEVEL: usize = 25;
const START_CELL: usize = MAZE_WIDTH + 1;

//macroquad window initialization
fn window_configuration() -> Conf {
    Conf {
        window_title: "P O R T A L".to_owned(),
        window_width: 912,
        window_height: 528,
        window_resizable: false,
        ..Default::default()
    }
}

#[macroquad::main(window_configuration)]
async fn main() {
    // load assets - AFAIK must be done within async main
    // let mut game_sound: Vec<Sound> = Vec::new();
    // game_sound.push(audio::load_sound("assets/audio/splat.ogg").await.unwrap());
    // game_sound.push(audio::load_sound("assets/audio/sound_view_focused_or_selected.ogg").await.unwrap());
    // initialize systems
    let mut info = Info::default();
    let mut stage = Stage::new(screen_width(), screen_height());
    let mut maze = Amaze::new(MAZE_WIDTH, MAZE_DEPTH, MAZE_LEVEL);
    maze.create_maze(START_CELL);
    // initialize entities
    let mut theseus = Theseus::new(START_CELL);
    let mut paraphernalia= Paraphernalia::new();
    paraphernalia.fill_rucksack(START_CELL);
    paraphernalia.fill_maze(START_CELL);
    eprintln!("Starting game loop...");
    loop { // macroquad game loop
        // get keyboard/mouse interface
        let (_play_sound, _sound_index) = stage.interface(&mut maze, &mut theseus, &mut paraphernalia);
        // if play_sound { audio::play_sound_once(game_sound[sound_index]); };
        paraphernalia.update(&mut theseus, &maze);
        // draw theseus view
        stage.update_stage(&mut info, &mut maze, &mut theseus, &paraphernalia);
        if stage.user_decided_to_exit {
			eprintln!("...Ending game loop");
            break;
        }
        next_frame().await;
    }
}
/*
if not already installed on linux OS:
apt install libx11-dev libxi-dev libgl1-mesa-dev libasound2-dev

cargo build --target wasm32-unknown-unknown
copy 'mq_js_bundle.js' and 'index.html' templates into crate folder
change TITLE and 'load("mquad.wasm")' to crate name
copy [crate name].wasm from ./target/wasm-unknown-unknown/debug/ or ../release/
basic-http-server
http://127.0.0.1:4000
*/