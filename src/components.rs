#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Illumination(pub u8);

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Chamber(pub usize);

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PurpleEmmission(pub bool);
/*
should these items be entities, components, systems or other?
start adventure with these:
    canteen
    fruitcake (specifically 1980s MRE orange nut cake)
    literature (telegrams, letters, diaries, notebooks, postcards, references, etc.)
    lantern
pick these up in labyrinth:
    ariadne's thread
    compass
    magnifying glass
    astrolabe
    ?

*/

/* not used

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct InHand(pub bool);

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct InPack(pub bool);

#[derive(Clone)]
pub struct Sprite {
    pub sprite_sheet: Vec<Texture2D>,
    pub tl_delta: Vec2, // Vec2 added to body.center to locate sprite (usually negative values)
    pub blit: usize,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Here(pub usize);

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Name(pub &'static str);

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Room;

*/