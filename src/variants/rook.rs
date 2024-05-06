use crate::variants::{ PieceColor, Piece };

#[derive( Debug, PartialEq, Copy, Clone )]
pub struct Rook {
    color: PieceColor,
    has_moved: bool,
}

impl Piece for Rook {
    fn build( color: PieceColor, has_moved: bool )-> Self {
        Rook {
            color: color,
            has_moved: has_moved,
        }
    }
    fn build_initialized( color: PieceColor )-> Self {
        Rook {
            color: color,
            has_moved: false,
        }
    }
}