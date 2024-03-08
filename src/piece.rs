trait Piece {
    fn build( )-> self;
    fn kill( &self );
    fn validate_move( &self, ( i32, i32 ) )-> bool;
}