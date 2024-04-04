#[derive(Copy, Clone)]
pub enum PieceType {
    Pawn,
    Rook,
    Bishop,
    Queen,
    King,
    Knight,
}

#[derive(Copy, Clone)]
pub enum PieceColor {
    Black,
    White,
}

pub mod pawn;