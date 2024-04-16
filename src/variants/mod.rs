#[derive(Debug, Copy, Clone)]
pub enum PieceType {
    Pawn,
    Rook,
    Bishop,
    Queen,
    King,
    Knight,
}

#[derive(Debug, Copy, Clone)]
pub enum PieceColor {
    Black,
    White,
}

pub mod pawn;