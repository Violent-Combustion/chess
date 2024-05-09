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
    fn verify_move( piece: &Self, x1: usize, y1: usize, x2: usize, y2: usize )-> bool {
        if x2 == x1 && y2 == y1+1 {
            true
        } else if piece.has_moved == false && x2 == x1 && y2 == y1+2 {
            true
        } else {
            false
        }
    }
}