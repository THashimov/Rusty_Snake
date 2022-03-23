use std::convert::TryInto;

use sdl2::{pixels::Color, rect::Rect, render::Canvas, video::Window, Sdl};

pub struct WindowManager {
    pub canvas: Canvas<Window>,
}

impl WindowManager {
    pub fn new_window(sdl_context: &Sdl) -> Self {
        let v_subsys = sdl_context.video().unwrap();
        let window = v_subsys
            .window("Rusty Snake", 640, 320)
            .position_centered()
            .build()
            .unwrap();

        let canvas = window.into_canvas().build().unwrap();

        WindowManager { canvas: canvas }
    }

    pub fn update_screen(&mut self, bitmap: &[[u8; 64]; 32]) {
        for (y, row) in bitmap.iter().enumerate() {
            for (x, _col) in row.iter().enumerate() {
                if bitmap[y][x] == 1 {
                    let x_coord: i32 = x.try_into().unwrap();
                    let y_coord: i32 = y.try_into().unwrap();

                    self.canvas.set_draw_color(Color::RGB(0, 255, 0));

                    self.canvas
                        .fill_rect(Rect::new(x_coord * 10, y_coord * 10, 8, 8))
                        .unwrap();
                }
            }
        }
        self.canvas.present();
    }
}
