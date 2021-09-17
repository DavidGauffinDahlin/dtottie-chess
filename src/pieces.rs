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
    id: char,
    kind: PieceKind,
    color_white: bool,
}

impl Default for Piece {
    fn default() -> Self {
        Piece {
            id: ' ',
            kind: PieceKind::None,
            color_white: false,
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
        Piece {
            id,
            kind,
            color_white,
        }
    }
}
