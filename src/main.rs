fn main( ) {
    println!( "Hello world!" );
}

#[ cfg( test ) ]
mod tests {
    use chess::board::Board;
    use chess::variants::{ PieceColor, PieceType };

    #[ test ]
    fn pawn_board( ) {
        Board::build( [ [ Some(( PieceColor::White, PieceType::Pawn )); 8 ]; 8 ] );
    }
}