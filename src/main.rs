use chess::board::Board;

fn main( ) {
    let mut game = Board::build( [ Some( 1 ); 8 ] );
    Board::get_first(&game);
    Board::change_grid(&mut game, [ Some( 2 ); 8 ]);
    Board::get_first(&game);
}