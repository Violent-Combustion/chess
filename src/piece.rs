use crate::variants::pawn::message;

pub const WHITE: i32 = 0xffffff;
pub const GREEN: i32 = 0x00ff00;

pub enum ColorEnum {
    White,
    Green,
}

pub trait Piece {
    fn build( color: ColorEnum )-> Self;
    fn validate_move( &self, message: message::Message )-> bool;
}