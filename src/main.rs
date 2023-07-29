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


fn reset_board(board: &mut Vec<Vec<Option<ChessPiece>>>) {
    // Black Pawns
    board[1][0] = Some(ChessPiece{piece_type: PieceType::Pawn, color: PieceColor::Black});
    board[2][1] = Some(ChessPiece{piece_type: PieceType::Pawn, color: PieceColor::Black});
    board[3][2] = Some(ChessPiece{piece_type: PieceType::Pawn, color: PieceColor::Black});
    board[4][3] = Some(ChessPiece{piece_type: PieceType::Pawn, color: PieceColor::Black});
    board[5][4] = Some(ChessPiece{piece_type: PieceType::Pawn, color: PieceColor::Black});
    board[6][3] = Some(ChessPiece{piece_type: PieceType::Pawn, color: PieceColor::Black});
    board[7][2] = Some(ChessPiece{piece_type: PieceType::Pawn, color: PieceColor::Black});
    board[8][1] = Some(ChessPiece{piece_type: PieceType::Pawn, color: PieceColor::Black});
    board[9][0] = Some(ChessPiece{piece_type: PieceType::Pawn, color: PieceColor::Black});

    // White Pawns
    board[1][6] = Some(ChessPiece{piece_type: PieceType::Pawn, color: PieceColor::White});
    board[2][6] = Some(ChessPiece{piece_type: PieceType::Pawn, color: PieceColor::White});
    board[3][6] = Some(ChessPiece{piece_type: PieceType::Pawn, color: PieceColor::White});
    board[4][6] = Some(ChessPiece{piece_type: PieceType::Pawn, color: PieceColor::White});
    board[5][6] = Some(ChessPiece{piece_type: PieceType::Pawn, color: PieceColor::White});
    board[6][6] = Some(ChessPiece{piece_type: PieceType::Pawn, color: PieceColor::White});
    board[7][6] = Some(ChessPiece{piece_type: PieceType::Pawn, color: PieceColor::White});
    board[8][6] = Some(ChessPiece{piece_type: PieceType::Pawn, color: PieceColor::White});
    board[9][6] = Some(ChessPiece{piece_type: PieceType::Pawn, color: PieceColor::White});

    // Black Knights
    board[3][0] = Some(ChessPiece{piece_type: PieceType::Knight, color: PieceColor::Black});
    board[7][0] = Some(ChessPiece{piece_type: PieceType::Knight, color: PieceColor::Black});

    // White Knights
    board[3][8] = Some(ChessPiece{piece_type: PieceType::Knight, color: PieceColor::White});
    board[7][8] = Some(ChessPiece{piece_type: PieceType::Knight, color: PieceColor::White});

    // Black Rooks
    board[2][0] = Some(ChessPiece{piece_type: PieceType::Rook, color: PieceColor::Black});
    board[8][0] = Some(ChessPiece{piece_type: PieceType::Rook, color: PieceColor::Black});
    
    // White Rooks
    board[2][7] = Some(ChessPiece{piece_type: PieceType::Rook, color: PieceColor::White});
    board[8][7] = Some(ChessPiece{piece_type: PieceType::Rook, color: PieceColor::White});

    // Black Queen
    board[4][0] = Some(ChessPiece {piece_type: PieceType::Queen, color: PieceColor::Black});
    
    // White Queen
    board[6][9] = Some(ChessPiece {piece_type: PieceType::Queen, color: PieceColor::White});

    // Black King
    board[6][0] = Some(ChessPiece {piece_type: PieceType::King, color: PieceColor::Black});
    
    // White King
    board[4][9] = Some(ChessPiece {piece_type: PieceType::King, color: PieceColor::White});

    // Black Bishops
    board[5][0] = Some(ChessPiece {piece_type: PieceType::Bishop, color: PieceColor::Black});
    board[5][1] = Some(ChessPiece {piece_type: PieceType::Bishop, color: PieceColor::Black});
    board[5][2] = Some(ChessPiece {piece_type: PieceType::Bishop, color: PieceColor::Black});

    // Black Bishops
    board[5][8] = Some(ChessPiece {piece_type: PieceType::Bishop, color: PieceColor::White});
    board[5][9] = Some(ChessPiece {piece_type: PieceType::Bishop, color: PieceColor::White});
    board[5][10] = Some(ChessPiece {piece_type: PieceType::Bishop, color: PieceColor::White});
}


// | Direction  | Left Side | Center Line | Right Side |
// | ---------- | --------- | ----------- | ---------- |
// | Up         | x,   y+1  | x,   y+1    | x,   y+1   |
// | Down       | x,   y-1  | x,   y-1    | x,   y-1   |
// | Left-Up    | x+1, y-1  | x+1, y-1    | x+1, y     |
// | Left-Down  | x+1, y    | x+1, y      | x+1, y+1   |
// | Right-Up   | x-1, y    | x-1, y-1    | x-1, y-1   |
// | Right-Down | x-1, y+1  | x-1, y      | x-1, y     |

