use crate::variants::{ PieceColor, PieceType };

#[ derive( Debug, PartialEq ) ]
pub struct Board {
    grid: [ [ Option<PieceType>; 8 ]; 8 ],
    turn: PieceColor,
}

impl Board {
    pub fn build( grid: [ [ Option<PieceType>; 8 ]; 8 ], color: PieceColor )-> Board {
        Board {
            grid: grid,
            turn: color,
        }
    }
}