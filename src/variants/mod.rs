use crate::variants::{ pawn::Pawn };

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum PieceType {
    Pawn(Pawn),
    Rook,
    Bishop,
    Queen,
    King,
    Knight,
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum PieceColor {
    Black,
    White,
}

trait Piece {
    fn verify_move( x1: usize, y1: usize, x2: usize, y2: usize )-> bool;
}

pub mod pawn;