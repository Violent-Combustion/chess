use crate::variants::{ PieceColor, Piece };
#[derive( Debug, PartialEq, Copy, Clone )]
pub struct Knight {
    color: PieceColor,
    has_moved: bool,
}

impl Piece for Knight {
    fn build( color: PieceColor, has_moved: bool )-> Self {
        Knight {
            color: color,
            has_moved: has_moved,
        }
    }
    fn build_initialized( color: PieceColor )-> Self {
        Knight {
            color: color,
            has_moved: false,
        }
    }
}