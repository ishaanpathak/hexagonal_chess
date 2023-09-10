use crate::pieces::*;
use std::fs;

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Board(pub [[Option<ChessPiece>; 11]; 11]);

pub fn decode_coordinates_bitwise(encoded_value: usize) -> (usize, usize) {
    let x = (encoded_value & 0xF0) >> 4;
    let y = encoded_value & 0x0F;
    (x, y)
}

pub fn decode_piece(piece_code: &str) -> Option<ChessPiece> {
    let color_code = piece_code.chars().nth(0).unwrap();
    let type_code = piece_code.chars().nth(1).unwrap();

    Some(ChessPiece {
        piece_type: {
            match type_code {
                'B' => PieceType::Bishop,
                'K' => PieceType::King,
                'N' => PieceType::Knight,
                'Q' => PieceType::Queen,
                'R' => PieceType::Rook,
                _ => PieceType::Pawn,
            }
        },
        color: {
            match color_code {
                'B' => PieceColor::Black,
                'W' => PieceColor::White,
                _ => PieceColor::White,
            }
        },
    })
}

pub fn get_board_from_file(file_path: &str) -> Board {
    let mut board: Board = Board([[Some(ChessPiece {
        piece_type: PieceType::None,
        color: PieceColor::None,
    }); 11]; 11]);

    let input_string = fs::read_to_string(file_path).expect("Failed to read the file");

    let cleaned_string = input_string.replace("\n", "").trim().to_string();

    let cleaned_string_with_semicolon = if !cleaned_string.ends_with(';') {
        format!("{};", cleaned_string)
    } else {
        cleaned_string
    };

    let values: Vec<&str> = cleaned_string_with_semicolon.split(";").collect();

    for value in values {
        let parts: Vec<&str> = value.split(":").collect();
        if parts.len() == 2 && !parts[0].is_empty() && !parts[1].is_empty() {
            let encoded_value: usize = parts[0]
                .parse()
                .expect("Failed to parse the encoded value.");
            let (i, j) = decode_coordinates_bitwise(encoded_value);
            let piece = decode_piece(parts[1]);
            board.0[i][j] = piece;
        }
    }

    board
}

