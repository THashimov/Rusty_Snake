#[derive(Debug)]
pub struct GameBoard {
    pub game_board: [[u8; 64]; 32],
}

impl GameBoard {
    pub fn new_game_board() -> Self {
        let game_board = [[0; 64]; 32];

        GameBoard {
            game_board: game_board,
        }
    }
}
