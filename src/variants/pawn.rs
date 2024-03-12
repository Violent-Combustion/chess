pub struct Pawn {
    display: char,
    color: ColorEnum,
    movement: [ ( i32, i32 ); 4 ],
}

impl Piece for Pawn {
    fn build( color: ColorEnum )-> self {

        //checks to see what color the piece is, 
        //    so to allow the system to build a pawn that moves the right way.
        let movement = match color {
            White => [ ( 0, 1 ),
                       ( 0, 2 ),
                       ( -1, 1 ),
                       ( 1, 1 ),
            ],
            Green => [ ( 0, -1 ),
                       ( 0, -2 ),
                       ( -1, -1 ),
                       ( 1, -1 ),
            ],
        };

        Pawn {
            display: 'P',
            color: color,
            movement: movement,
        }
    }

    fn validate_move( &self, ( x, y ): ( i32, i32 ) )-> bool;
}