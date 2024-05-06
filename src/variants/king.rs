use crate::variants::{ PieceColor, Piece };

#[derive( Debug, PartialEq, Copy, Clone )]
pub struct King {
    color: PieceColor,
    has_moved: bool,
}

impl Piece for King {
    fn build( color: PieceColor, has_moved: bool )-> Self {
        King {
            color: color,
            has_moved: has_moved,
        }
    }
    fn build_initialized( color: PieceColor )-> Self {
        King {
            color: color,
            has_moved: false,
        }
    }
}