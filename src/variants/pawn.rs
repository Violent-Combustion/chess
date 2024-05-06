use crate::variants::{ PieceColor, Piece };

#[derive( Debug, PartialEq, Copy, Clone )]
pub struct Pawn {
    color: PieceColor,
    has_moved: bool,
}

impl Piece for Pawn {
    fn build( color: PieceColor, has_moved: bool )-> Self {
        Pawn {
            color: color,
            has_moved: has_moved,
        }
    }
    fn build_initialized( color: PieceColor )-> Self {
        Pawn {
            color: color,
            has_moved: false,
        }
    }
}