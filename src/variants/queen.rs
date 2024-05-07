use crate::variants::{ PieceColor, Piece };

#[derive( Debug, PartialEq, Copy, Clone )]
pub struct Queen {
    color: PieceColor,
    has_moved: bool,
}

impl Piece for Queen {
    fn build( color: PieceColor, has_moved: bool )-> Self {
        Queen {
            color: color,
            has_moved: has_moved,
        }
    }
    fn build_initialized( color: PieceColor )-> Self {
        Queen {
            color: color,
            has_moved: false,
        }
    }
}