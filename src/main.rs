mod pieces;
mod moves_generation;
mod board;

use crate::moves_generation::*;
use crate::board::*;

fn main() {

    let board = get_board_from_file("/run/media/ishaan/Windows/Users/ishaan/Documents/HexaChess/Python Board Generator/output/pawn_test_new.txt");

    print_board(&board);

    println!("{:?}", get_legal_moves(&board, &(6,5)));
    println!("{:?}", get_legal_moves(&board, &(6,8)));
    println!("{:?}", get_legal_moves(&board, &(4,5)));
    println!("{:?}", get_legal_moves(&board, &(4,3)));
    println!("{:?}", get_legal_moves(&board, &(3,7)));
}
