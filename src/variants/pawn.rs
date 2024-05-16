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
    fn verify_move( piece: &Self, x1: usize, y1: usize, x2: usize, y2: usize )-> bool {
        if piece.color == PieceColor::White {
            if x2 == x1 && y2 == y1+1 {
                true
            } else if piece.has_moved == false && x2 == x1 && y2 == y1+2 { 
                true
            } else {
                false
            }
        } else {
            if x2 == x1 && y2 == y1-1 {
                true
            } else if piece.has_moved == false && x2 == x1 && y2 == y1-2 { 
                true
            } else {
                false
            }
        }
    }
    fn set_moved( piece: &mut Self ) {
        piece.has_moved = true;
    }
}