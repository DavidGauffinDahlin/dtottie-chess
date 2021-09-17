#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

#[macro_use]
pub mod macros;

pub mod pieces;
use pieces::Piece;

pub struct Game {
    pub board: [[Piece; 8]; 8],
    pub score: (u8, u8),
    pub turn_white: bool,
}

impl Game {
    fn new_game() -> Game {
        let turn_white = true;
        let board = Game::construct_board();
        let score = (0, 0);
        Game {
            board,
            score,
            turn_white,
        }
    }
    fn construct_board() -> [[Piece; 8]; 8] {
        let mut board: [[Piece; 8]; 8] = Default::default();
        let pieces = ["r", "k", "b", "q", "x", "b", "k", "r"];

        for i in 0..8 {
            board[0][i] = Piece::new_piece(&["b", pieces[i]].join(""));
            board[1][i] = Piece::new_piece("bp");

            board[6][i] = Piece::new_piece("wp");
            board[7][i] = Piece::new_piece(&["w", pieces[i]].join(""));
        }
        board
    }
}