fn main() {


    let mut board: Vec<Vec<Option<ChessPiece>>> = vec![
        vec![None; 0x6],
        vec![None; 0x7],
        vec![None; 0x8],
        vec![None; 0x9],
        vec![None; 0xA],
        vec![None; 0xB],
        vec![None; 0xA],
        vec![None; 0x9],
        vec![None; 0x8],
        vec![None; 0x7],
        vec![None; 0x6],
    ];

    reset_board(&mut board);

    // // Black Pawns
    // board[1][0] = Some(ChessPiece{piece_type: PieceType::Pawn, color: PieceColor::Black});
    // board[2][1] = Some(ChessPiece{piece_type: PieceType::Pawn, color: PieceColor::Black});
    // board[3][2] = Some(ChessPiece{piece_type: PieceType::Pawn, color: PieceColor::Black});
    // board[4][3] = Some(ChessPiece{piece_type: PieceType::Pawn, color: PieceColor::Black});
    // board[5][4] = Some(ChessPiece{piece_type: PieceType::Pawn, color: PieceColor::Black});
    // board[6][3] = Some(ChessPiece{piece_type: PieceType::Pawn, color: PieceColor::Black});
    // board[7][2] = Some(ChessPiece{piece_type: PieceType::Pawn, color: PieceColor::Black});
    // board[8][1] = Some(ChessPiece{piece_type: PieceType::Pawn, color: PieceColor::Black});
    // board[9][0] = Some(ChessPiece{piece_type: PieceType::Pawn, color: PieceColor::Black});

    // // White Pawns
    // board[1][6] = Some(ChessPiece{piece_type: PieceType::Pawn, color: PieceColor::White});
    // board[2][6] = Some(ChessPiece{piece_type: PieceType::Pawn, color: PieceColor::White});
    // board[3][6] = Some(ChessPiece{piece_type: PieceType::Pawn, color: PieceColor::White});
    // board[4][6] = Some(ChessPiece{piece_type: PieceType::Pawn, color: PieceColor::White});
    // board[5][6] = Some(ChessPiece{piece_type: PieceType::Pawn, color: PieceColor::White});
    // board[6][6] = Some(ChessPiece{piece_type: PieceType::Pawn, color: PieceColor::White});
    // board[7][6] = Some(ChessPiece{piece_type: PieceType::Pawn, color: PieceColor::White});
    // board[8][6] = Some(ChessPiece{piece_type: PieceType::Pawn, color: PieceColor::White});
    // board[9][6] = Some(ChessPiece{piece_type: PieceType::Pawn, color: PieceColor::White});

    // // Black Knights
    // board[3][0] = Some(ChessPiece{piece_type: PieceType::Knight, color: PieceColor::Black});
    // board[7][0] = Some(ChessPiece{piece_type: PieceType::Knight, color: PieceColor::Black});

    // // White Knights
    // board[3][8] = Some(ChessPiece{piece_type: PieceType::Knight, color: PieceColor::White});
    // board[7][8] = Some(ChessPiece{piece_type: PieceType::Knight, color: PieceColor::White});

    // // Black Rooks
    // board[2][0] = Some(ChessPiece{piece_type: PieceType::Rook, color: PieceColor::Black});
    // board[8][0] = Some(ChessPiece{piece_type: PieceType::Rook, color: PieceColor::Black});
    
    // // White Rooks
    // board[2][7] = Some(ChessPiece{piece_type: PieceType::Rook, color: PieceColor::White});
    // board[8][7] = Some(ChessPiece{piece_type: PieceType::Rook, color: PieceColor::White});

    // // Black Queen
    // board[4][0] = Some(ChessPiece {piece_type: PieceType::Queen, color: PieceColor::Black});
    
    // // White Queen
    // board[6][9] = Some(ChessPiece {piece_type: PieceType::Queen, color: PieceColor::White});

    // // Black King
    // board[6][0] = Some(ChessPiece {piece_type: PieceType::King, color: PieceColor::Black});
    
    // // White King
    // board[4][9] = Some(ChessPiece {piece_type: PieceType::King, color: PieceColor::White});

    // // Black Bishops
    // board[5][0] = Some(ChessPiece {piece_type: PieceType::Bishop, color: PieceColor::Black});
    // board[5][1] = Some(ChessPiece {piece_type: PieceType::Bishop, color: PieceColor::Black});
    // board[5][2] = Some(ChessPiece {piece_type: PieceType::Bishop, color: PieceColor::Black});

    // // Black Bishops
    // board[5][8] = Some(ChessPiece {piece_type: PieceType::Bishop, color: PieceColor::White});
    // board[5][9] = Some(ChessPiece {piece_type: PieceType::Bishop, color: PieceColor::White});
    // board[5][10] = Some(ChessPiece {piece_type: PieceType::Bishop, color: PieceColor::White});

    for (row_index, row) in board.iter().enumerate() {
        // Print spaces for indentation based on row index
        // for _ in 0..(board.len() - row_index - 1) {
        //     print!("  ");
        // }

        // Print cells in the current row
        for cell in row {
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
                    // match piece.color {
                    //     PieceColor::Black => print!("B"),
                    //     PieceColor::White => print!("W"),
                    // }
                    // match piece.piece_type {
                    //     PieceType::Pawn => print!("P "),
                    //     PieceType::Knight => print!("N "),
                    //     PieceType::King => print!("K "),
                    //     PieceType::Queen => print!("Q "),
                    //     PieceType::Bishop => print!("B "),
                    //     PieceType::Rook => print!("R "),
                    // }
                }
                None => print!("- "),
            }
        }

        println!();
    }
}
