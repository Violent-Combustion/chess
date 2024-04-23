use chess::board::Board;

fn main( ) {
    Board::build_default( ).move_piece( ( 1, 2 ), ( 1, 4 ) );
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
        let moved_piece = Board::build_default( ).move_piece( ( 1, 2 ), ( 1, 4 ) ).get_piece( ( 1, 4 ) );
        assert_eq!( moved_piece, Some( ( White, Pawn ) ) );
    }
}