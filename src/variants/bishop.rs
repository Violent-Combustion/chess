use crate::variants::{ PieceColor, Piece };

#[derive( Debug, PartialEq, Copy, Clone )]
pub struct Bishop {
    color: PieceColor,
    has_moved: bool,
}

impl Piece for Bishop {
    fn build( color: PieceColor, has_moved: bool )-> Self {
        Bishop {
            color: color,
            has_moved: has_moved,
        }
    }
    fn build_initialized( color: PieceColor )-> Self {
        Bishop {
            color: color,
            has_moved: false,
        }
    }
}