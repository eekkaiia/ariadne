#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Theseus {
    pub direction: u8, // [E, S, W, N, U, D]
    pub chamber: usize,
    pub ariadne: bool,
}
impl Theseus {
    pub fn new(chamber: usize) -> Self {
        Self {
            direction: 0,
            chamber,
            ariadne: false,
        }
    }
}