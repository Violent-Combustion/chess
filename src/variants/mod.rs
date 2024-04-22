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

pub mod pawn;