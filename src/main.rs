use chess::board::Board;

fn main( ) {
    let mut board = Board::build_initialized( );

    Board::move_piece( &mut board, ( 4, 2 ), ( 4, 4 ) ); //THIS BREAK. MAKE SO NOT BREAK WHEN 9!

    println!( "Initial location after movement: {:#?}", Board::get_piece( &board, 4, 2 ) );
    println!( "New location after movement: {:#?}", Board::get_piece( &board, 4, 4 ) );
}