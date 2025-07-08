// use dotenv::dotenv;
// use std::env;
use sdl2::rect::Rect;
use sdl2::pixels::Color;
use sdl2::render::Canvas;
use sdl2::video::Window;


pub struct Background {
    pub screen_area: Rect,
    pub clear_color: Color,
}

impl Background {
    pub fn render(&self, canvas: &mut Canvas<Window>) {
        let _ = canvas.set_draw_color(self.clear_color);
        let _ = canvas.fill_rect(self.screen_area);
    }
}
