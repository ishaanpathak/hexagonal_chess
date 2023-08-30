mod pieces;
mod moves;
mod board;
mod config;
mod game;

use crate::moves::*;
use crate::board::*;
use crate::config::load_config;
use crate::pieces::ChessPiece;
use crate::game::Game;

fn main() {
    let board: [[Option<ChessPiece>; 11]; 11];
    
    let config = load_config().unwrap();
    
    if config.board_from_file {
        board = get_board_from_file(&config.board_path);
    } else {
        board = get_default_board();
    }

    print_board(&board);

    println!("{:?}", get_legal_moves(&board, &(6,5)));
    println!("{:?}", get_legal_moves(&board, &(6,8)));
    println!("{:?}", get_legal_moves(&board, &(4,5)));
    println!("{:?}", get_legal_moves(&board, &(4,3)));
    println!("{:?}", get_legal_moves(&board, &(3,7)));
    
    let _current_game = Game {
        id: 1,
        move_history: Vec::new(),
        board: board.clone(),
        next_turn: pieces::PieceColor::White,
        captured_pieces: [Vec::new(), Vec::new()],
    };
}
