#[macro_export]
macro_rules! scan_threat {
    ($position:expr, $board:expr) => {
        match $board[$position.0][$position.1] {}
    };
}

#[macro_export]
macro_rules! scan {
    ($piece:expr,$position:expr,$board:expr,$allowed_board:expr,$reverse:expr,$horizontal:expr,$diagonal:expr, $checkcheck:expr) => {{
        let mut locked_board = [[true; 8]; 8];
        let mut locked_temp = vec![];
        let mut block = false;
        //validera input för att undvika att go out of range vid indexering av brädet
        if $horizontal == 0 {
            if $reverse == 0 {
                if $position.0 > 8 {
                    block = true;
                }
            } else {
                if $position.0 < 1 {
                    block = true;
                }
            }
        } else {
            if $reverse == 0 {
                if $position.1 > 8 {
                    block = true;
                }
            } else {
                if $position.1 < 1 {
                    block = true;
                }
            }
        }

        if $reverse == 0 {
            let iter = match ($horizontal, $diagonal) {
                (_, 1) => 1..7,
                (1, _) => ($position.1 + 1)..8,
                (_, _) => ($position.0 + 1)..8,
            };
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
                                        if $checkcheck && location.kind != PieceKind::King {
                                            locked_temp.push((($position.0 + i), ($position.1 + i)))
                                        }
                                    }
                                }
                                (_, 1) => {
                                    if $position.1 >= i && $position.0 + i < 8 {
                                        $allowed_board[$position.0 + i][$position.1 - i] = true;
                                        if $checkcheck && location.kind != PieceKind::King {
                                            locked_temp.push((($position.0 + i), ($position.1 - i)))
                                        }
                                    }
                                }
                                (1, _) => {
                                    $allowed_board[$position.0][i] = true;
                                    if $checkcheck && location.kind != PieceKind::King {
                                        locked_temp.push((($position.0), (i)))
                                    }
                                }
                                (_, _) => {
                                    $allowed_board[i][$position.1] = true;
                                    if $checkcheck && location.kind != PieceKind::King {
                                        locked_temp.push(((i), ($position.1)))
                                    }
                                }
                            };
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
            }
            if block == true && $checkcheck {
                for places in locked_temp {
                    locked_board[places.0][places.1] = true;
                }
            }
        } else {
            let iter = match ($horizontal, $diagonal) {
                (_, 1) => 1..7,
                (1, _) => (0..($position.1)),
                (_, _) => (0..($position.0)),
            };
            for i in iter.rev() {
                locked_temp.clear();
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
                                        if $checkcheck && location.kind != PieceKind::King {
                                            locked_temp.push((($position.0 - j), ($position.1 + j)))
                                        }
                                    }
                                }

                                (_, 1) => {
                                    if $position.0 >= j && $position.1 >= j {
                                        $allowed_board[$position.0 - j][$position.1 - j] = true;
                                        if $checkcheck && location.kind != PieceKind::King {
                                            locked_temp.push((($position.0 - j), ($position.1 - j)))
                                        }
                                    }
                                }

                                (1, _) => {
                                    $allowed_board[$position.0][i] = true;
                                    if $checkcheck && location.kind != PieceKind::King {
                                        locked_temp.push((($position.0), (i)))
                                    }
                                }
                                (_, _) => {
                                    $allowed_board[i][$position.1] = true;
                                    if $checkcheck && location.kind != PieceKind::King {
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
                                if $position.0 >= j && $position.1 + j < 8 {
                                    $allowed_board[$position.0 - j][$position.1 + j] = true
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
            }
            if block == true && $checkcheck {
                for places in locked_temp {
                    locked_board[places.0][places.1] = true;
                }
            }
        }

        if !$checkcheck {
            $allowed_board
        } else {
            locked_board
        }
    }};
}
