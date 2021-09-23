#[derive(Debug, Clone, PartialEq, Copy)]
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

#[derive(Debug, Clone, PartialEq, Copy)]
pub struct Piece {
    pub id: char,
    pub kind: PieceKind,
    pub color_white: bool,
    pub unique_name: u8,
    pub locked: u8,

    //only used for pawns
    pub passant: bool,

    //only used for rook and king
    pub moved: bool,
}

impl Default for Piece {
    fn default() -> Self {
        Piece {
            id: ' ',
            kind: PieceKind::None,
            color_white: false,
            unique_name: 0,
            locked: 0,
            passant: false,
            moved: false,
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
        let locked = 0;
        let passant = false;
        let moved = false;
        Piece {
            id,
            kind,
            color_white,
            unique_name,
            locked,
            passant,
            moved,
        }
    }

    //the function where legal moves for all piece types is calculated.
    pub fn check_mate(&self, board: &[[Piece; 8]; 8], position: (usize, usize)) -> bool {
        if self.kind == PieceKind::King {
            let mut allowed_board = [[false; 8]; 8];
            let adjacent_tiles = [
                (1, 0),
                (1, 1),
                (0, 1),
                (-1, 1),
                (-1, 0),
                (-1, -1),
                (0, -1),
                (1, -1),
            ];
            allowed_board = king_scan!(position, board, allowed_board, adjacent_tiles);
            let mut true_count = 0;
            for i in 0..8 {
                for j in 0..8 {
                    if allowed_board[i][j] == true {
                        true_count += 1;
                    }
                }
            }
            if true_count == 0 {
                return true;
            }
        }
        false
    }
    pub fn legal_move(
        &self,
        board: &mut [[Piece; 8]; 8],
        start: (usize, usize),
        stop: (usize, usize),
        check_for_check: bool,
    ) -> (bool, u8) {
        let mut allowed_board = [[false; 8]; 8];
        let mut locked_board = [[false; 8]; 8];
        //pawn
        if self.kind == PieceKind::Pawn {
            //since pawns are directional we need to write a black and a white verison of
            //these calculations
            if !check_for_check {
                if self.color_white {
                    if start.0 == 6 {
                        //jump two steps if on starting position
                        if board[start.0 - 2][start.1].kind == PieceKind::None
                            && board[start.0 - 1][start.1].kind == PieceKind::None
                        {
                            allowed_board[start.0 - 2][start.1] = true;
                        }
                    }
                    //jump 1 step forward
                    if board[start.0 - 1][start.1].kind == PieceKind::None {
                        allowed_board[start.0 - 1][start.1] = true;
                    }
                    //eliminate forward diagonally, and potentially allow for a passant
                    if start.1 < 7 {
                        let diagonal = &board[start.0 - 1][start.1 + 1];
                        if (diagonal.kind != PieceKind::None
                            && diagonal.color_white != self.color_white)
                            || (board[start.0][start.1 + 1].passant == true
                                && board[start.0 - 1][start.1].kind == PieceKind::None)
                        {
                            allowed_board[start.0 - 1][start.1 + 1] = true;
                        }
                    }
                    if start.1 > 0 {
                        let diagonal = &board[start.0 - 1][start.1 - 1];
                        if (diagonal.kind != PieceKind::None
                            && diagonal.color_white != self.color_white)
                            || (board[start.0][start.1 - 1].passant == true
                                && board[start.0 - 1][start.1].kind == PieceKind::None)
                        {
                            allowed_board[start.0 - 1][start.1 - 1] = true;
                        }
                    }
                } else {
                    if start.0 == 1 {
                        //jump two steps if on starting position
                        if board[start.0 + 2][start.1].kind == PieceKind::None
                            && board[start.0 + 1][start.1].kind == PieceKind::None
                        {
                            allowed_board[start.0 + 2][start.1] = true;
                        }
                    }
                    //jump 1 step forward
                    if board[start.0 + 1][start.1].kind == PieceKind::None {
                        allowed_board[start.0 + 1][start.1] = true;
                    }
                    //eliminate forward diagonally, and potentionally allow for a passant
                    if start.1 < 7 {
                        let diagonal = &board[start.0 + 1][start.1 + 1];
                        if (diagonal.kind != PieceKind::None
                            && diagonal.color_white != self.color_white)
                            || (board[start.0][start.1 + 1].passant == true
                                && board[start.0 + 1][start.1].kind == PieceKind::None)
                        {
                            allowed_board[start.0 + 1][start.1 + 1] = true;
                        }
                    }
                    if start.1 > 0 {
                        let diagonal = &board[start.0 + 1][start.1 - 1];
                        if (diagonal.kind != PieceKind::None
                            && diagonal.color_white != self.color_white)
                            || (board[start.0][start.1 - 1].passant == true
                                && board[start.0 + 1][start.1].kind == PieceKind::None)
                        {
                            allowed_board[start.0 + 1][start.1 - 1] = true;
                        }
                    }
                }
            } else {
                if self.color_white {
                    if start.0 >= 1 {
                        if start.1 + 1 < 8 {
                            if &board[start.0 - 1][start.1 + 1].kind == &PieceKind::King
                                && &board[start.0 - 1][start.1 + 1].color_white != &self.color_white
                            {
                                return (true, 1);
                            }
                        }
                        if start.1 >= 1 {
                            if &board[start.0 - 1][start.1 - 1].kind == &PieceKind::King
                                && &board[start.0 - 1][start.1 + 1].color_white != &self.color_white
                            {
                                return (true, 1);
                            }
                        }
                    }
                } else {
                    if start.0 + 1 < 8 {
                        if start.1 + 1 < 8 {
                            if &board[start.0 + 1][start.1 + 1].kind == &PieceKind::King
                                && &board[start.0 - 1][start.1 + 1].color_white != &self.color_white
                            {
                                return (true, 1);
                            }
                        }
                        if start.1 >= 1 {
                            if &board[start.0 + 1][start.1 - 1].kind == &PieceKind::King
                                && &board[start.0 - 1][start.1 + 1].color_white != &self.color_white
                            {
                                return (true, 1);
                            }
                        }
                    }
                }
            }
        }

        //Pieces travelling straight will use these calculations
        if self.kind == PieceKind::Queen || self.kind == PieceKind::Rook {
            if !check_for_check {
                allowed_board = scan!(self, start, board, allowed_board, 0, 0, 0, false);
                allowed_board = scan!(self, start, board, allowed_board, 1, 0, 0, false);
                allowed_board = scan!(self, start, board, allowed_board, 0, 1, 0, false);
                allowed_board = scan!(self, start, board, allowed_board, 1, 1, 0, false);
            } else {
                locked_board = scan!(self, start, board, locked_board, 0, 0, 0, true);
                locked_board = scan!(self, start, board, locked_board, 1, 0, 0, true);
                locked_board = scan!(self, start, board, locked_board, 0, 1, 0, true);
                locked_board = scan!(self, start, board, locked_board, 1, 1, 0, true);
            }
        }

        //pieces travelling diagonally will use these calculations
        if self.kind == PieceKind::Queen || self.kind == PieceKind::Bishop {
            if !check_for_check {
                allowed_board = scan!(self, start, board, allowed_board, 0, 0, 1, false);
                allowed_board = scan!(self, start, board, allowed_board, 1, 0, 1, false);
                allowed_board = scan!(self, start, board, allowed_board, 0, 1, 1, false);
                allowed_board = scan!(self, start, board, allowed_board, 1, 1, 1, false);
            } else {
                locked_board = scan!(self, start, board, locked_board, 0, 0, 1, true);
                locked_board = scan!(self, start, board, locked_board, 1, 0, 1, true);
                locked_board = scan!(self, start, board, locked_board, 0, 1, 1, true);
                locked_board = scan!(self, start, board, locked_board, 1, 1, 1, true);
            }
        }

        //Knight
        if self.kind == PieceKind::Knight {
            if !check_for_check {
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
            } else {
                let moves: [(i8, i8); 8] = [
                    (2, 1),
                    (2, -1),
                    (-2, 1),
                    (-2, -1),
                    (1, 2),
                    (1, -2),
                    (-1, 2),
                    (-1, -2),
                ];
                for pmove in moves {
                    if ((pmove.0 < 0 && stop.0 as i8 >= -(pmove.0))
                        || (pmove.0 >= 0 && pmove.0 as usize + stop.0 < 7))
                        && ((pmove.1 < 0 && stop.1 as i8 >= -(pmove.1))
                            || (pmove.1 >= 0 && pmove.1 as usize + stop.1 < 7))
                    {
                        let coordinate = (stop.0 as i8 + pmove.0, stop.1 as i8 + pmove.1);
                        let location = &board[coordinate.0 as usize][coordinate.1 as usize];

                        if location.kind == PieceKind::King
                            && location.color_white != self.color_white
                        {
                            return (true, 1);
                        }
                    }
                }
            }
        }

        //King
        if self.kind == PieceKind::King {
            //calculate legal move from the target square?
            let adjacent_tiles = [
                (1, 0),
                (1, 1),
                (0, 1),
                (-1, 1),
                (-1, 0),
                (-1, -1),
                (0, -1),
                (1, -1),
            ];
            allowed_board = king_scan!(start, board, allowed_board, adjacent_tiles);
            if start == (7, 4) || start == (0, 4) && self.moved == false {
                if self.color_white {
                    if board[7][5].kind == PieceKind::None
                        && board[7][6].kind == PieceKind::None
                        && (board[7][7].kind == PieceKind::Rook && board[7][7].moved == false)
                    {
                        allowed_board[7][6] = true;
                    }
                    if board[7][3].kind == PieceKind::None
                        && board[7][2].kind == PieceKind::None
                        && board[7][1].kind == PieceKind::None
                        && (board[7][0].kind == PieceKind::Rook && board[7][0].moved == false)
                    {
                        allowed_board[7][2] = true;
                    }
                } else {
                    if board[0][5].kind == PieceKind::None
                        && board[0][6].kind == PieceKind::None
                        && (board[0][7].kind == PieceKind::Rook && board[0][7].moved == false)
                    {
                        allowed_board[0][6] = true;
                    }
                    if board[0][3].kind == PieceKind::None
                        && board[0][2].kind == PieceKind::None
                        && board[0][1].kind == PieceKind::None
                        && (board[0][0].kind == PieceKind::Rook && board[0][0].moved == false)
                    {
                        allowed_board[0][2] = true;
                    }
                }
            }
        }

        if check_for_check {
            for i in 0..8 {
                for j in 0..8 {
                    if locked_board[i][j] == true {
                        board[i][j].locked = self.unique_name;
                    }
                }
            }
            let mut locked_counter = 0;
            for i in 0..8 {
                for j in 0..8 {
                    if locked_board[i][j] == true {
                        locked_counter += 1;
                    }
                }
            }
            if locked_counter != 0 {
                return (true, locked_counter);
            } else {
                return (false, 0);
            }
        }

        if allowed_board[stop.0][stop.1] && !check_for_check {
            return (true, 0);
        } else {
            (false, 0)
        }
    }
}
