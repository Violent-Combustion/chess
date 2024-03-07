pub struct GamePiece {
    variant: variants::VariantEnum,
    color: color::ColorEnum,
    //     ^^^    This value should be hexadecimal
}

pub mod color {
    pub enum ColorEnum {
        White,
        Black,
    }
}

pub mod variants {
    pub enum VariantEnum {
        Pawn,
        Rook,
        Knight,
        Bishop,
        Queen,
        King,
    }
}

impl GamePiece {
    pub fn build( variant: variants::VariantEnum, color: color::ColorEnum )-> GamePiece {
        GamePiece {
            variant: variant,
            color: color,
        }
    }
}
