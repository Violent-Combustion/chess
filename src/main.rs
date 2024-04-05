use chess::variants::{PieceColor, PieceType};
use chess::board::Board;

fn main( ) {
    println!( "Hello world!" );
}

#[ cfg( test ) ]
mod tests {
    use chess::board::Board;
    
    #[ test ]
    fn setup_board( ) {
        Board::build_default( );
    }
}