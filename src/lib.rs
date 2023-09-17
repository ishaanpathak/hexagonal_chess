mod pieces;
mod board;
mod moves;
mod game;
mod validation;

use crate::moves::Move;
use crate::pieces::PieceColor;
use crate::pieces::ChessPiece;
use crate::board::Board;
use crate::game::History;
use crate::game::GameResult;
use crate::pieces::PieceType;
use crate::moves::MoveList;

use wasm_bindgen::prelude::*;
use std::collections::HashMap;
use serde_json;

#[wasm_bindgen]
pub struct Game {
    id: u32,
    move_history: History,
    board: Board,
    current_turn: PieceColor,
    captured_pieces: [Vec<ChessPiece>; 2],
    result: GameResult
}

#[wasm_bindgen]
impl Game {
    pub fn new() -> Game {
        Game {
            id: 0,
            move_history: Vec::new(),
            board: board::get_default_board(),
            current_turn: PieceColor::White,
            captured_pieces: [Vec::new(), Vec::new()],
            result: GameResult::Ongoing
        }
    }

    fn convert_piece_id_to_icon(id: String) -> String {
        match id.as_str() {
            "WP" => String::from("♙"),
            "WR" => String::from("♖"),
            "WN" => String::from("♘"),
            "WB" => String::from("♗"),
            "WQ" => String::from("♕"),
            "WK" => String::from("♔"),
            "BP" => String::from("♟︎"),
            "BR" => String::from("♜"),
            "BN" => String::from("♞"),
            "BB" => String::from("♝"),
            "BQ" => String::from("♛"),
            "BK" => String::from("♚"),
            _ => String::from("")
        }
    }

    pub fn get_board(&self) -> JsValue {
        let mut board: HashMap<String, String> = HashMap::new();
        for i in 0usize..11usize {
            for j in 0usize..11usize {
                if let Some(piece) = self.board.0[i][j] {
                    if (piece.color != PieceColor::None) && (piece.piece_type != PieceType::None) {
                        let piece_type = match piece.piece_type {
                            pieces::PieceType::Pawn => "P",
                            pieces::PieceType::Rook => "R",
                            pieces::PieceType::Knight => "N",
                            pieces::PieceType::Bishop => "B",
                            pieces::PieceType::Queen => "Q",
                            pieces::PieceType::King => "K",
                            pieces::PieceType::None => ""
                        };
                        let piece_color = match piece.color {
                            pieces::PieceColor::White => "W",
                            pieces::PieceColor::Black => "B",
                            pieces::PieceColor::None => ""
                        };
                        let mut piece_string = format!("{}{}", piece_color, piece_type);
                        piece_string = Self::convert_piece_id_to_icon(piece_string);
                        let coordinate_string = format!("{},{}", i, j);
                        board.insert(coordinate_string, piece_string);
                    }
                }
            }
        }
        JsValue::from_str(serde_json::to_string(&board).unwrap().as_str())
    }

    pub fn get_next_turn(&self) -> String {
        match self.current_turn {
            PieceColor::White => String::from("W"),
            PieceColor::Black => String::from("B"),
            PieceColor::None => String::from("")
        }
    }

    pub fn get_current_player_moves(&self) -> JsValue {
        let mut moves: HashMap<String, Vec<String>> = HashMap::new();
        let all_pieces = pieces::get_all_pieces(&self.board, self.current_turn);
        let mut move_list: MoveList = MoveList::new();
        for piece in all_pieces {
            let piece_moves = moves::get_legal_moves(&self.board, &(piece.x, piece.y));
            move_list.insert((piece.x, piece.y), piece_moves);
        }
        moves::remove_check_moves(&self.board, &mut move_list);
        for (coordinate, moves_internal) in move_list {
            let coordinate_string = format!("{},{}", coordinate.0, coordinate.1);
            let mut move_strings: Vec<String> = Vec::new();
            for m in moves_internal {
                let move_string = format!("{},{}", m.0, m.1);
                move_strings.push(move_string);
            }
            moves.insert(coordinate_string, move_strings);
        }
        JsValue::from_str(serde_json::to_string(&moves).unwrap().as_str())
    }

    pub fn make_move(&mut self, from: String, to: String) {
        let from_split: Vec<&str> = from.split(",").collect();
        let from_x: usize = from_split[0].parse().unwrap();
        let from_y: usize = from_split[1].parse().unwrap();
        
        let to_split: Vec<&str> = to.split(",").collect();
        let to_x: usize = to_split[0].parse().unwrap();
        let to_y: usize = to_split[1].parse().unwrap();

        let piece = self.board.clone().0[from_x][from_y].unwrap();

        moves::execute_move(&mut self.board, &Move {
            piece,
            from: (from_x, from_y),
            to: (to_x, to_y)
        });
    }

    pub fn switch_player(&mut self) {
        self.current_turn = match self.current_turn {
            PieceColor::White => PieceColor::Black,
            PieceColor::Black => PieceColor::White,
            PieceColor::None => PieceColor::None
        };
    }
}

