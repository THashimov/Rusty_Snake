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
    pub head_coord: [u8; 2],
    pub tail_coord: [u8; 2],
    pub snake_len: Vec<[u8; 2]>,
    direction: Direction,
}

impl Snake {
    pub fn new_snake() -> Self {
        let head_coord = [50, 15];
        let mut snake_len = vec![];

        for i in 0..3 {
            let mut coord_to_push = [0u8; 2];
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
        }
    }

    pub fn change_snake_direction(&mut self, direction: Direction) {
        if (direction == Direction::Right || direction == Direction::Left)
            && (self.direction == Direction::Left || self.direction == Direction::Right)
        {
        } else if (direction == Direction::Up || direction == Direction::Down)
            && (self.direction == Direction::Down || self.direction == Direction::Up)
        {
        } else {
            self.direction = direction
        };
    }

    pub fn update_snake_coords(&mut self) {
        for i in 0..self.snake_len.len() {
            let coords = self.snake_len[i];
            let x_coord = coords[0] as usize;
            let y_coord = coords[1] as usize;

            self.game_board.game_board[y_coord][x_coord] = 1;
        }
    }

    pub fn move_snake(&mut self) {
        match &self.direction {
            Direction::Up => self.move_up(),
            Direction::Down => self.move_down(),
            Direction::Left => self.move_left(),
            Direction::Right => self.move_right(),
        }
    }

    fn move_up(&mut self) {
    }

    fn move_down(&mut self) {
    }

    fn move_left(&mut self) {
        println!("{:?}", self.snake_len);

        self.head_coord[0] = self.head_coord[0] - 1;

        self.snake_len.insert(0, self.head_coord);
        let tail_coord = self.snake_len.pop().unwrap();

        self.game_board.game_board[tail_coord[1] as usize][tail_coord[0] as usize] = 0;
    }

    fn move_right(&mut self) {
        println!("{:?}", self.snake_len);
        self.head_coord[0] = self.head_coord[0] + 1;

        self.snake_len.insert(0, self.head_coord);
        let tail_coord = self.snake_len.pop().unwrap();

        self.game_board.game_board[tail_coord[1] as usize][tail_coord[0] as usize] = 0;
    }
}
