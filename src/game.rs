use crate::pieces::PieceColor;
use crate::pieces::ChessPiece;
use crate::board::Board;

enum GameResult {
    Ongoing,
    Checkmate,
    Stalemate,
    Draw
}

/// This struct is responsible for storing the Game's state.
/// 
/// # Feilds and their Meanings
/// 
/// 1. `id`: This ID will be useful for storing, identifying and loading gamess.
/// 2. `move_history`: This is a Vector of (x,y) coordinates which will basically tell us which piece was move to which coordinate.
/// 3. `board`: This is the actual board itself. Contains all the places, and pieces.
/// 4. `next_turn`: This will store which color will play next.
/// 5. `captured_pieces`: Array of 2 Vectors, 1 for Black and 1 for White. Each vector will store what Pieces have Black and White sides captured.
pub struct Game {
    pub id: u32,
    pub move_history: Vec<[(usize, usize); 2]>,
    pub board: Board,
    pub next_turn: PieceColor,
    pub captured_pieces: [Vec<ChessPiece>; 2]
}