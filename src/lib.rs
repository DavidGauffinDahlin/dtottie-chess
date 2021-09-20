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
        let moves = ["C7C6\n", "D2D3\n", "D8B6\n", "C1D2\n","B6A5\n","D2A5\n"];
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
        let moves = ["C7C6\n", "H2H3\n", "D8A5\n", "H3H4\n","A5B4\n"];
        for pmove in moves {
            assert!(!game1.player_move(pmove.to_owned()).is_err());
        }
    }
    #[test]
    fn move_aggressive_piece_clears_lock() {
        let mut game1 = crate::Game::new_game();
        let moves = ["C7C6\n", "H2H3\n", "D8A5\n", "H3H4\n","A5D8\n", "D2D3\n"];
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
        let moves = ["D7D5\n", "E2E4\n", "A7A6\n", "E1E2\n","A6A5\n", "E2E3\n", "D5D4\n"];
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
        let moves = ["D7D5\n", "E2E4\n", "D5D4\n", "E1E2\n","A7A6\n"];
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
        let moves = ["D7D6\n", "D2D3\n", "E8D7\n","E1D2\n", "D7E6\n", "D2E3\n" , "E6E5\n"];
        for pmove in moves {
            assert!(game1.player_move(pmove.to_owned()).is_ok());
        }
        assert!(game1.player_move("E3E4\n".to_owned()).is_err());
    }
}

//TO IMPLEMENT (remove when there is a corresponding test)
//king scan at the end of every move? [lib]
//king can threaten tiles [kingscan]
//game can end [lib]
//piece can eliminate king's threatener so game doesn't end [lib]
//test how well it works when piece is locked by several pieces
//king can tower with rook [pieces.rs]
//chess 50 move rule
//chess 3 move rule
//points for eliminated pieces



#[macro_use]
pub mod macros;

pub mod pieces;
use pieces::Piece;
use pieces::PieceKind;
use std::collections::HashMap;

pub struct Game {
    pub board: [[Piece; 8]; 8],
    pub score: (u8, u8),
    pub turn_white: bool,
    pub w_checked: bool,
    pub b_checked: bool,
    pub locked_table: HashMap<u8, u8>,
}

impl Game {
    pub fn new_game() -> Game {
        let turn_white = true;
        let board = Game::construct_board();
        let score = (0, 0);
        let (w_checked, b_checked) = (false, false);
        let locked_table = HashMap::new();
        Game {
            board,
            score,
            turn_white,
            w_checked,
            b_checked,
            locked_table,
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

    pub fn player_move(&mut self, input: String) -> Result<bool, &str> {
        //verifiera user input
        if input.len() < 5 {
            return Err("Invalid input");
        }

        //Convertera user input till index
        let table = ["A", "B", "C", "D", "E", "F", "G", "H"];
        if !table.contains(&&input[0..1]) || !table.contains(&&input[2..3]) {
            return Err("Invalid Input");
        }

        //parse the digits and convert them to usize
        let parsed = &input[1..2];
        let from: usize = match parsed.parse::<usize>() {
            Ok(value) => value.to_owned() - 1,
            Err(error) => {
                println!("{}", error);
                return Err("internal error");
            }
        };
        //let from = from.to_owned();
        //let from = from - 1;
        let parsed = &input[3..4];
        let to: usize = match parsed.parse::<usize>() {
            Ok(value) => value.to_owned() - 1,
            Err(error) => {
                println!("{}", error);
                return Err("Internal Error");
            }
        };
        //iterate digits from input
        let i = table.iter().position(|&s| s == &input[0..1]).unwrap();
        let j = table.iter().position(|&s| s == &input[2..3]).unwrap();

        //iterate board to update threat zone
        for x in 0..8 {
            for z in 0..8 {
                let checkpiece = self.board[x][z].clone();
                let threatposition = (x,z);
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
                    println!("checkpoint 0");
                    //let (check_scan,locked_counter) = (false,0);
                    let (check_scan, locked_counter) = checkpiece.legal_move(&mut self.board, threatposition, threatposition, true);

                    /* for p in 0..8 {
                        for q in 0..8 {
                            if self.board[p][q].locked > 0 {
                                println!("following piece is locked: {:?}", self.board[p][q].kind);
                            }
                        }
                    } */

                    //if the locking piece is the one currently being moved, we scan from where it is being moved instead.
                    //Although this happens later in the code anyway.
                    /* if checkpiece == self.board[from][i] {
                    check_scan = checkpiece.legal_move(&mut self.board, (to,j), (to,j), true).0;
                    locked_counter = checkpiece.legal_move(&mut self.board, (to,j), (to,j), true).1;
                } */
                    println!("checkscan = {}, locked_counter= {}",check_scan, locked_counter);
                    if check_scan && locked_counter == 1 {
                        println!("checkpoint 1");
                        if checkpiece.color_white {
                            self.b_checked = true;
                        } else {
                            self.w_checked = true;
                        }
                    }
                    if locked_counter > 1 {
                        self.locked_table.insert(checkpiece.unique_name, locked_counter);
                    }
                }

            }
        }



        //check piece, confirm that its this piece's turn
        let piece = &mut self.board[from][i].clone();
        if piece.id == ' ' || piece.color_white != self.turn_white {
            return Err("Illegal move: Wrong player turn");
        }

        //calculate legal move
        let (current_move, _) = piece.legal_move(&mut self.board, (from, i), (to, j), false);
        println!("current_move is {:?}", current_move);
        
        let locked_lookup = match self.locked_table.get_key_value(&piece.locked) {
            Some(n) => n.1.to_owned(),
            None => 0,
        };
        
        println!("lockedlookup is {}", locked_lookup);
        if current_move
            && (!((piece.locked > 0
                && piece.kind != PieceKind::King)
                && self.board[to][j].unique_name != piece.locked) || locked_lookup > 2)
        {
            //check if the move opens up an opportunity for en passant.
            if ((from == 6 && to == 4) || (from == 1 && to == 3)) && piece.kind == PieceKind::Pawn {
                piece.passant = true;
            }
            self.board[to][j] = piece.clone();
            self.board[from][i] = Default::default();

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
                println!("moved piece is king");
                for i in 0..8 {
                    for j in 0..8 {
                        let location = &mut self.board[i][j];
                        if location.color_white == piece.color_white && location.locked > 0 {
                            self.locked_table.remove_entry(&location.locked);
                            location.locked = 0;
                            print!("removed lock for {:?}\n", location.kind);
                        }
                    }
                }
                println!("we reached to the part where we remove the checking");
                if piece.color_white {
                    self.w_checked = false;
                } else {
                    self.b_checked = false;
                }
            }

            Ok(true)
        } else {
            return Err("Illegal move");
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
