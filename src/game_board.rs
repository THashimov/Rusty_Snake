use rand::Rng;


#[derive(Debug)]
pub struct GameBoard {
    pub game_board: [[u8; 64]; 32],
    pub food_coord: [i8; 2]
}

impl GameBoard {
    pub fn new_game_board() -> Self {
        let game_board = [[0; 64]; 32];
        let food_coord: [i8; 2] = [50, 30];

        GameBoard {
            game_board: game_board,
            food_coord: food_coord
        }
    }

    pub fn new_food_coords(&mut self) {
        let mut arr: [i8; 2] = [0, 0];
        let mut rng = rand::thread_rng();

        let food_x: i8 = rng.gen_range(0..64);
        let food_y: i8 = rng.gen_range(0..32);

        arr[0] = food_x;
        arr[1] = food_y;

        self.food_coord = arr;
    }
}
