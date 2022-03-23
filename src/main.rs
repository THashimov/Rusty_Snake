mod game_board;
mod snake_controller;
mod window_manager;
mod tests;

use sdl2::pixels::Color;
use sdl2::{event::Event, keyboard::Keycode};

use crate::snake_controller::{Direction, Snake};
use crate::window_manager::WindowManager;

fn main() {
    let sdl_context = sdl2::init().unwrap();
    let mut window = WindowManager::new_window(&sdl_context);
    let mut event_pump = sdl_context.event_pump().unwrap();
    let mut snake = Snake::new_snake();

    'running: loop {
        window.canvas.set_draw_color(Color::RGB(0, 40, 40));
        window.canvas.clear();
        window.canvas.present();

        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                Event::KeyDown {
                    keycode: Some(Keycode::Up),
                    ..
                } => snake.change_snake_direction(Direction::Up),
                Event::KeyDown {
                    keycode: Some(Keycode::Down),
                    ..
                } => snake.change_snake_direction(Direction::Down),
                Event::KeyDown {
                    keycode: Some(Keycode::Left),
                    ..
                } => snake.change_snake_direction(Direction::Left),
                Event::KeyDown {
                    keycode: Some(Keycode::Right),
                    ..
                } => snake.change_snake_direction(Direction::Right),
                _ => {}
            }
        }

        snake.move_snake();
        snake.update_snake_coords();
        window.update_screen(&snake.game_board.game_board);
        std::thread::sleep(std::time::Duration::from_millis(100))
    }
}
