use crate::pieces::PieceColor;
use crate::pieces::ChessPiece;
use crate::board::Board;
use crate::moves::Move;

type History = Vec<Move>;

/// Struct that contains what the Game's current Result is. Can be Ongoing, Checkmate, Stalemate, Draw.
#[derive(PartialEq, Debug)]
pub enum GameResult {
    Ongoing,
    Checkmate,
    Stalemate,
    Draw
}

/// This struct is responsible for storing the Game's state.
/// 
/// # Fields and their Meanings
/// 
/// 1. `id`: This ID will be useful for storing, identifying and loading games.
/// 2. `move_history`: Vector of Moves. This will store all the moves that have been played in the game.
/// 3. `board`: This is the actual board itself. Contains all the places, and pieces.
/// 4. `next_turn`: This will store which color will play next.
/// 5. `captured_pieces`: Array of 2 Vectors, 1 for Black and 1 for White. Each vector will store what Pieces have Black and White sides captured.
/// 6. `result`: Contains state of the Game: Ongoing, Checkmate, Stalemate, Draw
pub struct Game {
    pub id: u32,
    pub move_history: History,
    pub board: Board,
    pub next_turn: PieceColor,
    pub captured_pieces: [Vec<ChessPiece>; 2],
    pub result: GameResult
}