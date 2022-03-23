#[cfg(test)]
mod tests {
    use crate::snake_controller::Snake;

    #[test]
    fn new_snake() {
        let mut snake= Snake::new_snake();

        for i in 0..3 {
            let mut coord_to_push = [0u8; 2];
            coord_to_push[0] = snake.head_coord[0];
            coord_to_push[1] = snake.head_coord[1] - i;

            snake.snake_len.push(coord_to_push);
        }

        println!("{:?}", snake.snake_len);
    }

    #[test]
    fn update_snake_coords() {
        let mut snake = Snake::new_snake();

        for i in 0..snake.snake_len.len() {
            let coords = snake.snake_len[i];
            let x_coord = coords[0] as usize;
            let y_coord = coords[1] as usize;

            snake.game_board.game_board[y_coord][x_coord] = 1;
        }

        assert_eq!(snake.game_board.game_board[15][50], 1);
        assert_eq!(snake.game_board.game_board[15][49], 1);
        assert_eq!(snake.game_board.game_board[15][48], 1);
    }

    #[test]
    fn move_snake() {
        let mut snake = Snake::new_snake();

        // Increase head coord by 1
        snake.head_coord[0] = snake.head_coord[0] + 1;

        // Insert new head coord
        snake.snake_len.insert(0, snake.head_coord);
        
        snake.tail_coord = snake.snake_len.pop().unwrap();

        snake.game_board.game_board[snake.tail_coord[0] as usize][snake.tail_coord[1] as usize] = 0;
    }
}