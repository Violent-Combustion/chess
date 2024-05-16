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
    fn verify_move( piece: &Self, x1: usize, y1: usize, x2: usize, y2: usize )-> bool {
        let x1 = x1 as isize;
        let x2 = x2 as isize;
        let y1 = y1 as isize;
        let y2 = y2 as isize;

        if x2 == x1 && y2 != y1 {
            true
        } else if x2 != x1 && y2 == y1 {
            true
        } else if x2 - x1 == y2 - y1 { //THE QUEEN CANNOT MOVE DIAGONALLY
            true
        } else if x2 - x1 == -( y2 - y1 ) { //THE QUEEN CANNOT MOVE DIAGONALLY
            true
        } else {
            false
        }
    }
    fn set_moved( piece: &mut Self ) {
        piece.has_moved = true;
    }
}