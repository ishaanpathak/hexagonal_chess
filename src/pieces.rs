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