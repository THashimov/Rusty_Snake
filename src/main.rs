mod game_board;
mod snake_controller;
mod tests;
mod window_manager;

use sdl2::EventPump;
use sdl2::pixels::Color;
use sdl2::{event::Event, keyboard::Keycode};

use crate::snake_controller::{Direction, Snake};
use crate::window_manager::WindowManager;

fn main() {
    let sdl_context = sdl2::init().unwrap();
    let mut window = WindowManager::new_window(&sdl_context);
    let mut event_pump = sdl_context.event_pump().unwrap();
    let mut snake = Snake::new_snake();

    while !snake.game_over {
        window.canvas.set_draw_color(Color::RGB(0, 40, 40));
        window.canvas.clear();
        window.canvas.present();

        scan_for_key_press(&mut event_pump, &mut snake);

        snake.update_snake_coords();
        window.update_screen(&snake.game_board.game_board);
        snake.move_snake();
        if snake.has_food_been_eaten() {
            snake.grow_snake()
        };
        
        std::thread::sleep(std::time::Duration::from_millis(50))
    }

    'running: loop {
        if scan_for_key_press(&mut event_pump, &mut snake) {
            break 'running
        }
    }

}

fn scan_for_key_press(event_pump: &mut EventPump, snake: &mut Snake) -> bool {
    for event in event_pump.poll_iter() {
        match event {
            Event::Quit { .. }
            | Event::KeyDown {
                keycode: Some(Keycode::Escape),
                ..
            } => {
                snake.game_over = true;
                return true
            },
            Event::KeyDown {
                keycode: Some(Keycode::Up),
                ..
            } => {
                if snake.direction != Direction::Down {
                    snake.direction = Direction::Up
                }
            }
            Event::KeyDown {
                keycode: Some(Keycode::Down),
                ..
            } => {
                if snake.direction != Direction::Up {
                    snake.direction = Direction::Down
                }
            }
            Event::KeyDown {
                keycode: Some(Keycode::Left),
                ..
            } => {
                if snake.direction != Direction::Right {
                    snake.direction = Direction::Left
                }
            }
            Event::KeyDown {
                keycode: Some(Keycode::Right),
                ..
            } => {
                if snake.direction != Direction::Left {
                    snake.direction = Direction::Right
                }
            }
            _ => {}
        }
    }
    return false
}