pub fn get_default_board() -> Board {
    let mut board: Board = Board([[Some(ChessPiece {
        piece_type: PieceType::None,
        color: PieceColor::None,
    }); 11]; 11]);
    // Black Pawns
    board.0[0][9] = Some(ChessPiece {
        piece_type: PieceType::Pawn,
        color: PieceColor::Black,
    });
    board.0[1][8] = Some(ChessPiece {
        piece_type: PieceType::Pawn,
        color: PieceColor::Black,
    });
    board.0[2][7] = Some(ChessPiece {
        piece_type: PieceType::Pawn,
        color: PieceColor::Black,
    });
    board.0[3][6] = Some(ChessPiece {
        piece_type: PieceType::Pawn,
        color: PieceColor::Black,
    });
    board.0[4][5] = Some(ChessPiece {
        piece_type: PieceType::Pawn,
        color: PieceColor::Black,
    });
    board.0[4][4] = Some(ChessPiece {
        piece_type: PieceType::Pawn,
        color: PieceColor::Black,
    });
    board.0[4][3] = Some(ChessPiece {
        piece_type: PieceType::Pawn,
        color: PieceColor::Black,
    });
    board.0[4][2] = Some(ChessPiece {
        piece_type: PieceType::Pawn,
        color: PieceColor::Black,
    });
    board.0[4][1] = Some(ChessPiece {
        piece_type: PieceType::Pawn,
        color: PieceColor::Black,
    });

    // White Pawns
    board.0[6][9] = Some(ChessPiece {
        piece_type: PieceType::Pawn,
        color: PieceColor::White,
    });
    board.0[6][8] = Some(ChessPiece {
        piece_type: PieceType::Pawn,
        color: PieceColor::White,
    });
    board.0[6][7] = Some(ChessPiece {
        piece_type: PieceType::Pawn,
        color: PieceColor::White,
    });
    board.0[6][6] = Some(ChessPiece {
        piece_type: PieceType::Pawn,
        color: PieceColor::White,
    });
    board.0[6][5] = Some(ChessPiece {
        piece_type: PieceType::Pawn,
        color: PieceColor::White,
    });
    board.0[7][4] = Some(ChessPiece {
        piece_type: PieceType::Pawn,
        color: PieceColor::White,
    });
    board.0[8][3] = Some(ChessPiece {
        piece_type: PieceType::Pawn,
        color: PieceColor::White,
    });
    board.0[9][2] = Some(ChessPiece {
        piece_type: PieceType::Pawn,
        color: PieceColor::White,
    });
    board.0[10][1] = Some(ChessPiece {
        piece_type: PieceType::Pawn,
        color: PieceColor::White,
    });

    // Black Knights
    board.0[0][7] = Some(ChessPiece {
        piece_type: PieceType::Knight,
        color: PieceColor::Black,
    });
    board.0[2][3] = Some(ChessPiece {
        piece_type: PieceType::Knight,
        color: PieceColor::Black,
    });

    // White Knights
    board.0[8][7] = Some(ChessPiece {
        piece_type: PieceType::Knight,
        color: PieceColor::White,
    });
    board.0[10][3] = Some(ChessPiece {
        piece_type: PieceType::Knight,
        color: PieceColor::White,
    });

    // Black Rooks
    board.0[0][8] = Some(ChessPiece {
        piece_type: PieceType::Rook,
        color: PieceColor::Black,
    });
    board.0[3][2] = Some(ChessPiece {
        piece_type: PieceType::Rook,
        color: PieceColor::Black,
    });

    // White Rooks
    board.0[7][8] = Some(ChessPiece {
        piece_type: PieceType::Rook,
        color: PieceColor::White,
    });
    board.0[10][2] = Some(ChessPiece {
        piece_type: PieceType::Rook,
        color: PieceColor::White,
    });

    // Black Queen
    board.0[0][6] = Some(ChessPiece {
        piece_type: PieceType::Queen,
        color: PieceColor::Black,
    });

    // White Queen
    board.0[9][6] = Some(ChessPiece {
        piece_type: PieceType::Queen,
        color: PieceColor::White,
    });

    // Black King
    board.0[1][4] = Some(ChessPiece {
        piece_type: PieceType::King,
        color: PieceColor::Black,
    });

    // White King
    board.0[10][4] = Some(ChessPiece {
        piece_type: PieceType::King,
        color: PieceColor::White,
    });

    // Black Bishops
    board.0[0][5] = Some(ChessPiece {
        piece_type: PieceType::Bishop,
        color: PieceColor::Black,
    });
    board.0[1][5] = Some(ChessPiece {
        piece_type: PieceType::Bishop,
        color: PieceColor::Black,
    });
    board.0[2][5] = Some(ChessPiece {
        piece_type: PieceType::Bishop,
        color: PieceColor::Black,
    });

    // Black Bishops
    board.0[8][5] = Some(ChessPiece {
        piece_type: PieceType::Bishop,
        color: PieceColor::White,
    });
    board.0[9][5] = Some(ChessPiece {
        piece_type: PieceType::Bishop,
        color: PieceColor::White,
    });
    board.0[10][5] = Some(ChessPiece {
        piece_type: PieceType::Bishop,
        color: PieceColor::White,
    });

    board
}

pub fn print_board(board: &Board) {
    for (row_index, row) in board.0.iter().enumerate() {
        // Print cells in the current row
        for (column_index, cell) in row.iter().enumerate() {
            if (row_index + column_index > 4) && (row_index + column_index < 16) {
                match cell {
                    Some(piece) => match piece.color {
                        PieceColor::Black => match piece.piece_type {
                            PieceType::King => print!("♚ "),
                            PieceType::Queen => print!("♛ "),
                            PieceType::Rook => print!("♜ "),
                            PieceType::Bishop => print!("♝ "),
                            PieceType::Knight => print!("♞ "),
                            PieceType::Pawn => print!("♟ "),
                            PieceType::None => print!("_ "),
                        },
                        PieceColor::White => match piece.piece_type {
                            PieceType::King => print!("♔ "),
                            PieceType::Queen => print!("♕ "),
                            PieceType::Rook => print!("♖ "),
                            PieceType::Bishop => print!("♗ "),
                            PieceType::Knight => print!("♘ "),
                            PieceType::Pawn => print!("♙ "),
                            PieceType::None => print!("_ "),
                        },
                        PieceColor::None => {
                            print!("_ ");
                        }
                    },
                    None => print!("- "),
                }
            } else {
                print!(". ");
            }
        }
        println!();
    }
}
