use crate::message::Message;



pub const WHITE: i32 = 0xffffff;
pub const GREEN: i32 = 0x00ff00;

pub trait Piece {
    fn build( )-> Self;
    fn validate_move( &self, message: Message )-> bool;
}

pub enum ColorEnum {
    White,
    Green,
}