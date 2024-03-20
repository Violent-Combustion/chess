// Contains the board, which contains all the pieces.
// Communicates context to the pieces and allows them to figure out whiether they are allowed to
//     move or not, and if they can, executes the movement.

use crate::piece::ColorEnum;

struct Board<'a> {
    grid: [ [ Option< P >; 8 ]; 8 ],
    turn: ColorEnum,
    draw: bool,
    history: Vec< &'a str >,
}

mod coordinate_system {
    fn todo( ) { println!("TODO"); }
}