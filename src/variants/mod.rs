#[derive(Debug, Copy, Clone, PartialEq)]
pub enum PieceType {
    Pawn,
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
    fn verify_move( ( x, y ): ( usize, usize ) )-> bool; //WHY ARE PATTERNS NOT PERMITTED HERE?!?!
}

pub mod pawn;