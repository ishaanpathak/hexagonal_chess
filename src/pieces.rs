use crate::board::Board;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PieceType {
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King,
    None,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PieceColor {
    White,
    Black,
    None,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ChessPiece {
    pub piece_type: PieceType,
    pub color: PieceColor,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Piece {
    pub x: usize,
    pub y: usize,
    pub piece: ChessPiece
}

pub fn get_all_pieces(board: &Board, check_color: PieceColor) -> Vec<Piece> {
    let mut all_pieces: Vec<Piece> = Vec::new();
    for i in 0usize..11usize {
        for j in 0usize..11usize {
            if (i + j > 4) && (i + j < 16) {
                if let Some(current_piece) = board.0[i][j] {
                    if current_piece.color == check_color {
                        all_pieces.push(Piece { x: i, y: j, piece: current_piece });
                    }
                }
            }
        }
    }
    all_pieces
}