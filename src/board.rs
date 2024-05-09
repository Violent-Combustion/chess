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
    pub fn get_piece( board: &Self, x: usize, y: usize )-> Option<PieceType> {
        board.grid[y-1][x-1]
    }
    pub fn set_piece( board: &mut Self, ( x, y ): ( usize, usize ), piece: Option<PieceType> ) {
        board.grid[y-1][x-1] = piece;
    }
    pub fn move_piece( board: &mut Self, ( x1, y1 ): ( usize, usize ), ( x2, y2 ): ( usize, usize ) ) {
        Board::set_piece( board, ( x2, y2 ), Board::get_piece( board, x1, y1 ));
        Board::set_piece( board, ( x1, y1 ), None );
    }
    pub fn move_checked( board: &mut Self, ( x1, y1 ): ( usize, usize ), ( x2, y2 ): ( usize, usize ) ) { //KEEP WORKING ON THIS FUNCTION
        let valid = match Board::get_piece( board, x1, y1 ) {
            None => false,
            Some( piece_label ) => match piece_label {
                PieceType::Pawn( pawn ) => Piece::verify_move( &pawn, x1, y1, x2, y2 ),
                _ => false,
            }
        };
        if valid {
            Board::set_piece( board, ( x2, y2 ), Board::get_piece( board, x1, y1 ) );
            Board::set_piece( board, ( x1, y1 ), None );
        }
    }
}