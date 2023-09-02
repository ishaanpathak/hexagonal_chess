use crate::pieces::{PieceColor, PieceType, Piece, get_all_pieces};
use crate::moves::{get_legal_moves, MoveList};
use crate::board::Board;

pub fn is_check(board: &Board, color_to_check: PieceColor) -> bool {
    // find color_to_check's King's coordinates on the board
    let king_position = {
        let mut king_coordinates: Vec<(usize, usize)> = Vec::new();
        let all_pieces = get_all_pieces(board, color_to_check);
        for piece in all_pieces {
            if piece.piece.piece_type == PieceType::King {
                king_coordinates.push((piece.x, piece.y));
            }
        }
        king_coordinates
    };

    let mut check = false;
    let opponent: PieceColor = match color_to_check {
        PieceColor::Black => PieceColor::White,
        PieceColor::White => PieceColor::Black,
        PieceColor::None => PieceColor::None
    };
    let mut opponent_moves: Vec<(usize, usize)> = Vec::new();
    
    let opponent_pieces: Vec<Piece> = get_all_pieces(board, opponent);

    for piece in opponent_pieces {
        let mut current_piece_moves = get_legal_moves(board, &(piece.x, piece.y));
        opponent_moves.extend(current_piece_moves.drain(..));
    }

    if king_position.len() > 0 {
        if opponent_moves.contains(&king_position[0]) {
            check = true;
        }
    }
    check
}

/// Function for checking if the player is in Checkmate or Stalemate
/// 
/// If player is in check, and this function returns false, Checkmate
/// If player is not in check, and this function returns false, Stalemate
pub fn has_legal_moves(move_list: MoveList) -> bool {
    if move_list.len() == 0 {
        return false;
    }
    true
}