#[macro_export]
macro_rules! king_scan {
    //detta macro kollar vilka rutor kungen får flytta till. Finns inga, och kungen är i schack
    //så är spelet förlorat
    ($position:expr, $board:expr, $allowed_board:expr, $adjacent_tiles:expr) => {{
        for tile in $adjacent_tiles {
            let mut tile_checked = false;
            //kolla så vi inte går out of bounds
            if ((tile.0 < 0 && $position.0 as i8 >= -(tile.0))
                || (tile.0 >= 0 && $position.0 as i8 + tile.0 < 8))
                && ((tile.1 < 0 && $position.1 as i8 >= -(tile.1))
                    || (tile.1 >= 0 && $position.1 as i8 + tile.1 < 8))
            {
                
                let location = (
                    ($position.0 as i8 + tile.0) as usize,
                    ($position.1 as i8 + tile.1) as usize,
                );
                //check if the tile is empty or occupied by an enemy piece
                if $board[location.0][location.1].kind == PieceKind::None
                    || ($board[location.0][location.1].kind != PieceKind::None
                        && $board[location.0][location.1].color_white
                            != $board[$position.0][$position.1].color_white)
                {
                    $allowed_board[location.0][location.1] = true;
                    //down
                    if tile_checked == false {
                        for i in (location.0 + 1)..7 {
                            let square = &$board[i][location.1];
                            if square.kind != PieceKind::None {
                                if square.color_white
                                    == $board[$position.0][$position.1].color_white
                                {
                                    break;
                                } else {
                                if square.kind == PieceKind::Queen
                                    || square.kind == PieceKind::Rook
                                    || (square.kind == PieceKind::King && i == 1)
                                {
                                    $allowed_board[location.0][location.1] = false;
                                    tile_checked = true;
                                }}
                            }
                        }
                    }
                    //up
                    if tile_checked == false {
                        for i in (0..(location.0)).rev() {
                            let square = &$board[i][location.1];
                            if square.kind != PieceKind::None {
                                if square.color_white
                                    == $board[$position.0][$position.1].color_white
                                {
                                    break;
                                } else {
                                if square.kind == PieceKind::Queen
                                    || square.kind == PieceKind::Rook
                                    || (square.kind == PieceKind::King && i == 1)
                                {
                                    $allowed_board[location.0][location.1] = false;
                                    tile_checked = true;
                                }}
                            }
                        }
                    }
                    //right
                    if tile_checked == false {
                        for i in (location.1 + 1)..7 {
                            let square = &$board[location.0][i];
                            if square.kind != PieceKind::None {
                                if square.color_white
                                    == $board[$position.0][$position.1].color_white
                                {
                                    break;
                                } else {
                                if square.kind == PieceKind::Queen
                                    || square.kind == PieceKind::Rook
                                    || (square.kind == PieceKind::King && i == 1)
                                {
                                    $allowed_board[location.0][location.1] = false;
                                    tile_checked = true;
                                }}
                            }
                        }
                    }
                    //left
                    if tile_checked == false {
                        for i in (0..(location.1)).rev() {
                            let square = &$board[location.0][i];
                            if square.kind != PieceKind::None {
                                if square.color_white
                                    == $board[$position.0][$position.1].color_white
                                {
                                    break;
                                } else {
                                if square.kind == PieceKind::Queen
                                    || square.kind == PieceKind::Rook
                                    || (square.kind == PieceKind::King && i == 1)
                                {
                                    $allowed_board[location.0][location.1] = false;
                                    tile_checked = true;
                                }}
                            }
                        }
                    }
                    //diagonal ones
                    if tile_checked == false {
                        for i in 1..7 {
                            if (location.0 + i < 8 && location.1 + i < 8) {
                                let square = &$board[location.0 + i][location.1 + i];
                                if square.color_white
                                    == $board[$position.0][$position.1].color_white
                                {
                                    break;
                                } else {
                                if square.kind == PieceKind::Queen
                                    || (square.kind == PieceKind::Pawn && i == 1)
                                    || (square.kind == PieceKind::King && i == 1)
                                    || square.kind == PieceKind::Bishop
                                {
                                    $allowed_board[location.0][location.1] = false;
                                    tile_checked = true;
                                }}
                            }
                        }
                    }
                    if tile_checked == false {
                        for i in 1..7 {
                            if (location.0 + i < 8 && location.1 >= i) {
                                let square = &$board[location.0 + i][location.1 - i];
                                if square.color_white
                                    == $board[$position.0][$position.1].color_white
                                {
                                    break;
                                } else {
                                if square.kind == PieceKind::Queen
                                    || (square.kind == PieceKind::Pawn && i == 1)
                                    || (square.kind == PieceKind::King && i == 1)
                                    || square.kind == PieceKind::Bishop
                                {
                                    $allowed_board[location.0][location.1] = false;
                                    tile_checked = true;
                                }}
                            }
                        }
                    }
                    if tile_checked == false {
                        for i in 1..7 {
                            if (location.0 >= i && location.1 + i < 8) {
                                let square = &$board[location.0 - i][location.1 + i];
                                if square.color_white
                                    == $board[$position.0][$position.1].color_white
                                {
                                    break;
                                } else {
                                if square.kind == PieceKind::Queen
                                    || (square.kind == PieceKind::Pawn && i == 1)
                                    || (square.kind == PieceKind::King && i == 1)
                                    || square.kind == PieceKind::Bishop
                                {
                                    $allowed_board[location.0][location.1] = false;
                                    tile_checked = true;
                                }}
                            }
                        }
                    }
                    if tile_checked == false {
                        for i in 1..7 {
                            if (location.0 >= i && location.1 >= i) {
                                let square = &$board[location.0 - i][location.1 - i];

                                if square.color_white
                                    == $board[$position.0][$position.1].color_white
                                    && square.kind != PieceKind::King
                                {
                                    break;
                                } else {
                                if square.kind == PieceKind::Queen
                                    || (square.kind == PieceKind::Pawn && i == 1)
                                    || (square.kind == PieceKind::King && i == 1)
                                    || square.kind == PieceKind::Bishop
                                {
                                    $allowed_board[location.0][location.1] = false;
                                    tile_checked = true;
                                }}
                            }
                        }
                    }
                    //check for that damned horse
                    if tile_checked == false {
                        let moves = [(2,1),(2,-1),(1,2),(1,-2),(-2,1),(-2,-1),(-1,2),(-1,-2)];
                        for pmove in moves {
                            if tile_checked == false {
                            if ((pmove.0 < 0 && location.0 as i8 >= -(pmove.0))
                                || (pmove.0 >= 0 && location.0 as i8 + pmove.0 < 8))
                                && ((pmove.1 < 0 && location.1 as i8 >= -(pmove.1))
                                || (pmove.1 >= 0 && location.1 as i8 + pmove.1 < 8)) {
                                    let knightcheck = (
                                        (location.0 as i8 + pmove.0) as usize,
                                        (location.1 as i8 + pmove.1) as usize,
                                    );
                                    if $board[knightcheck.0][knightcheck.1].kind == PieceKind::Knight && $board[knightcheck.0][knightcheck.1].color_white != $board[$position.0][$position.1].color_white {
                                        $allowed_board[location.0][location.1] = false;
                                        tile_checked = true;
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        $allowed_board
    };};
}

#[macro_export]
macro_rules! scan {
    ($piece:expr,$position:expr,$board:expr,$allowed_board:expr,$reverse:expr,$horizontal:expr,$diagonal:expr, $checkcheck:expr) => {{
        let mut locked_board = $allowed_board;
        let mut block = false;

        //validera input för att undvika att go out of range vid indexering av brädet
        if $horizontal == 0 {
            if $reverse == 0 {
                if $position.0 > 8 {
                    block = true;
                }
            } /* else {
                if $position.0 < 0 {
                    block = true;
                }
            } */
        } else {
            if $reverse == 0 {
                if $position.1 > 8 {
                    block = true;
                }
            } /* else {
                if $position.1 < 0 {
                    block = true;
                }
            } */
        }
        //när vi skannar åt ett håll antar vi att alla rutor med pjäser ska låsas (ej få flyttas då det sätter kungen i schack)
        //om vi sedan inte hittar någon kung i samma riktning så droppar vi dessa element.
        let mut locked_temp = vec![];
        if $reverse == 0 {
            let iter = match ($horizontal, $diagonal) {
                (_, 1) => 1..7,
                (1, _) => ($position.1 + 1)..8,
                (_, _) => ($position.0 + 1)..8,
            };
            //två for-loopar. En framlänges och en baklänges. Detta så vi kan
            //iterera genom rutor bort från den valda pjäsens position.

            for i in iter {
                if block == false {
                    let location = match ($horizontal, $diagonal) {
                        (1, 1) => {
                            if $position.0 + i < 8 && $position.1 + i < 8 {
                                &$board[$position.0 + i][$position.1 + i]
                            } else {
                                &$board[0][0]
                            }
                        }
                        (_, 1) => {
                            if $position.0 + i < 8 && $position.1 >= i {
                                &$board[$position.0 + i][$position.1 - i]
                            } else {
                                &$board[0][0]
                            }
                        }
                        (1, _) => &$board[$position.0][i],
                        (_, _) => &$board[i][$position.1],
                    };
                    if location.kind != PieceKind::None {
                        if location.color_white != $piece.color_white {
                            match ($horizontal, $diagonal) {
                                (1, 1) => {
                                    if $position.0 + i < 8 && $position.1 + i < 8 {
                                        $allowed_board[$position.0 + i][$position.1 + i] = true;
                                        if $checkcheck {
                                            locked_temp.push((($position.0 + i), ($position.1 + i)))
                                        }
                                    }
                                }
                                (_, 1) => {
                                    if $position.1 >= i && $position.0 + i < 8 {
                                        $allowed_board[$position.0 + i][$position.1 - i] = true;
                                        if $checkcheck {
                                            locked_temp.push((($position.0 + i), ($position.1 - i)))
                                        }
                                    }
                                }
                                (1, _) => {
                                    $allowed_board[$position.0][i] = true;
                                    if $checkcheck {
                                        locked_temp.push((($position.0), (i)))
                                    }
                                }
                                (_, _) => {
                                    $allowed_board[i][$position.1] = true;
                                    if $checkcheck {
                                        locked_temp.push(((i), ($position.1)))
                                    }
                                }
                            };
                        }
                        if $checkcheck == true && location.kind == PieceKind::King {
                            block = true;
                        }
                        if $checkcheck == false && location.kind != PieceKind::King {
                            block = true;
                        }
                    }
                    if block == false {
                        match ($horizontal, $diagonal) {
                            (1, 1) => {
                                if $position.0 + i < 8 && $position.1 + i < 8 {
                                    $allowed_board[$position.0 + i][$position.1 + i] = true
                                }
                            }
                            (_, 1) => {
                                if $position.1 >= i && $position.0 + i < 8 {
                                    $allowed_board[$position.0 + i][$position.1 - i] = true
                                }
                            }
                            (1, _) => $allowed_board[$position.0][i] = true,
                            (_, _) => $allowed_board[i][$position.1] = true,
                        };
                    }
                }
                if block == true && $checkcheck && locked_temp.len() > 0 {
                    for places in &locked_temp {
                        locked_board[places.0][places.1] = true;
                    }
                    //locked_board[locked_temp[0].0][locked_temp[0].1] = true;
                }
            }
        } else {
            let iter = match ($horizontal, $diagonal) {
                (_, 1) => 1..7,
                (1, _) => (0..($position.1)),
                (_, _) => (0..($position.0)),
            };
            for i in iter.rev() {
                let j = if $diagonal == 1 { 7 - i } else { i };
                if block == false {
                    let location = match ($horizontal, $diagonal) {
                        (1, 1) => {
                            if $position.0 >= j && $position.1 + j < 8 {
                                &$board[$position.0 - j][$position.1 + j]
                            } else {
                                &$board[0][0]
                            }
                        }
                        (_, 1) => {
                            if $position.1 >= j && $position.0 >= j {
                                &$board[$position.0 - j][$position.1 - j]
                            } else {
                                &$board[0][0]
                            }
                        }
                        (1, _) => &$board[$position.0][i],
                        (_, _) => &$board[i][$position.1],
                    };
                    if location.kind != PieceKind::None {
                        if location.color_white != $piece.color_white {
                            match ($horizontal, $diagonal) {
                                (1, 1) => {
                                    if $position.0 >= j && $position.1 + j < 8 {
                                        $allowed_board[$position.0 - j][$position.1 + j] = true;
                                        if $checkcheck {
                                            locked_temp.push((($position.0 - j), ($position.1 + j)));
                                        }
                                    }
                                }

                                (_, 1) => {
                                    if $position.0 >= j && $position.1 >= j {
                                        $allowed_board[$position.0 - j][$position.1 - j] = true;
                                        if $checkcheck {
                                            locked_temp
                                                .push((($position.0 - j), ($position.1 - j)));
                                        }
                                    }
                                }

                                (1, _) => {
                                    $allowed_board[$position.0][i] = true;
                                    if $checkcheck {
                                        locked_temp.push((($position.0), (i)))
                                    }
                                }
                                (_, _) => {
                                    $allowed_board[i][$position.1] = true;
                                    if $checkcheck {
                                        locked_temp.push(((i), ($position.1)))
                                    }
                                }
                            };
                        }

                        if $checkcheck == true && location.kind == PieceKind::King {
                            block = true;
                        }

                        if $checkcheck == false && location.kind != PieceKind::King {
                            block = true;
                        }
                    }
                    if block == false && !$checkcheck {
                        match ($horizontal, $diagonal) {
                            (1, 1) => {
                                if $position.0 >= j && $position.1 + j < 8 {
                                    $allowed_board[$position.0 - j][$position.1 + j] = true;
                                }
                            }
                            (_, 1) => {
                                if $position.0 >= j && $position.1 >= j {
                                    $allowed_board[$position.0 - j][$position.1 - j] = true
                                }
                            }
                            (1, _) => $allowed_board[$position.0][i] = true,
                            (_, _) => $allowed_board[i][$position.1] = true,
                        };
                    }
                }
                if block == true && $checkcheck && locked_temp.len() > 0 {
                    for places in &locked_temp {
                        locked_board[places.0][places.1] = true;
                    }
                    //locked_board[locked_temp[0].0][locked_temp[0].1] = true;
                }
            }
        }

        //för tydlighets skull valde jag att kalla dessa olika saker.
        if !$checkcheck {
            $allowed_board
        } else {
            locked_board
        }
    }};
}
