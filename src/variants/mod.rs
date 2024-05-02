use crate::variants::{ pawn::Pawn, rook::Rook };

#[derive( Debug, PartialEq )]
pub enum PieceColor {
    Black,
    White,
}

#[derive( Debug, PartialEq )]
pub enum PieceType {
    Pawn(Pawn),
    Rook(Rook),
}

trait Piece {
    fn build( color: PieceColor, has_moved: bool )-> Self;
    fn build_initialized( color: PieceColor )-> Self;
}

pub mod pawn;
pub mod rook;