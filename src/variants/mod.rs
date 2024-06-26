use crate::variants::{
    pawn::Pawn,
    rook::Rook,
    bishop::Bishop,
    king::King,
    queen::Queen,
    knight::Knight,
};

#[derive( Debug, PartialEq, Copy, Clone )]
pub enum PieceColor {
    Black,
    White,
}

#[derive( Debug, PartialEq, Copy, Clone )]
pub enum PieceType {
    Pawn( Pawn ),
    Rook( Rook ),
    Bishop( Bishop ),
    King( King ),
    Queen( Queen ),
    Knight( Knight ),
}

pub trait Piece {
    fn build( color: PieceColor, has_moved: bool )-> Self;
    fn build_initialized( color: PieceColor )-> Self;
    fn verify_move( piece: &Self, x1: isize, y1: isize, x2: isize, y2: isize )-> bool;
    fn set_moved( piece: &mut Self );
}

pub mod pawn;
pub mod rook;
pub mod bishop;
pub mod king;
pub mod queen;
pub mod knight;