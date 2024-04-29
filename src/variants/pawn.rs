pub struct Pawn {
    potential_moves: [ ( usize, usize ); 1 ],
}

impl Pawn {
    pub fn build_default( )-> Pawn {
        Pawn {
            potential_moves: [ ( 0, 1 ) ],
        }
    }
}