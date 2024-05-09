use chess::board::Board;

fn main( ) {
    let mut board = Board::build_initialized( );

    Board::move_checked( &mut board, ( 4, 2 ), ( 4, 4 ) ); //THIS BREAK. MAKE SO NOT BREAK WHEN INDEX OUT OF BOUNDS!

    println!( "Initial location after movement: {:#?}", Board::get_piece( &board, 4, 2 ) );
    println!( "New location after movement: {:#?}", Board::get_piece( &board, 4, 4 ) );

    Board::move_checked( &mut board, ( 4, 4 ), ( 4, 6 ) );

    println!( "Initial location after movement: {:#?}", Board::get_piece( &board, 4, 4 ) );
    println!( "New location after movement: {:#?}", Board::get_piece( &board, 4, 6 ) );
}