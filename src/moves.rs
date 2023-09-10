use crate::pieces::{ChessPiece, PieceColor, PieceType};
use crate::board::Board;
use crate::validation::is_check;
use std::collections::HashMap;

pub type MoveList = HashMap<(usize, usize), Vec<(usize, usize)>>;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MoveDirection {
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

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Move {
    pub piece: ChessPiece,
    pub from: (usize, usize),
    pub to: (usize, usize)
}

pub fn is_coordinate_in_bounds(coordinates: &(usize, usize)) -> bool {
    let (x, y) = *coordinates;
    if x + y < 5 || x + y > 15 || (x >= 11 || y >= 11) {
        return false;
    }
    true
}

pub fn get_move_coordinate(direction: MoveDirection, coordinates: &(usize, usize)) -> (usize, usize) {
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

pub fn get_pawn_moves(board: &Board, current_coordinates: &(usize, usize)) -> Vec<(usize, usize)> {
    let mut legal_moves: Vec<(usize, usize)> = Vec::new();    
    let mut forward: MoveDirection = MoveDirection::Up;
    let mut up_left: MoveDirection = MoveDirection::LeftUp;
    let mut up_right: MoveDirection = MoveDirection::RightUp;
    let mut starting_positions: [(usize, usize); 9] = [
        (6,9), (6,8), (6,7), (6,6), (6,5), (7,4), (8,3), (9,2), (10,1)
    ];
    if let Some(current_piece) = board.0[current_coordinates.0][current_coordinates.1] {
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
        if let Some(left_up_piece) = board.0[next_coordinate.0][next_coordinate.1] {
            if (left_up_piece.color != PieceColor::None) && (left_up_piece.color != current_piece.color) {
                legal_moves.push(next_coordinate);
            }
        }
        // Check up_right
        next_coordinate = get_move_coordinate(up_right, current_coordinates);
        if let Some(right_up_piece) = board.0[next_coordinate.0][next_coordinate.1] {
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
            if let Some(next_piece) = board.0[next_coordinate.0][next_coordinate.1] {
                if next_piece.color == PieceColor::None {
                    legal_moves.push(next_coordinate);
                }
            }
            coordinate = next_coordinate;
        }
    }
    
    legal_moves
}

pub fn get_knight_moves(board: &Board, current_coordinates: &(usize, usize)) -> Vec<(usize, usize)> {
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
    fn is_not_overlapping(board: &Board, current_coordinates: &(usize, usize), next_coordinates: &(usize, usize)) -> bool {
        if let Some(next_piece) = board.0[next_coordinates.0][next_coordinates.1] {
            if let Some(current_piece) = board.0[current_coordinates.0][current_coordinates.1] {
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

pub fn get_bishop_moves(board: &Board, current_coordinates: &(usize, usize)) -> Vec<(usize, usize)> {
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
                if let Some(next_piece) = board.0[next_coordinate.0][next_coordinate.1] {
                    if let Some(current_piece) = board.0[current_coordinates.0][current_coordinates.1] {
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

pub fn get_rook_moves(board: &Board, current_coordinates: &(usize, usize)) -> Vec<(usize, usize)> {
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
                if let Some(next_piece) = board.0[next_coordinate.0][next_coordinate.1] {
                    if let Some(current_piece) = board.0[current_coordinates.0][current_coordinates.1] {
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

pub fn get_queen_moves(board: &Board, current_coordinates: &(usize, usize)) -> Vec<(usize, usize)> {
    let mut legal_moves: Vec<(usize, usize)> = Vec::new();
    let mut regular_moves: Vec<(usize, usize)> = get_rook_moves(board, current_coordinates);
    let mut diagonal_moves: Vec<(usize, usize)> = get_bishop_moves(board, current_coordinates);
    legal_moves.extend(regular_moves.drain(..));
    legal_moves.extend(diagonal_moves.drain(..));
    legal_moves
}

pub fn get_king_moves(board: &Board, current_coordinates: &(usize, usize)) -> Vec<(usize, usize)> {
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
            if let Some(current_piece) = board.0[current_coordinates.0][current_coordinates.1] {
                if let Some(next_piece) = board.0[next_coordinate.0][next_coordinate.1] {
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

pub fn get_legal_moves(board: &Board, coordinates: &(usize, usize)) -> Vec<(usize, usize)> {
    let mut legal_moves: Vec<(usize, usize)> = Vec::new();
    if let Some(piece) = board.0[coordinates.0][coordinates.1] { match piece.color {
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

/// Function to actually change the place of a Piece on the Board.
pub fn execute_move(board: &mut Board, move_info: &Move) {
    let to: (usize, usize) = move_info.to;
    let from: (usize, usize) = move_info.from;

    board.0[to.0][to.1] = Some(move_info.piece);
    
    board.0[from.0][from.1] = Some(ChessPiece {
        piece_type: PieceType::None,
        color: PieceColor::None
    });
}

/// Removes the moves that Keep King in Check or lead King to Check
pub fn remove_check_moves(board: &Board, move_list: &mut MoveList) {
    let mut king_color: PieceColor = PieceColor::None;
    for (from, _) in move_list.iter() {
        if let Some(piece) = board.0[from.0][from.1] {
            if piece.piece_type == PieceType::King {
                king_color = piece.color;
            }
        }
    }
    for (from, moves) in move_list.clone().iter() {
        let mut filtered_moves: Vec<(usize, usize)> = Vec::new();
        for to in moves.iter() {
            let mut board_copy = board.clone();
            let move_info = Move {
                piece: board_copy.0[from.0][from.1].unwrap(),
                from: *from,
                to: *to
            };
            execute_move(&mut board_copy, &move_info);
            if !is_check(&board_copy, king_color) {
                filtered_moves.push(*to);
            }
        }
        move_list.insert(*from, filtered_moves);
    }
    move_list.retain(|_, moves| moves.len() > 0);
}
