#[derive(Debug, Clone, PartialEq)]
pub enum PieceKind {
    None,
    Pawn,
    Rook,
    Knight,
    Bishop,
    Queen,
    King,
}

// impl Default for PieceKind {
//     fn default() -> Self {
//         PieceKind::None
//     }
// }

#[derive(Debug, Clone)]
pub struct Piece {
    pub id: char,
    pub kind: PieceKind,
    pub color_white: bool,
    pub unique_name: u8,

    //only used for pawns
    pub passant: bool,
}

impl Default for Piece {
    fn default() -> Self {
        Piece {
            id: ' ',
            kind: PieceKind::None,
            color_white: false,
            unique_name: 0,
            passant: false,
        }
    }
}

impl Piece {
    pub fn new_piece(code: &str) -> Piece {
        let (id, kind, color_white) = match code {
            "wp" => ('\u{265F}', PieceKind::Pawn, true),
            "wr" => ('\u{265C}', PieceKind::Rook, true),
            "wk" => ('\u{265E}', PieceKind::Knight, true),
            "wb" => ('\u{265D}', PieceKind::Bishop, true),
            "wq" => ('\u{265B}', PieceKind::Queen, true),
            "wx" => ('\u{265A}', PieceKind::King, true),
            "bp" => ('\u{2659}', PieceKind::Pawn, false),
            "br" => ('\u{2656}', PieceKind::Rook, false),
            "bk" => ('\u{2658}', PieceKind::Knight, false),
            "bb" => ('\u{2657}', PieceKind::Bishop, false),
            "bq" => ('\u{2655}', PieceKind::Queen, false),
            "bx" => ('\u{2654}', PieceKind::King, false),
            "nn" => (' ', PieceKind::None, false),
            _ => (' ', PieceKind::None, false),
        };
        let unique_name = 0;
        let passant = false;
        Piece {
            id,
            kind,
            color_white,
            unique_name,
            passant,
        }
    }

    //the function where legal moves for all piece types is calculated.
    pub fn legal_move(
        &self,
        board: &[[Piece; 8]; 8],
        start: (usize, usize),
        stop: (usize, usize),
    ) -> bool {
        let mut allowed_board = [[false; 8]; 8];
        println!("{},{}", start.0, start.1);
        //pawn
        if self.kind == PieceKind::Pawn {
            if self.color_white {
                if start.0 == 6 {
                    //jump two steps if on starting position
                    if board[start.0 - 2][start.1].kind == PieceKind::None {
                        allowed_board[start.0 - 2][start.1] = true;
                    }
                }
                //jump 1 step forward
                if board[start.0 - 1][start.1].kind == PieceKind::None {
                    allowed_board[start.0 - 1][start.1] = true;
                }
                //eliminate forward diagonally
                if start.1 < 7 {
                    let diagonal = &board[start.0 - 1][start.1 + 1];
                    if (diagonal.kind != PieceKind::None
                        && diagonal.color_white != self.color_white)
                        || board[start.0][start.1 + 1].passant == true
                    {
                        allowed_board[start.0 - 1][start.1 + 1] = true;
                    }
                }
                if start.1 > 0 {
                    let diagonal = &board[start.0 - 1][start.1 - 1];
                    if (diagonal.kind != PieceKind::None
                        && diagonal.color_white != self.color_white)
                        || board[start.0][start.1 - 1].passant == true
                    {
                        allowed_board[start.0 - 1][start.1 - 1] = true;
                    }
                }
            } else {
                if start.0 == 1 {
                    //jump two steps if on starting position
                    if board[start.0 + 2][start.1].kind == PieceKind::None {
                        allowed_board[start.0 + 2][start.1] = true;
                    }
                }
                //jump 1 step forward
                if board[start.0 + 1][start.1].kind == PieceKind::None {
                    allowed_board[start.0 + 1][start.1] = true;
                }
                //eliminate forward diagonally
                if start.1 < 7 {
                    let diagonal = &board[start.0 + 1][start.1 + 1];
                    if (diagonal.kind != PieceKind::None
                        && diagonal.color_white != self.color_white)
                        || board[start.0][start.1 + 1].passant == true
                    {
                        allowed_board[start.0 + 1][start.1 + 1] = true;
                    }
                }
                if start.1 > 0 {
                    let diagonal = &board[start.0 + 1][start.1 - 1];
                    if (diagonal.kind != PieceKind::None
                        && diagonal.color_white != self.color_white)
                        || board[start.0][start.1 - 1].passant == true
                    {
                        allowed_board[start.0 + 1][start.1 - 1] = true;
                    }
                }
            }
        }

        //straight
        if self.kind == PieceKind::Queen || self.kind == PieceKind::Rook {
            allowed_board = scan!(self, start, board, allowed_board, 0, 0, 0);
            allowed_board = scan!(self, start, board, allowed_board, 1, 0, 0);
            allowed_board = scan!(self, start, board, allowed_board, 0, 1, 0);
            allowed_board = scan!(self, start, board, allowed_board, 1, 1, 0);
        }

        //diagonal
        if self.kind == PieceKind::Queen || self.kind == PieceKind::Bishop {
            allowed_board = scan!(self, start, board, allowed_board, 0, 0, 1);
            allowed_board = scan!(self, start, board, allowed_board, 1, 0, 1);
            allowed_board = scan!(self, start, board, allowed_board, 0, 1, 1);
            allowed_board = scan!(self, start, board, allowed_board, 1, 1, 1);
        }

        //Knight
        if self.kind == PieceKind::Knight {
            if !(board[stop.0][stop.1].color_white == self.color_white
                && board[stop.0][stop.1].kind != PieceKind::None)
            {
                if start.0 + 2 == stop.0 || (start.0 >= 2 && start.0 - 2 == stop.0) {
                    if start.1 + 1 == stop.1 || (start.1 >= 1 && start.1 - 1 == stop.1) {
                        allowed_board[stop.0][stop.1] = true;
                    }
                }
                if start.0 + 1 == stop.0 || (start.0 >= 1 && start.0 - 1 == stop.0) {
                    if start.1 + 2 == stop.1 || (start.1 >= 2 && start.1 - 2 == stop.1) {
                        allowed_board[stop.0][stop.1] = true;
                    }
                }
            }
        }

        //King
        if self.kind == PieceKind::King {
            //calculate legal move from the target square?
            //calculation from all adjacent squares must be done at the
            //beginning of each turn if and only if the king is checked
        }

        for i in 0..8 as usize {
            print!("{}  ", i + 1);
            for j in 0..8 as usize {
                print!("{} ", allowed_board[i][j]);
            }
            print!("\n");
        }
        //confirm the move is in the array of allowed moves
        if allowed_board[stop.0][stop.1] {
            return true;
        } else {
            false
        }
    }
}
