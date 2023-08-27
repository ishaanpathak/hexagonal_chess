use std::fs;

#[derive(Clone, Copy, Debug, PartialEq)]
enum PieceType {
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King,
    None,
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum PieceColor {
    White,
    Black,
    None,
}

#[derive(Clone, Copy, Debug, PartialEq)]
struct ChessPiece {
    piece_type: PieceType,
    color: PieceColor,
}

#[derive(Clone, Copy, Debug, PartialEq)]
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
    
    // let mut board: [[Option<ChessPiece>; 11]; 11] = [[None; 11]; 11];
    let mut board: [[Option<ChessPiece>; 11]; 11] = [[Some(ChessPiece { piece_type: PieceType::None, color: PieceColor::None }); 11]; 11];

    let input_string = fs::read_to_string(file_path)
        .expect("Failed to read the file");

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
            let encoded_value: usize = parts[0].parse()
                .expect("Failed to parse the encoded value.");
            let (i, j) = decode_coordinates_bitwise(encoded_value);
            let piece = decode_piece(parts[1]);
            board[i][j] = piece;
        }
    }

    board
}

fn is_coordinate_in_bounds(coordinates: &(usize, usize)) -> bool {
    let (x, y) = *coordinates;
    if x + y < 5 || x + y > 15 || (x >= 11 || y >= 11) {
        return false;
    }
    true
}

fn get_move_coordinate(direction: MoveDirection, coordinates: &(usize, usize)) -> (usize, usize) {
    let (x, y) = *coordinates;
    let fallback: (usize, usize) = (11, 11); // Fall back for negative values
    match direction {
        MoveDirection::Up => {
            if x >= 1 { (x-1, y) } else { fallback }
        },
        MoveDirection::Down => (x+1, y),

        MoveDirection::LeftUp => {
            if x >= 1 { (x-1, y+1) } else { fallback }
        },
        MoveDirection::LeftDown => (x, y+1),
        
        MoveDirection::RightUp => {
            if y >= 1 { (x, y-1) } else { fallback }
        },
        MoveDirection::RightDown => {
            if y >= 1 { (x+1, y-1) } else { fallback }
        },
        
        MoveDirection::DiagonalLeft => {
            if x >= 1 { (x-1, y+2) } else { fallback }
        },
        MoveDirection::DiagonalRight => {
            if y >= 2 { (x+1, y-2) } else { fallback }
        },
        
        MoveDirection::DiagonalLeftUp => {
            if x >= 2 { (x-2, y+1) } else { fallback }
        },
        MoveDirection::DiagonalLeftDown => (x+1, y+1),
        
        MoveDirection::DiagonalRightUp => {
            if x >= 1 && y >= 1 { (x-1, y-1) } else { fallback }
        },
        MoveDirection::DiagonalRightDown => {
            if y >= 1 { (x+2, y-1) } else { fallback }
        }
    }
}

fn get_pawn_moves(board: &[[Option<ChessPiece>; 11]; 11], current_coordinates: &(usize, usize)) -> Vec<(usize, usize)> {
    let mut legal_moves: Vec<(usize, usize)> = Vec::new();    
    let mut forward: MoveDirection = MoveDirection::Up;
    let mut up_left: MoveDirection = MoveDirection::LeftUp;
    let mut up_right: MoveDirection = MoveDirection::RightUp;
    let mut starting_positions: [(usize, usize); 9] = [
        (6,9), (6,8), (6,7), (6,6), (6,5), (7,4), (8,3), (9,2), (10,1)
    ];
    if let Some(current_piece) = board[current_coordinates.0][current_coordinates.1] {
        if current_piece.color == PieceColor::Black {
            forward = MoveDirection::Down;
            up_left = MoveDirection::RightDown;
            up_right = MoveDirection::LeftDown;
            starting_positions = [
                (0,9), (1,8), (2,7), (3,6), (4,5), (4,4), (4,3), (4,2), (4,1)
            ];
        }
        // Checking up_left
        let mut next_coordinate = get_move_coordinate(up_left, current_coordinates);
        if let Some(left_up_piece) = board[next_coordinate.0][next_coordinate.1] {
            if (left_up_piece.color != PieceColor::None) && (left_up_piece.color != current_piece.color) {
                legal_moves.push(next_coordinate);
            }
        }
        // Check up_right
        next_coordinate = get_move_coordinate(up_right, current_coordinates);
        if let Some(right_up_piece) = board[next_coordinate.0][next_coordinate.1] {
            if (right_up_piece.color != PieceColor::None) && (right_up_piece.color != current_piece.color) {
                legal_moves.push(next_coordinate);
            }
        }
        // Check forward
        let forward_count: usize = {
            if starting_positions.contains(current_coordinates) {
                2
            } else {
                1
            }
        };
        let mut coordinate = *current_coordinates;
        for _ in 1..=forward_count {
            next_coordinate = get_move_coordinate(forward, &coordinate);
            if let Some(next_piece) = board[next_coordinate.0][next_coordinate.1] {
                if next_piece.color == PieceColor::None {
                    legal_moves.push(next_coordinate);
                }
            }
            coordinate = next_coordinate;
        }
    }
    
    legal_moves
}

