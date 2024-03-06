pub struct GamePiece {
    coordinates: ( i32, i32 ),
    color: i32,
    //     ^^^    This value should be hexadecimal
    dead: bool,
}

impl GamePiece {
    pub fn build( ( x, y ): ( i32, i32 ), color: i32 )-> GamePiece {
        GamePiece {
            coordinates: ( x, y ),
            color: color,
            dead: false,
        }
    }
}