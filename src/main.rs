use chess::variants::{PieceColor, PieceType};

fn main( ) {
    println!( "Hello world!" );
}

#[ cfg( test ) ]
mod tests {
    use chess::{board::Board, variants::{PieceColor, PieceType} };
    
    #[ test ]
    fn setup_board( ) {
        Board::build_default( );
    }

    #[ test ]
    fn move_pawn( ) {
        let board = Board::build_default( );
        board.move_piece( ( 2, 5 ), ( 4, 5 ) );
        assert_eq!( board.get_piece( ( 2, 5 ) ), None );
        assert_eq!( board.get_piece( ( 4, 5 ) ), Some( ( White, Pawn ) );
    }
}