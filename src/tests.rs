#[cfg(test)]
mod tests {
    use crate::snake_controller::Snake;

    #[test]
    fn new_snake() {
        let snake = Snake::new_snake();
        let mut snake_len = vec![];

        for i in 0..3 {
            let mut coord_to_push = [0u8; 2];
            coord_to_push[0] = snake.head_coord[0] + i;
            coord_to_push[1] = snake.head_coord[1];

            snake_len.push(coord_to_push);
        }

        println!("{:?}", snake_len);
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
        assert_eq!(snake.game_board.game_board[15][51], 1);
        assert_eq!(snake.game_board.game_board[15][52], 1);
    }

    #[test]
    fn move_left() {
        let mut snake = Snake::new_snake();

        // Decrease head coord by 1 to move left
        // Increase to move right
        snake.head_coord[0] = snake.head_coord[0] - 1;

        // Insert new head coord
        snake.snake_len.insert(0, snake.head_coord);

        snake.tail_coord = snake.snake_len.pop().unwrap();

        snake.game_board.game_board[snake.tail_coord[0] as usize][snake.tail_coord[1] as usize] = 0;
    }

    #[test]
    fn move_up() {
        let mut snake = Snake::new_snake();

        snake.head_coord[0] = snake.head_coord[0] - 1;

        // Insert new head coord
        snake.snake_len.insert(0, snake.head_coord);

        snake.tail_coord = snake.snake_len.pop().unwrap();

        snake.game_board.game_board[snake.tail_coord[0] as usize][snake.tail_coord[1] as usize] = 0;

    }
}
