fn main( ) {
    println!( "Hello world!" );
}

#[ cfg( test ) ]
mod tests {
    #[ test ]
    fn pawn_board( ) {
        board::build( [ [ option( Pawn, White ); 8 ]; 8 ] )
    }
}