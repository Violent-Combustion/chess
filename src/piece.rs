pub struct GamePiece {
    variant: variants::PieceEnum,
    color: i32,
    //     ^^^    This value should be hexadecimal
}

pub mod variants {
    pub enum PieceEnum {
        Pawn,
        Rook,
        Knight,
        Bishop,
        Queen,
        King,
    }
}

impl GamePiece {
    pub fn build( variant: variants::PieceEnum, color: i32 )-> GamePiece {
        GamePiece {
            variant: variant,
            color: color,
        }
    }
}
