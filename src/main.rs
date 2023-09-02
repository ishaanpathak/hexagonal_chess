mod pieces;
mod moves;
mod board;
mod config;
mod game;
mod validation;

use pieces::PieceColor;

use crate::moves::*;
use crate::board::*;
use crate::config::load_config;
use crate::game::Game;

fn main() {
    let board: Board;
    
    let config = load_config().unwrap();
    
    if config.board_from_file {
        board = get_board_from_file(&config.board_path);
    } else {
        board = get_default_board();
    }

    print_board(&board);

    let _current_game = Game {
        id: 1,
        move_history: Vec::new(),
        board: board.clone(),
        next_turn: pieces::PieceColor::White,
        captured_pieces: [Vec::new(), Vec::new()],
        result: game::GameResult::Ongoing
    };

    let chance = PieceColor::White;
    
    let current_pieces = pieces::get_all_pieces(&board, chance);

    let mut all_moves: MoveList = MoveList::new();
    
    for piece in current_pieces {
        let current_piece_moves = get_legal_moves(&board, &(piece.x, piece.y));
        all_moves.insert((piece.x, piece.y), current_piece_moves);
    }
    
    let in_check = validation::is_check(&board, chance);
    
    remove_check_moves(&board, &mut all_moves);
    
    if in_check {
        if validation::has_legal_moves(all_moves) {
            println!("Check!");
        } else {
            println!("Checkmate!");
        }
    } else {
        if !validation::has_legal_moves(all_moves) {
            println!("Stalemate!");
        } else {
            println!("Nothing");
        }
    }
}
