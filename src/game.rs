struct Game {
    id: u32,
    move_history: Vec<[(usize, usize); 2]>,
    board: [[Option<ChessPiece>; 11]; 11]
}