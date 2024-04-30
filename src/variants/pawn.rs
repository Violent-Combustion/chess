use crate::variants::{ PieceColor, PieceType, Piece };

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Pawn {
    potential_moves: [ ( usize, usize ); 1 ],
}

impl Pawn {
    pub fn build_default( )-> Pawn {
        Pawn {
            potential_moves: [ ( 0, 1 ) ],
        }
    }
}

impl Piece for Pawn {
    fn verify_move( x1: usize, y1: usize, x2: usize, y2: usize )-> bool {
        //This confusing mess checks to see that the y coordinate is unchanged, and that the x2 coord is only one or two greater than x1
        if { if y1 == y2 { //y coordinate is unchanged
            true
        } else { //y coordinate is changed (invalid)
            false
        } } && { if { x2 - x1 } == 1 || { x2 - x1 } == 2 { //"delta x" is eq to 1 or 2
            true
        } else { //"delta x" is not eq to 1 or 2 (invalid)
            false
        } } { true } else { false } //if either of these evaluate to false, evaluates to false, else true
    }
}