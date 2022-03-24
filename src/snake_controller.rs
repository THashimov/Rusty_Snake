use crate::game_board::GameBoard;

#[derive(Debug, PartialEq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug)]
pub struct Snake {
    pub game_board: GameBoard,
    pub head_coord: [i8; 2],
    pub tail_coord: [i8; 2],
    pub snake_len: Vec<[i8; 2]>,
    pub direction: Direction,
    pub previous_direction: Direction,
    pub game_over: bool
}

impl Snake {
    pub fn new_snake() -> Self {
        let head_coord: [i8; 2] = [50, 15];
        let mut snake_len: Vec<[i8; 2]> = Vec::new();

        for i in 0..3 {
            let mut coord_to_push = [0i8; 2];
            // Index column (x)
            coord_to_push[0] = head_coord[0] + i;
            // index row (y)
            coord_to_push[1] = head_coord[1];

            snake_len.push(coord_to_push);
        }

        Snake {
            game_board: GameBoard::new_game_board(),
            head_coord: head_coord,
            tail_coord: [0, 0],
            snake_len: snake_len,
            direction: Direction::Left,
            previous_direction: Direction::Left,
            game_over: false
        }
    }

    pub fn update_snake_coords(&mut self) {
        for i in 0..self.snake_len.len() {
            let coords = self.snake_len[i];
            let x_coord = (coords[0] as usize) % 64;
            let y_coord = (coords[1] as usize) % 32;

            self.game_board.game_board[y_coord][x_coord] = 1;
        }
    }

    pub fn move_snake(&mut self) {
        self.draw_food();
        let mut _x_or_y_index = 0;
        let mut _index_direction = 0;

        match self.direction {
            Direction::Up => {
                _x_or_y_index = 1;
                _index_direction = -1;
            }
            Direction::Down => {
                _x_or_y_index = 1;
                _index_direction = 1;
            }
            Direction::Left => {
                _x_or_y_index = 0;
                _index_direction = -1;
            }
            Direction::Right => {
                _x_or_y_index = 0;
                _index_direction = 1;
            }
        };

        // Increase or decrease head coord based on direction
        self.head_coord[_x_or_y_index] += _index_direction;

        // Wrap screen 
        // Found this formula on stack overflow, not really sure how it works
        // Remainder is real tough to understand
        // (x%32 + 32)%32
        self.head_coord[1] = (self.head_coord[1] % 32 + 32) % 32;
        self.head_coord[0] = (self.head_coord[0] % 64 + 64) % 64;

        // Push new head coord into the snake vec
        self.snake_len.insert(0, self.head_coord);

        // Check if snake has eaten itself
        self.check_for_collision_with_self();

        // Remove the last element from the snake vec
        self.tail_coord = self.snake_len.pop().unwrap();

        // Set the tail coord on gameboard to 0 to turn pixel off
        self.game_board.game_board[self.tail_coord[1] as usize][self.tail_coord[0] as usize] = 0;
    }

    fn draw_food(&mut self) {
        let x = self.game_board.food_coord[0];
        let y = self.game_board.food_coord[1];
        self.game_board.game_board[y as usize][x as usize] = 1;
    }    

    pub fn has_food_been_eaten(&mut self) -> bool {
        if self.head_coord == self.game_board.food_coord {
            self.game_board.new_food_coords();
            return true 
        } else {
            return false
        }
    }

    pub fn grow_snake(&mut self) {
        self.snake_len.push(self.tail_coord);
    }
    
    pub fn check_for_collision_with_self(&mut self) {
        for i in 1..self.snake_len.len() {
            if self.snake_len[i] == self.head_coord {
                self.game_over = true
            }
        }
    }
}
