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
    fn verify_move( _piece: &Self, x1: isize, y1: isize, x2: isize, y2: isize )-> bool {
        if x2 == x1 && y2 != y1 {
            true
        } else if x2 != x1 && y2 == y1 {
            true
        } else if x2 - x1 == y2 - y1 { 
            true
        } else if x2 - x1 == -( y2 - y1 ) {
            true
        } else if -( x2 - x1 ) == y2 - y1 { 
            true
        } else if -( x2 - x1 ) == -( y2 - y1 ) {
            true
        } else {
            false
        }
    }
    fn set_moved( piece: &mut Self ) {
        piece.has_moved = true;
    }
}