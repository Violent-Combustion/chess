pub struct Pawn {
    display: char,
    color: ColorEnum,
    movement: [ ( i32, i32 ); 4 ],
}

impl Piece for Pawn {
    fn build( color: ColorEnum )-> self {
        Pawn {
            display: 'P',
            color: color,
            movement: [ ( 0, 1 ),
                        ( 0, 2 ),
                        ( -1, 1 ),
                        ( 1, 1 ),
                      ],
        }
    }

    fn kill( &self );
    fn validate_move( &self, ( x, y ): ( i32, i32 ) )-> bool;
}