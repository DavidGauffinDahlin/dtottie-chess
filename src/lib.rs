#[cfg(test)]
mod tests {
    #[test]
    fn illegal_move() {
        let mut game1 = crate::Game::new_game();
        assert!(game1.player_move("D8B8\n".to_owned()).is_err());
    }
    #[test]
    fn passant() {
        let mut game1 = crate::Game::new_game();
        let moves = ["D7D5\n", "E2E3\n", "D5D4\n", "C2C4\n", "D4C3\n"];
        for pmove in moves {
            assert!(game1.player_move(pmove.to_owned()).is_ok());
        }
    }
    #[test]
    fn passant_remove() {
        let mut game1 = crate::Game::new_game();
        let moves = ["D7D5\n", "E2E3\n", "D5D4\n", "C2C4\n", "D4D3\n"];
        for pmove in moves {
            let _ = game1.player_move(pmove.to_owned());
        }
        assert!(game1.board[3][2].passant == false);
    }
    #[test]
    fn check_king_knight() {
        let mut game1 = crate::Game::new_game();
        let moves = ["B8C6\n", "A2A3\n", "C6D4\n", "A3A4\n", "D4C2\n"];
        for pmove in moves {
            let _ = game1.player_move(pmove.to_owned());
        }
        assert!(game1.b_checked);
    }
    #[test]
    fn check_king_other() {
        let mut game1 = crate::Game::new_game();
        let moves = ["E7E6\n", "F2F3\n", "D8H4\n"];
        for pmove in moves {
            let _ = game1.player_move(pmove.to_owned());
        }
        assert!(game1.b_checked);
    }
    #[test]
    fn cant_move_locked_piece() {
        let mut game1 = crate::Game::new_game();
        let moves = ["E7E6\n", "A2A3\n", "D8H4\n"];
        for pmove in moves {
            let _ = game1.player_move(pmove.to_owned());
        }
        assert!(game1.player_move("F2F3\n".to_owned()).is_err());
    }
    #[test]
    fn move_king_clears_lock_and_check() {
        let mut game1 = crate::Game::new_game();
        let moves = ["E7E6\n", "D2D3\n", "D8H4\n", "E1D2\n"];
        for pmove in moves {
            let _ = game1.player_move(pmove.to_owned());
        }
        assert!(game1.board[1][5].locked == 0);
        assert!(!game1.b_checked);
    }
    #[test]
    fn takedown_clears_lock_and_check() {
        let mut game1 = crate::Game::new_game();
        let moves = ["C7C6\n", "D2D3\n", "D8B6\n", "C1D2\n", "B6A5\n", "D2A5\n"];
        for pmove in moves {
            let _ = game1.player_move(pmove.to_owned());
        }
    }
    #[test]
    fn threat_doesnt_lock_all_pieces_bug() {
        let mut game1 = crate::Game::new_game();
        let moves = ["E7E6\n", "D2D3\n", "D8H4\n"];
        for pmove in moves {
            let _ = game1.player_move(pmove.to_owned());
        }
        assert!(game1.board[7][1].locked == 0);
    }
    #[test]
    fn queen_doesnt_lock_from_a5_bug() {
        let mut game1 = crate::Game::new_game();
        let moves = ["C7C6\n", "H2H3\n", "D8A5\n"];
        for pmove in moves {
            let _ = game1.player_move(pmove.to_owned());
        }
        assert!(game1.board[1][3].locked != 0);
    }
    #[test]
    fn queen_cant_move_up_diagonally_from_a5_bug() {
        let mut game1 = crate::Game::new_game();
        let moves = ["C7C6\n", "H2H3\n", "D8A5\n", "H3H4\n", "A5B4\n"];
        for pmove in moves {
            assert!(!game1.player_move(pmove.to_owned()).is_err());
        }
    }
    #[test]
    fn move_aggressive_piece_clears_lock() {
        let mut game1 = crate::Game::new_game();
        let moves = ["C7C6\n", "H2H3\n", "D8A5\n", "H3H4\n", "A5D8\n", "D2D3\n"];
        for pmove in moves {
            let _ = game1.player_move(pmove.to_owned());
        }
        assert!(game1.board[2][3].locked == 0);
    }
    #[test]
    fn can_move_locked_piece_if_several_pieces_are_locked() {
        let mut game1 = crate::Game::new_game();
        let moves = ["E7E6\n", "G2G3\n", "D8H4\n", "G3G4\n"];
        for pmove in moves {
            assert!(game1.player_move(pmove.to_owned()).is_ok());
        }
    }
    #[test]
    fn pawn_cant_move_through_piece() {
        let mut game1 = crate::Game::new_game();
        let moves = ["E7E6\n", "A2A3\n", "D8F6\n"];
        for pmove in moves {
            let _ = game1.player_move(pmove.to_owned());
        }
        assert!(game1.player_move("F7F5\n".to_owned()).is_err());
    }
    #[test]
    fn pawn_can_check_king() {
        let mut game1 = crate::Game::new_game();
        let moves = [
            "D7D5\n", "E2E4\n", "A7A6\n", "E1E2\n", "A6A5\n", "E2E3\n", "D5D4\n",
        ];
        for pmove in moves {
            let _ = game1.player_move(pmove.to_owned());
        }
        assert!(game1.b_checked);
    }
    #[test]
    fn king_cant_move_forward_glitch() {
        let mut game1 = crate::Game::new_game();
        let moves = ["D7D5\n", "E2E4\n", "A7A6\n", "E1E2\n"];
        for pmove in moves {
            assert!(game1.player_move(pmove.to_owned()).is_ok());
        }
    }
    #[test]
    fn pawn_can_threaten_tile() {
        let mut game1 = crate::Game::new_game();
        let moves = ["D7D5\n", "E2E4\n", "D5D4\n", "E1E2\n", "A7A6\n"];
        for pmove in moves {
            let _ = game1.player_move(pmove.to_owned());
        }
        assert!(game1.player_move("E2E3\n".to_owned()).is_err());
    }
    #[test]
    fn king_checks_for_threatening_knight() {
        let mut game1 = crate::Game::new_game();
        let moves = ["B8C6\n", "E2E3\n", "C6D4\n"];
        for pmove in moves {
            let _ = game1.player_move(pmove.to_owned());
        }
        assert!(game1.player_move("E1E2\n".to_owned()).is_err());
    }
    #[test]
    fn king_can_threaten_tile() {
        let mut game1 = crate::Game::new_game();
        let moves = [
            "D7D6\n", "D2D3\n", "E8D7\n", "E1D2\n", "D7E6\n", "D2E3\n", "E6E5\n",
        ];
        for pmove in moves {
            assert!(game1.player_move(pmove.to_owned()).is_ok());
        }
        assert!(game1.player_move("E3E4\n".to_owned()).is_err());
    }
    #[test]
    fn check_mate() {
        let mut game1 = crate::Game::new_game();
        let moves = [
            "D7D6\n", "E2E4\n", "D8D7\n", "E1E2\n", "D7C6\n", "D1E1\n", "C8D7\n", "B1C3\n",
            "D7C8\n", "B2B3\n", "C8D7\n", "C1B2\n", "D7C8\n", "A1D1\n", "C8D7\n", "C3B1\n",
            "C6E4\n",
        ];
        for pmove in moves {
            let _ = game1.player_move(pmove.to_owned());
        }
        assert!(game1.check_mate);
    }
    #[test]
    fn not_check_mate_if_can_save() {
        let mut game1 = crate::Game::new_game();
        let moves = [
            "D7D6\n", "E2E4\n", "D8D7\n", "E1E2\n", "D7C6\n", "D1E1\n", "C8D7\n", "B1C3\n",
            "D7C8\n", "B2B3\n", "C8D7\n", "C1B2\n", "D7C8\n", "A1D1\n", "C6E4\n",
        ];
        for pmove in moves {
            let _ = game1.player_move(pmove.to_owned());
        }
        assert!(!game1.check_mate);
    }
    #[test]
    fn queen_move_through_king_bug() {
        let mut game1 = crate::Game::new_game();
        let moves = [
            "D7D6\n", "E2E4\n", "D8D7\n", "E1E2\n", "D7C6\n", "D1E1\n", "C8D7\n", "B1C3\n",
            "D7C8\n", "B2B3\n", "C8D7\n", "C1B2\n", "D7C8\n", "A1D1\n", "C6E4\n",
        ];
        for pmove in moves {
            let _ = game1.player_move(pmove.to_owned());
        }
        assert!(game1.player_move("E1E4\n".to_owned()).is_err());
    }
    #[test]
    fn can_save_check_by_blocking() {
        let mut game1 = crate::Game::new_game();
        let moves = ["E7E6\n", "F2F3\n", "D8H4\n"];
        for pmove in moves {
            let _ = game1.player_move(pmove.to_owned());
        }
        assert!(game1.player_move("G2G3\n".to_owned()).is_ok());
    }
    #[test]
    fn cant_move_irrelevant_piece_during_check() {
        let mut game1 = crate::Game::new_game();
        let moves = ["E7E6\n", "F2F3\n", "D8H4\n"];
        for pmove in moves {
            let _ = game1.player_move(pmove.to_owned());
        }
        assert!(game1.player_move("D2D4\n".to_owned()).is_err());
    }
    #[test]
    fn can_move_if_still_blocks() {
        let mut game1 = crate::Game::new_game();
        let moves = ["E7E6\n", "A2A3\n", "D8F6\n", "A3A4\n", "F6E5\n"];
        for pmove in moves {
            let _ = game1.player_move(pmove.to_owned());
        }
        assert!(game1.player_move("E2E3\n".to_owned()).is_ok());
    }
    #[test]
    fn king_can_castle_right() {
        let mut game1 = crate::Game::new_game();
        let moves = ["G8H6\n", "A2A3\n", "E7E6\n", "A3A4\n", "F8E7\n", "A4A5\n"];
        for pmove in moves {
            let _ = game1.player_move(pmove.to_owned());
        }
        assert!(game1.player_move("E8G8\n".to_owned()).is_ok());
    }
    #[test]
    fn king_can_castle_left() {
        let mut game1 = crate::Game::new_game();
        let moves = [
            "B8A6\n", "A2A3\n", "D7D6\n", "A3A4\n", "C8E6\n", "A4A5\n", "D8D7\n", "B2B3\n",
        ];
        for pmove in moves {
            let _ = game1.player_move(pmove.to_owned());
        }
        assert!(game1.player_move("E8C8\n".to_owned()).is_ok());
    }
    #[test]
    fn fifty_move_rule() {
        let mut game1 = crate::Game::new_game();
        let moves = ["E7E6\n", "E2E3\n"];
        for pmove in moves {
            let _ = game1.player_move(pmove.to_owned());
        }
        for _ in 0..13 {
            let _ = game1.player_move("E8E7\n".to_owned());
            let _ = game1.player_move("E1E2\n".to_owned());
            let _ = game1.player_move("E7E8\n".to_owned());
            let _ = game1.player_move("E2E1\n".to_owned());
        }
        assert!(game1.claim_draw());
    }
    #[test]
    fn pawn_promotion() {
        panic!();
    }
}