fn get_knight_moves(board: &[[Option<ChessPiece>; 11]; 11], current_coordinates: &(usize, usize)) -> Vec<(usize, usize)> {
    let mut legal_moves: Vec<(usize, usize)> = Vec::new();
    let (x, y) = current_coordinates;
    let conditions = [
        { x >= &1 }, { x >= &2 }, { x >= &3 },
        { y >= &1 }, { y >= &2 }, { y >= &3 }
    ];
    if conditions[2] {
        legal_moves.push((x-3, y+1));
        legal_moves.push((x-3, y+2));
    }
    if conditions[1] && conditions[3] {
        legal_moves.push((x-2, y-1));
    }
    if conditions[0] && conditions[4] {
        legal_moves.push((x-1, y-2));
    }
    if conditions[5] {
        legal_moves.push((x+1, y-3));
        legal_moves.push((x+2, y-3));
    }
    if conditions[4] {
        legal_moves.push((x+3, y-2));
    }
    if conditions[3] {
        legal_moves.push((x+3, y-1));
    }
    legal_moves.push((x+2, y+1));
    legal_moves.push((x+1, y+2));
    if conditions[0] {
        legal_moves.push((x-1, y+3));
    }
    if conditions[1] {
        legal_moves.push((x-2, y+3));
    }

    // Only keeping coordinates that are inside the board bounds
    legal_moves.retain(is_coordinate_in_bounds);

    // Function to check if any coordinate has the same colored piece
    fn is_not_overlapping(board: &[[Option<ChessPiece>; 11]; 11], current_coordinates: &(usize, usize), next_coordinates: &(usize, usize)) -> bool {
        if let Some(next_piece) = board[next_coordinates.0][next_coordinates.1] {
            if let Some(current_piece) = board[current_coordinates.0][current_coordinates.1] {
                if next_piece.color != PieceColor::None && next_piece.color == current_piece.color {
                    return false;
                }
            }
        }
        true
    }

    let is_valid_closure = |item: &(usize, usize)| -> bool {
        is_not_overlapping(board, current_coordinates, item)
    };

    legal_moves.retain(is_valid_closure);
    
    legal_moves
}

fn get_bishop_moves(board: &[[Option<ChessPiece>; 11]; 11], current_coordinates: &(usize, usize)) -> Vec<(usize, usize)> {
    let mut legal_moves: Vec<(usize, usize)> = Vec::new();
    let mut stop: bool = false;
    let mut coordinates = *current_coordinates;
    let check_directions = [
        MoveDirection::DiagonalLeftUp, MoveDirection::DiagonalLeft, MoveDirection::DiagonalLeftDown,
        MoveDirection::DiagonalRightUp, MoveDirection::DiagonalRight, MoveDirection::DiagonalRightDown,
    ];
    for direction in check_directions.iter() {
        while !stop {
            let next_coordinate = get_move_coordinate(*direction, &coordinates);
            if is_coordinate_in_bounds(&next_coordinate) {
                if let Some(next_piece) = board[next_coordinate.0][next_coordinate.1] {
                    if let Some(current_piece) = board[current_coordinates.0][current_coordinates.1] {
                        if next_piece.color != PieceColor::None { 
                            if next_piece.color != current_piece.color {
                                legal_moves.push(next_coordinate);
                                stop = true;
                            } else {
                                stop = true;
                            }
                        } else {
                            legal_moves.push(next_coordinate);
                            coordinates = next_coordinate;
                        }
                    }
                } else {
                    legal_moves.push(next_coordinate);
                    coordinates = next_coordinate;
                }
            } else {
                stop = true;
            }
        }
        coordinates = *current_coordinates;
        stop = false;
    }
    
    legal_moves
}

fn get_rook_moves(board: &[[Option<ChessPiece>; 11]; 11], current_coordinates: &(usize, usize)) -> Vec<(usize, usize)> {
    let mut legal_moves: Vec<(usize, usize)> = Vec::new();
    let mut stop: bool = false;
    let mut coordinates = *current_coordinates;
    let check_directions = [
        MoveDirection::LeftUp, MoveDirection::Up, MoveDirection::RightUp,
        MoveDirection::LeftDown, MoveDirection::Down, MoveDirection::RightDown,
    ];
    for direction in check_directions.iter() {
        while !stop {
            let next_coordinate = get_move_coordinate(*direction, &coordinates);
            if is_coordinate_in_bounds(&next_coordinate) {
                if let Some(next_piece) = board[next_coordinate.0][next_coordinate.1] {
                    if let Some(current_piece) = board[current_coordinates.0][current_coordinates.1] {
                        if next_piece.color != PieceColor::None { 
                            if next_piece.color != current_piece.color {
                                legal_moves.push(next_coordinate);
                                stop = true;
                            } else {
                                stop = true;
                            }
                        } else {
                            legal_moves.push(next_coordinate);
                            coordinates = next_coordinate;
                        }
                    }
                } else {
                    legal_moves.push(next_coordinate);
                    coordinates = next_coordinate;
                }
            } else {
                stop = true;
            }
        }
        coordinates = *current_coordinates;
        stop = false;
    }
    legal_moves
}

