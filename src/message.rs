pub struct Message {
    moving: bool,
    en_passant_allowed: option<bool>,
    castle_allowed: option<bool>,
    current_coordinates: ( i32, i32 ),
    indicated_coordinates: ( i32, i32 ),
}

impl Message {
    pub fn build( is_moving: bool, 
                  en_passant_allowed: option<bool>,
                  castle_allowed: option<bool>,
                  current_coordinates: ( i32, i32 ),
                  indicated_coordinates: (i32, i32 ) )-> self {
        Message {
            moving: is_moving,
            en_passant_allowed: en_passant_allowed,
            castle_allowed: castle_allowed,
            current_coordinates: current_coordinates,
            indicated_coordinates: indicated_coordinates,
        }
    }
}