//TO IMPLEMENT (remove when there is a corresponding test)
//test how well it works when piece is locked by several pieces
//pawn promotion
//points for eliminated pieces

#[macro_use]
pub mod macros;

pub mod pieces;
use pieces::Piece;
use pieces::PieceKind;
use std::collections::HashMap;
use std::collections::VecDeque;

pub struct Game {
    pub board: [[Piece; 8]; 8],
    pub score: (u8, u8),
    pub turn_white: bool,
    pub w_checked: bool,
    pub b_checked: bool,
    pub locked_table: HashMap<u8, u8>,
    pub check_mate: bool,
    pub tie: bool,
    pub fifty_moves: VecDeque<(Piece, u8)>,
    pub vector_of_the_fallen: VecDeque<Piece>,
}

impl Game {
    pub fn new_game() -> Game {
        let turn_white = true;
        let board = Game::construct_board();
        let score = (0, 0);
        let (w_checked, b_checked) = (false, false);
        let locked_table = HashMap::new();
        let check_mate = false;
        let mut fifty_moves: VecDeque<(Piece, u8)> = VecDeque::with_capacity(50);
        for _ in 0..50 {
            fifty_moves.push_front((Piece::new_piece("bp"), 0));
        }
        let vector_of_the_fallen: VecDeque<Piece> = VecDeque::new();
        let tie = false;

        Game {
            board,
            score,
            turn_white,
            w_checked,
            b_checked,
            locked_table,
            check_mate,
            tie,
            fifty_moves,
            vector_of_the_fallen,
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

        //assigna alla pieces ett unikt id, som används för att hålla koll på vilka
        //rutor de hotar.
        let mut counter = 1;
        for i in 0..8 {
            for j in 0..8 {
                board[i][j].unique_name = counter;
                counter += 1;
            }
        }
        board
    }
    pub fn claim_draw(&self) -> bool {
        let mut found_pawn: bool = false;
        for i in self.fifty_moves.iter() {
            if i.0.kind == PieceKind::Pawn {
                found_pawn = true;
            }
        }
        let no_claim: bool = self.fifty_moves[0].1 == self.fifty_moves[49].1;
        if !found_pawn && no_claim {
            true
        } else {
            false
        }
    }

    pub fn pawn_promotion(&mut self, input: String, to_piece: String) -> Result<bool, String> {
        let verify_string = input.clone();
        let verify_string = [input].join(&verify_string);
        let verify_string = [verify_string].join("\n");
        let ((i, j), _) = match self.verify_input(&verify_string) {
            Ok(e) => e,
            Err(error) => return Err(error),
        };
        let piece = self.board[i][j];
        let valid_pieces_white = ["wr", "wk", "wb", "wq"];
        let valid_pieces_black = ["br", "bk", "bb", "bq"];
        if piece.kind == PieceKind::Pawn {
            if (i == 0 && !piece.color_white) || (i == 7 && piece.color_white) {
                if piece.color_white && valid_pieces_white.contains(&to_piece.as_str()) {
                    self.board[i][j] = Piece::new_piece(to_piece.as_str());
                }
                if !piece.color_white && valid_pieces_black.contains(&to_piece.as_str()) {
                    self.board[i][j] = Piece::new_piece(to_piece.as_str());
                }
            }
        }
        return Ok(true);
    }

    fn verify_input(&self, input: &String) -> Result<((usize, usize), (usize, usize)), String> {
        if input.len() < 5 {
            return Err("false input".to_owned());
        }

        //Convertera user input till index
        let table = ["A", "B", "C", "D", "E", "F", "G", "H"];
        if !table.contains(&&input[0..1]) || !table.contains(&&input[2..3]) {
            return Err("Invalid Input".to_owned());
        }

        //parse the digits and convert them to usize
        let parsed = &input[1..2];
        let from: usize = match parsed.parse::<usize>() {
            Ok(value) => value.to_owned() - 1,
            Err(error) => {
                return Err(error.to_string());
            }
        };
        //let from = from.to_owned();
        //let from = from - 1;
        let parsed = &input[3..4];
        let to: usize = match parsed.parse::<usize>() {
            Ok(value) => value.to_owned() - 1,
            Err(error) => {
                println!("{}", error);
                return Err("Internal Error".to_owned());
            }
        };
        //iterate digits from input
        let i = table.iter().position(|&s| s == &input[0..1]).unwrap();
        let j = table.iter().position(|&s| s == &input[2..3]).unwrap();
        return Ok(((from, i), (to, j)));
    }
    pub fn player_move(&mut self, input: String) -> Result<bool, String> {
        if self.check_mate {
            Ok(self.check_mate)
        } else {
            //verifiera user input
            let ((from, i), (to, j)) = match self.verify_input(&input) {
                Ok(e) => e,
                Err(err) => panic!("{:?}", err),
            };

            //iterate board to update threat zone
            for x in 0..8 {
                for z in 0..8 {
                    let checkpiece = self.board[x][z].clone();
                    let threatposition = (x, z);
                    if self.locked_table.contains_key(&checkpiece.unique_name) {
                        self.locked_table.remove(&checkpiece.unique_name);
                        for q in 0..8 {
                            for w in 0..8 {
                                let lockedpiece = &self.board[q][w];
                                if lockedpiece.locked == checkpiece.unique_name {
                                    self.board[q][w].locked = 0
                                }
                            }
                        }

                        let (check_scan, locked_counter) = checkpiece.legal_move(
                            &mut self.board,
                            threatposition,
                            threatposition,
                            true,
                        );

                        if check_scan && locked_counter == 1 {
                            if checkpiece.color_white {
                                self.b_checked = true;
                            } else {
                                self.w_checked = true;
                            }
                        }
                        if locked_counter > 1 {
                            self.locked_table
                                .insert(checkpiece.unique_name, locked_counter);
                        }
                    }
                }
            }

            //check piece, confirm that its this piece's turn
            let piece = &mut self.board[from][i].clone();
            if piece.id == ' ' || piece.color_white != self.turn_white {
                return Err("Illegal move: Wrong player turn".to_owned());
            }

            //calculate legal move
            let (current_move, _) = piece.legal_move(&mut self.board, (from, i), (to, j), false);

            let locked_lookup = match self.locked_table.get_key_value(&piece.locked) {
                Some(n) => n.1.to_owned(),
                None => 0,
            };
            //if the king is checked we need to verify that the move stops the check
            let mut move_clears_check = false;
            if self.b_checked || self.w_checked || piece.locked > 0 {
                let mut temp_board = self.board.clone();
                temp_board[to][j] = piece.clone();
                temp_board[from][i] = Default::default();
                for t in 0..8 {
                    for y in 0..8 {
                        if temp_board[t][y].kind == PieceKind::King
                            && temp_board[t][y].color_white == piece.color_white
                        {
                            let mut board_check = [[false; 8]; 8];
                            board_check = king_scan!((t, y), temp_board, board_check, [(0, 0)]);
                            if board_check[t][y] == true {
                                move_clears_check = true;
                            }
                        }
                    }
                }
            }

            if current_move
                && (!((piece.locked > 0 && piece.kind != PieceKind::King)
                    && self.board[to][j].unique_name != piece.locked)
                    || locked_lookup > 2
                    || move_clears_check)
                && ((!self.b_checked && !self.w_checked)
                    || ((self.b_checked || self.w_checked) && move_clears_check))
            {
                //check if the move opens up an opportunity for en passant.
                if ((from == 6 && to == 4) || (from == 1 && to == 3))
                    && piece.kind == PieceKind::Pawn
                {
                    piece.passant = true;
                }
                piece.moved = true;
                //if it captures a piece, move it to the Vector of the fallen
                if self.board[to][j].kind != PieceKind::None {
                    self.vector_of_the_fallen
                        .push_front(self.board[to][j].clone());
                }

                //make the move
                self.board[to][j] = piece.clone();
                self.board[from][i] = Default::default();

                //insert move in the move list
                self.fifty_moves
                    .push_front((piece.clone(), self.vector_of_the_fallen.len() as u8));
                self.fifty_moves.pop_back();

                //castling stuff
                if piece.kind == PieceKind::King {
                    match (from, i, to, j) {
                        (7, 4, 7, 6) => {
                            self.board[7][7].moved = true;
                            self.board[7][5] = self.board[7][7].clone();
                            self.board[7][7] = Default::default();
                        }
                        (7, 4, 7, 2) => {
                            self.board[7][0].moved = true;
                            self.board[7][3] = self.board[7][0].clone();
                            self.board[7][0] = Default::default();
                        }
                        (0, 4, 0, 2) => {
                            self.board[0][0].moved = true;
                            self.board[0][3] = self.board[7][0].clone();
                            self.board[0][0] = Default::default();
                        }
                        (0, 4, 0, 6) => {
                            self.board[0][7].moved = true;
                            self.board[0][5] = self.board[7][0].clone();
                            self.board[0][7] = Default::default();
                        }
                        (_, _, _, _) => (),
                    }
                }

                //scan to see if the moved piece is checking the king.
                let (check_scan, locked_counter) =
                    piece.legal_move(&mut self.board, (to, j), (to, j), true);
                if check_scan && locked_counter == 1 {
                    if piece.color_white {
                        self.b_checked = true;
                    } else {
                        self.w_checked = true;
                    }
                }
                if locked_counter > 1 {
                    self.locked_table.insert(piece.unique_name, locked_counter);
                }

                self.turn_white = !self.turn_white;
                //remove passants of the opposite team
                for i in 0..8 {
                    for j in 0..8 {
                        if self.board[i][j].color_white != piece.color_white {
                            self.board[i][j].passant = false;
                        }
                    }
                }

                if piece.kind == PieceKind::King {
                    for i in 0..8 {
                        for j in 0..8 {
                            let location = &mut self.board[i][j];
                            if location.color_white == piece.color_white && location.locked > 0 {
                                self.locked_table.remove_entry(&location.locked);
                                location.locked = 0;
                            }
                        }
                    }

                    if piece.color_white {
                        self.w_checked = false;
                    } else {
                        self.b_checked = false;
                    }
                }
                for i in 0..8 {
                    for j in 0..8 {
                        let location = &mut self.board[i][j].clone();
                        if location.kind == PieceKind::King {
                            let checkmate = location.check_mate(&self.board, (i, j));
                            if checkmate == true {
                                if (location.color_white && self.w_checked)
                                    || (!location.color_white && self.b_checked)
                                {
                                    let mut notsafe = true;
                                    for p in 0..8 {
                                        for e in 0..8 {
                                            if self.board[p][e].color_white == location.color_white
                                                && self.board[p][e].kind != PieceKind::None
                                            {
                                                for q in 0..8 {
                                                    for w in 0..8 {
                                                        let mut temp_board = self.board.clone();
                                                        if self.board[p][e]
                                                            .legal_move(
                                                                &mut temp_board,
                                                                (p, e),
                                                                (q, w),
                                                                false,
                                                            )
                                                            .0
                                                        {
                                                            temp_board[q][w] =
                                                                temp_board[p][e].clone();
                                                            temp_board[p][e] = Default::default();
                                                        }
                                                        let mut board_check = [[false; 8]; 8];
                                                        board_check = king_scan!(
                                                            (i, j),
                                                            temp_board,
                                                            board_check,
                                                            [(0, 0)]
                                                        );
                                                        if board_check[i][j] == true {
                                                            notsafe = false;
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                    if notsafe == true {
                                        self.check_mate = true;
                                    }
                                }
                            }
                        }
                    }
                }
                Ok(true)
            } else {
                return Err("Illegal move".to_owned());
            }
        }
    }

    pub fn print_board(&self) {
        for i in 0..8 as usize {
            print!("{}  ", i + 1);
            for j in 0..8 as usize {
                print!("{} ", self.board[i][j].id);
            }
            print!("\n");
        }
        println!("   A B C D E F G H");
    }
}
