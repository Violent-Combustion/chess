use chess::variants::{PieceColor, PieceType};

fn main( ) {
    println!( "Hello world!" );
}

#[ cfg( test ) ]
mod tests {
    use chess::{board::Board, variants::{PieceColor::{self, *}, PieceType::{self, *}} };
    
    #[ test ]
    fn setup_board( ) {
        Board::build_default( );
    }

    #[ test ]
    fn move_pawn( ) {
        let board = Board::build_default( );
        board.move_piece( ( 2, 5 ), ( 4, 5 ) );
        assert!( board.get_piece( ( 2, 5 ) ).is_none() );

        match board.get_piece( ( 4, 5 ) ) {
            None => { panic!(); },
            Some( ( color, piece ) ) => {
                match color {
                    White => { },
                    Black => { panic!(); }
                };
                match piece {
                    Pawn => { },
                    _ => { panic!(); }
                }
            }
        };
    }
}