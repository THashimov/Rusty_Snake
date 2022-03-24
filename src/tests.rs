#[cfg(test)]
mod tests {
    use crate::snake_controller::Snake;

    #[test]
    fn new_snake() {
        let snake = Snake::new_snake();
        let mut snake_len = vec![];

        for i in 0..3 {
            let mut coord_to_push = [0i8; 2];
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
    fn grow_snake() { 
        let snake = Snake::new_snake();

        let index = snake.snake_len.len() - 1;
        
        println!("{:?}", snake.snake_len);
        
        println!("{:?}", snake.snake_len[index])
    }

    #[test]
    fn check_for_collision_with_self() {
        let mut snake = Snake::new_snake();

        snake.head_coord = [51, 15];
        println!("{:?}", snake.snake_len);

        for i in 1..snake.snake_len.len() {
            if snake.snake_len[i] == snake.head_coord
            {
                println!("ded");
            }
        }
    }
}
