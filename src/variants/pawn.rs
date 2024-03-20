#[ path="../piece.rs" ]
mod piece;
use piece::{ ColorEnum, Piece };

#[ path="../message.rs"]
mod message;
use message::Message;



pub struct Pawn {
    display: char,
    color: ColorEnum,
    movement: [ ( i32, i32 ); 4 ],
    has_moved: bool,
}

impl Piece for Pawn {
    fn build( color: ColorEnum )-> Self {

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
            has_moved: false,
        }
    }

    fn validate_move( &self, x: Message )-> bool { true }
}