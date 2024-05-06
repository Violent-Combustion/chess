use crate::variants::{
    PieceColor,
    PieceType,
    Piece,
    rook::Rook,
    knight::Knight,
    bishop::Bishop,
    king::King,
    queen::Queen,
    pawn::Pawn,
};

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
    pub fn build_initialized( )-> Board {
        Board {
            grid: [
                [ Some( PieceType::Rook( Rook::build_initialized( PieceColor::White ) ) ),
                  Some( PieceType::Knight( Knight::build_initialized( PieceColor::White ) ) ),
                  Some( PieceType::Bishop( Bishop::build_initialized( PieceColor::White ) ) ),
                  Some( PieceType::Queen( Queen::build_initialized( PieceColor::White ) ) ),
                  Some( PieceType::King( King::build_initialized( PieceColor::White ) ) ),
                  Some( PieceType::Bishop( Bishop::build_initialized( PieceColor::White ) ) ),
                  Some( PieceType::Knight( Knight::build_initialized( PieceColor::White ) ) ),
                  Some( PieceType::Rook( Rook::build_initialized( PieceColor::White ) ) ), ],
                
                [ Some( PieceType::Pawn( Pawn::build_initialized( PieceColor::White ) ) ); 8],

                [ None; 8 ],
                [ None; 8 ],
                [ None; 8 ],
                [ None; 8 ],

                [ Some( PieceType::Pawn( Pawn::build_initialized( PieceColor::Black ) ) ); 8],

                [ Some( PieceType::Rook( Rook::build_initialized( PieceColor::Black ) ) ),
                  Some( PieceType::Knight( Knight::build_initialized( PieceColor::Black ) ) ),
                  Some( PieceType::Bishop( Bishop::build_initialized( PieceColor::Black ) ) ),
                  Some( PieceType::Queen( Queen::build_initialized( PieceColor::Black ) ) ),
                  Some( PieceType::King( King::build_initialized( PieceColor::Black ) ) ),
                  Some( PieceType::Bishop( Bishop::build_initialized( PieceColor::Black ) ) ),
                  Some( PieceType::Knight( Knight::build_initialized( PieceColor::Black ) ) ),
                  Some( PieceType::Rook( Rook::build_initialized( PieceColor::Black ) ) ), ], ],
                turn: PieceColor::White,
        }
    }
}