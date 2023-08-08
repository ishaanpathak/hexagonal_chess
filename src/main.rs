use std::fs;

#[derive(Clone, Copy, Debug, PartialEq)]
enum PieceType {
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King,
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum PieceColor {
    White,
    Black,
}

#[derive(Clone, Copy, Debug, PartialEq)]
struct ChessPiece {
    piece_type: PieceType,
    color: PieceColor,
}

enum MoveDirection {
    // Regular Move Directions
    Up,
    Down,
    LeftUp,
    LeftDown,
    RightUp,
    RightDown,

    // Diagonal Move Directions
    DiagonalLeft,
    DiagonalRight,
    DiagonalLeftUp,
    DiagonalLeftDown,
    DiagonalRightUp,
    DiagonalRightDown,
}

fn decode_coordinates_bitwise(encoded_value: usize) -> (usize, usize) {
    let x = (encoded_value & 0xF0) >> 4;
    let y = encoded_value & 0x0F;
    (x, y)
}

fn decode_piece(piece_code: &str) -> Option<ChessPiece> {
    let color_code = piece_code.chars().nth(0).unwrap();
    let type_code = piece_code.chars().nth(1).unwrap();
    
    Some(ChessPiece {
        piece_type: {
            match type_code {
                'B' => { PieceType::Bishop },
                'K' => { PieceType::King },
                'N' => { PieceType::Knight },
                'Q' => { PieceType::Queen },
                'R' => { PieceType::Rook },
                _ => { PieceType::Pawn }
            }
        },
        color: {
            match color_code {
                'B' => { PieceColor::Black },
                'W' => { PieceColor::White },
                _ => { PieceColor::White },
            }
        },
    })
}

fn get_board_from_file(file_path: &str) -> [[Option<ChessPiece>; 11]; 11] {

    let mut board: [[Option<ChessPiece>; 11]; 11] = [[None; 11]; 11];
    
    let input_string = fs::read_to_string(file_path)
        .expect("Failed to read the file");
    
    let cleaned_string = input_string.replace("\n", "").trim().to_string();

    let values: Vec<&str> = cleaned_string.split(";").collect();

    for value in values {        
        let parts: Vec<&str> = value.split(":").collect();
        let encoded_value: usize = parts[0].parse()
            .expect("Failed to parse the encoded value.");
        let (i, j) = decode_coordinates_bitwise(encoded_value);
        let piece = decode_piece(parts[1]);
        board[i][j] = piece;
    }

    board
}

fn get_move_coordinate(direction: MoveDirection, coordinates: &(usize, usize)) -> (usize, usize) {
    let (x, y) = *coordinates;
    match direction {
        MoveDirection::Up => (x-1, y),
        MoveDirection::Down => (x+1, y),

        MoveDirection::LeftUp => (x-1, y+1),
        MoveDirection::LeftDown => (x, y+1),
        
        MoveDirection::RightUp => (x, y-1),
        MoveDirection::RightDown => (x+1, y-1),
        
        MoveDirection::DiagonalLeft => (x-1, y+2),
        MoveDirection::DiagonalRight => (x+1, y-2),
        
        MoveDirection::DiagonalLeftUp => (x-2, y+1),
        MoveDirection::DiagonalLeftDown => (x+1, y+1),
        
        MoveDirection::DiagonalRightUp => (x-1, y-1),
        MoveDirection::DiagonalRightDown => (x+2, y-1)
    }
}


fn reset_board(board: &mut [[Option<ChessPiece>; 11]; 11]) {
    // Black Pawns
    board[0][9] = Some(ChessPiece{piece_type: PieceType::Pawn, color: PieceColor::Black});
    board[1][8] = Some(ChessPiece{piece_type: PieceType::Pawn, color: PieceColor::Black});
    board[2][7] = Some(ChessPiece{piece_type: PieceType::Pawn, color: PieceColor::Black});
    board[3][6] = Some(ChessPiece{piece_type: PieceType::Pawn, color: PieceColor::Black});
    board[4][5] = Some(ChessPiece{piece_type: PieceType::Pawn, color: PieceColor::Black});
    board[4][4] = Some(ChessPiece{piece_type: PieceType::Pawn, color: PieceColor::Black});
    board[4][3] = Some(ChessPiece{piece_type: PieceType::Pawn, color: PieceColor::Black});
    board[4][2] = Some(ChessPiece{piece_type: PieceType::Pawn, color: PieceColor::Black});
    board[4][1] = Some(ChessPiece{piece_type: PieceType::Pawn, color: PieceColor::Black});

    // White Pawns
    board[6][9] = Some(ChessPiece{piece_type: PieceType::Pawn, color: PieceColor::White});
    board[6][8] = Some(ChessPiece{piece_type: PieceType::Pawn, color: PieceColor::White});
    board[6][7] = Some(ChessPiece{piece_type: PieceType::Pawn, color: PieceColor::White});
    board[6][6] = Some(ChessPiece{piece_type: PieceType::Pawn, color: PieceColor::White});
    board[6][5] = Some(ChessPiece{piece_type: PieceType::Pawn, color: PieceColor::White});
    board[7][4] = Some(ChessPiece{piece_type: PieceType::Pawn, color: PieceColor::White});
    board[8][3] = Some(ChessPiece{piece_type: PieceType::Pawn, color: PieceColor::White});
    board[9][2] = Some(ChessPiece{piece_type: PieceType::Pawn, color: PieceColor::White});
    board[10][1] = Some(ChessPiece{piece_type: PieceType::Pawn, color: PieceColor::White});

    // Black Knights
    board[0][7] = Some(ChessPiece{piece_type: PieceType::Knight, color: PieceColor::Black});
    board[2][3] = Some(ChessPiece{piece_type: PieceType::Knight, color: PieceColor::Black});

    // White Knights
    board[8][7] = Some(ChessPiece{piece_type: PieceType::Knight, color: PieceColor::White});
    board[10][3] = Some(ChessPiece{piece_type: PieceType::Knight, color: PieceColor::White});

    // Black Rooks
    board[0][8] = Some(ChessPiece{piece_type: PieceType::Rook, color: PieceColor::Black});
    board[3][2] = Some(ChessPiece{piece_type: PieceType::Rook, color: PieceColor::Black});
    
    // White Rooks
    board[7][8] = Some(ChessPiece{piece_type: PieceType::Rook, color: PieceColor::White});
    board[10][2] = Some(ChessPiece{piece_type: PieceType::Rook, color: PieceColor::White});

    // Black Queen
    board[0][6] = Some(ChessPiece {piece_type: PieceType::Queen, color: PieceColor::Black});
    
    // White Queen
    board[9][6] = Some(ChessPiece {piece_type: PieceType::Queen, color: PieceColor::White});

    // Black King
    board[1][4] = Some(ChessPiece {piece_type: PieceType::King, color: PieceColor::Black});
    
    // White King
    board[10][4] = Some(ChessPiece {piece_type: PieceType::King, color: PieceColor::White});

    // Black Bishops
    board[0][5] = Some(ChessPiece {piece_type: PieceType::Bishop, color: PieceColor::Black});
    board[1][5] = Some(ChessPiece {piece_type: PieceType::Bishop, color: PieceColor::Black});
    board[2][5] = Some(ChessPiece {piece_type: PieceType::Bishop, color: PieceColor::Black});

    // Black Bishops
    board[8][5] = Some(ChessPiece {piece_type: PieceType::Bishop, color: PieceColor::White});
    board[9][5] = Some(ChessPiece {piece_type: PieceType::Bishop, color: PieceColor::White});
    board[10][5] = Some(ChessPiece {piece_type: PieceType::Bishop, color: PieceColor::White});
}

fn main() {

    // let mut board: [[Option<ChessPiece>; 11]; 11] = [[None; 11]; 11];
    let mut board = get_board_from_file("C:/Users/ishaan/Documents/HexaChess/Board Loader/src/pawn_test.txt");

    // reset_board(&mut board);

    for (row_index, row) in board.iter().enumerate() {

        // Print cells in the current row
        for (column_index, cell) in row.iter().enumerate() {
            if (row_index + column_index > 4) && (row_index + column_index < 16) {
                match cell {
                    Some(piece) => {
                        match piece.color {
                            PieceColor::Black => {
                                match piece.piece_type {
                                    PieceType::King => print!("♚ "),
                                    PieceType::Queen => print!("♛ "),
                                    PieceType::Rook => print!("♜ "),
                                    PieceType::Bishop => print!("♝ "),
                                    PieceType::Knight => print!("♞ "),
                                    PieceType::Pawn => print!("♟ "),
                                }
                            },
                            PieceColor::White => {
                                match piece.piece_type {
                                    PieceType::King => print!("♔ "),
                                    PieceType::Queen => print!("♕ "),
                                    PieceType::Rook => print!("♖ "),
                                    PieceType::Bishop => print!("♗ "),
                                    PieceType::Knight => print!("♘ "),
                                    PieceType::Pawn => print!("♙ "),
                                }
                            }
                        }
                    }
                    None => print!("- "),
                }
            } else {
                print!(". ");
            }
        }
        println!();
    }
}
