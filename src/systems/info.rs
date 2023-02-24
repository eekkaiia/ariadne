use macroquad::color::Color;
use macroquad::color::colors::*;
use macroquad::input::*;
use macroquad::math::*;
use macroquad::time::*;
use macroquad::ui::{hash, root_ui, widgets};

#[derive(Clone, Debug)]
pub struct Info {
    pub visible: bool,
    pub mx: f32,
    pub my: f32,
    pub context: String,
    pub current_ft: f32,
    pub longest_ft: f32,
    pub running_ft: [f32; 1024],
    pub ft_idx: usize,
    pub average_ft: f32,
    pub background: Color,
}
impl Info {
    pub fn default() -> Self {
        let running_ft = [0.0; 1024];
        Self {
            visible: false,
            mx: 0.0,
            my: 0.0,
            context: "Macroquad Info".to_string(),
            current_ft: 0.0,
            longest_ft: 0.0,
            running_ft,
            ft_idx: 0,
            average_ft: 0.0,
            background: BLACK,
        }
    }
    /// get_average_ft() calculates the average frame time over the last 32 frames
    fn get_average_ft(&self) -> f32 {
        let mut average: f32 = 0.0;
        for i in 0..1024 {
            average += self.running_ft[i];
        }
        average / 1024.0
    }
    /// update() the info struct this frame
    pub fn update(&mut self) {
        (self.mx, self.my) = mouse_position();
        self.current_ft = get_frame_time();
        if self.current_ft > self.longest_ft {
            self.longest_ft = self.current_ft;
        }
        self.running_ft[self.ft_idx] = self.current_ft;
        if self.ft_idx < 1023 {
            self.ft_idx += 1
        } else {
            self.ft_idx = 0
        }
        self.average_ft = self.get_average_ft();
    }
    /// draw_panel() displays info in window widget
    pub fn draw_panel(&mut self) {
        root_ui().window(hash!(), Vec2::new(18., 19.), Vec2::new(190., 155.), |ui| {
            widgets::Group::new(hash!(), Vec2::new(186., 122.))
                .position(Vec2::new(1., 1.))
                .ui(ui, |ui| {
                    
                    ui.label(Vec2::new(7., 0.), "Frame Times");
                    ui.label(Vec2::new(11., 15.), &format!("FPS:       {:2}", &get_fps()));
                    ui.label(Vec2::new(11., 30.), &format!("Current:   {:8.5}", &self.current_ft));
                    ui.label(Vec2::new(11., 45.), &format!("Average:   {:8.5}", &self.average_ft));
                    ui.label(Vec2::new(11., 60.), &format!("Longest:   {:8.5}", &self.longest_ft));
                    ui.label(Vec2::new(7., 80.), "Mouse Coordinates");
                    ui.label(Vec2::new(11., 95.), &format!("x: {:4}     y: {:4}", &self.mx, &self.my));
                    
                });
            widgets::Group::new(hash!(), Vec2::new(186., 25.))
                .position(Vec2::new(1., 125.))
                .ui(ui, |ui| {
                    let mut grey: f32 = self.background.r;
                    ui.slider(hash!(), " Background", 0f32..1f32, &mut grey);
                    self.background = Color::new(grey, grey, grey, 1.);
                });
            });
    }
}