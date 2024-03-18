// All pieces that implement piece will take an instance of this structure as an argument to the
//     validate_move( ) trait.

pub struct Message {
    moving: bool,
    en_passant_allowed: Option<bool>,
    castle_allowed: Option<bool>,
    current_coordinates: ( i32, i32 ),
    indicated_coordinates: ( i32, i32 ),
}

impl Message {
    pub fn build( is_moving: bool, 
                  en_passant_allowed: Option<bool>,
                  castle_allowed: Option<bool>,
                  current_coordinates: ( i32, i32 ),
                  indicated_coordinates: (i32, i32 ) )-> Self {

        Message {
            moving: is_moving,
            en_passant_allowed: en_passant_allowed,
            castle_allowed: castle_allowed,
            current_coordinates: current_coordinates,
            indicated_coordinates: indicated_coordinates,
        }
    }
}