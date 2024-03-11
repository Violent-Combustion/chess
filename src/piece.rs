pub const WHITE: i32 = 0xffffff;
pub const GREEN: i32 = 0x00ff00;

pub trait Piece {
    fn build( )-> self;
    fn kill( &self );
    fn validate_move( &self, ( x, y ): ( i32, i32 ) )-> bool;
}

pub enum ColorEnum {
    White,
    Green,
}