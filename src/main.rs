use chess::board::Board;

fn main( ) {
    let mut board = Board::build_initialized( );

    Board::move_checked( &mut board, ( 4, 7 ), ( 4, 5 ) ); //THIS BREAK. MAKE SO NOT BREAK WHEN INDEX OUT OF BOUNDS!

    println!( "Initial location after movement: {:#?}", Board::get_piece( &board, 4, 7 ) );
    println!( "New location after movement: {:#?}", Board::get_piece( &board, 4, 5 ) );

    Board::move_checked( &mut board, ( 4, 1 ), ( 1, 6 ) ); //THE QUEEN DOES NOT MOVE

    println!( "Initial location after movement: {:#?}", Board::get_piece( &board, 4, 1 ) );
    println!( "New location after movement: {:#?}", Board::get_piece( &board, 2, 3 ) );
}