fn get_queen_moves(board: &[[Option<ChessPiece>; 11]; 11], current_coordinates: &(usize, usize)) -> Vec<(usize, usize)> {
    let mut legal_moves: Vec<(usize, usize)> = Vec::new();
    let mut regular_moves: Vec<(usize, usize)> = get_rook_moves(board, current_coordinates);
    let mut diagonal_moves: Vec<(usize, usize)> = get_bishop_moves(board, current_coordinates);
    legal_moves.extend(regular_moves.drain(..));
    legal_moves.extend(diagonal_moves.drain(..));
    legal_moves
}

fn get_king_moves(board: &[[Option<ChessPiece>; 11]; 11], current_coordinates: &(usize, usize)) -> Vec<(usize, usize)> {
    let mut legal_moves = Vec::new();
    let directions = [
        MoveDirection::Up,
        MoveDirection::DiagonalRightUp,
        MoveDirection::RightUp,
        MoveDirection::DiagonalRight,
        MoveDirection::RightDown,
        MoveDirection::DiagonalRightDown,
        MoveDirection::Down,
        MoveDirection::DiagonalLeftDown,
        MoveDirection::LeftDown,
        MoveDirection::DiagonalLeft,
        MoveDirection::LeftUp,
        MoveDirection::DiagonalLeftUp
    ];
    for direction in directions {
        let next_coordinate: (usize, usize) = get_move_coordinate(direction, current_coordinates);
        if is_coordinate_in_bounds(&next_coordinate) {
            if let Some(current_piece) = board[current_coordinates.0][current_coordinates.1] {
                if let Some(next_piece) = board[next_coordinate.0][next_coordinate.1] {
                    if next_piece.color != PieceColor::None {
                        if next_piece.color != current_piece.color {
                            legal_moves.push(next_coordinate);
                        }
                    } else {
                        legal_moves.push(next_coordinate);
                    }
                }
            }
        }
    }
    legal_moves
}

fn _reset_board(board: &mut [[Option<ChessPiece>; 11]; 11]) {
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


fn get_legal_moves(board: &[[Option<ChessPiece>; 11]; 11], coordinates: &(usize, usize)) -> Vec<(usize, usize)> {
    let mut legal_moves: Vec<(usize, usize)> = Vec::new();
    if let Some(piece) = board[coordinates.0][coordinates.1] { match piece.color {
        PieceColor::White => {
            match piece.piece_type {
                PieceType::Pawn => {
                    legal_moves = get_pawn_moves(&board, coordinates);
                },
                PieceType::Knight => {
                    legal_moves = get_knight_moves(&board, coordinates);
                },
                PieceType::Bishop => {
                    legal_moves = get_bishop_moves(&board, coordinates);
                },
                PieceType::Rook => {
                    legal_moves = get_rook_moves(&board, coordinates);
                },
                PieceType::Queen => {
                    legal_moves = get_queen_moves(&board, coordinates);
                },
                PieceType::King => {
                    legal_moves = get_king_moves(&board, coordinates);
                },
                PieceType::None => {
                    legal_moves = vec![*coordinates];
                }
            }
        },
        PieceColor::Black => {
            match piece.piece_type {
                PieceType::Pawn => {
                    legal_moves = get_pawn_moves(&board, coordinates);
                },
                PieceType::Knight => {
                    legal_moves = get_knight_moves(&board, coordinates);
                },
                PieceType::Bishop => {
                    legal_moves = get_bishop_moves(&board, coordinates);
                },
                PieceType::Rook => {
                    legal_moves = get_rook_moves(&board, coordinates);
                },
                PieceType::Queen => {
                    legal_moves = get_queen_moves(&board, coordinates);
                },
                PieceType::King => {
                    legal_moves = get_king_moves(&board, coordinates);
                },
                PieceType::None => {
                    legal_moves = vec![*coordinates];
                }
            }
        },
        PieceColor::None => {
            legal_moves = vec![*coordinates];
        }
    } }
    legal_moves
}

fn main() {

    // let mut board: [[Option<ChessPiece>; 11]; 11] = [[None; 11]; 11];
    let mut board = get_board_from_file("/run/media/ishaan/Windows/Users/ishaan/Documents/HexaChess/Python Board Generator/output/pawn_test_new.txt");

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
                                    PieceType::None => print!("_ "),
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
                                    PieceType::None => print!("_ "),
                                }
                            },
                            PieceColor::None => {
                                print!("_ ");
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

    println!("{:?}", get_legal_moves(&board, &(6,5)));
    println!("{:?}", get_legal_moves(&board, &(6,8)));
    println!("{:?}", get_legal_moves(&board, &(4,5)));
    println!("{:?}", get_legal_moves(&board, &(4,3)));
    println!("{:?}", get_legal_moves(&board, &(3,7)));
}
