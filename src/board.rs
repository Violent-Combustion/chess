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
    pub fn move_piece( board: &mut Self, ( x1, y1 ): ( isize, isize ), ( x2, y2 ): ( isize, isize ) ) {
        Board::set_piece( board, ( x2 as usize, y2 as usize ), Board::get_piece( board, x1 as usize, y1 as usize ));
        Board::set_piece( board, ( x1 as usize, y1 as usize ), None );
    }
    pub fn flag_piece_as_moved( piece: &mut Option<PieceType> )-> &mut Option<PieceType> {
        *piece = match piece {
            Some( piece_label ) => match piece_label {
                PieceType::Pawn( mut pawn ) => {
                    Piece::set_moved( &mut pawn );
                    Some( PieceType::Pawn( pawn ) )
                },
                PieceType::Rook( mut rook ) => {
                    Piece::set_moved( &mut rook );
                    Some( PieceType::Rook( rook ) )
                },
                PieceType::Queen( mut queen ) => {
                    Piece::set_moved( &mut queen );
                    Some( PieceType::Queen( queen ) )
                },
                PieceType::King( mut king ) => {
                    Piece::set_moved( &mut king );
                    Some( PieceType::King( king ) )
                },
                PieceType::Bishop( mut bishop ) => {
                    Piece::set_moved( &mut bishop );
                    Some( PieceType::Bishop( bishop ) )
                },
                PieceType::Knight( mut knight ) => {
                    Piece::set_moved( &mut knight );
                    Some( PieceType::Knight( knight ) )
                },
            }
            None => None,
        };
        piece
    }
    pub fn move_checked( board: &mut Self, ( x1, y1 ): ( isize, isize ), ( x2, y2 ): ( isize, isize ) ) { //KEEP WORKING ON THIS FUNCTION
        let valid = match Board::get_piece( board, x1 as usize, y1 as usize ) {
            None => false,
            Some( piece_label ) => match piece_label {
                PieceType::Pawn( pawn ) => Piece::verify_move( &pawn, x1, y1, x2, y2 ),
                PieceType::Rook( rook ) => Piece::verify_move( &rook, x1, y1, x2, y2 ),
                PieceType::Queen( queen ) => Piece::verify_move( &queen, x1, y1, x2, y2 ),
                PieceType::King( king ) => Piece::verify_move( &king, x1, y1, x2, y2 ),
                PieceType::Bishop( bishop ) => Piece::verify_move( &bishop, x1, y1, x2, y2 ),
                PieceType::Knight( knight ) => Piece::verify_move( &knight, x1, y1, x2, y2 ),
            }
        };
        if valid {
            Board::set_piece( board, ( x2 as usize, y2 as usize ), *Self::flag_piece_as_moved( &mut Board::get_piece( board, x1 as usize, y1 as usize) ) );
            Board::set_piece( board, ( x1 as usize, y1 as usize ), None );
        }
    }
}