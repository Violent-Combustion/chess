use crate::variants::PieceColor;

struct Board {
    grid: [ [ Option<( PieceColor, PieceType )>; 8]; 8],
    player_turn: PieceColor,
    move_history: Vec<String>,
}

impl Board {
    pub fn build( board_config: [ [ Option<( PieceColor, PieceColor )> ] ] )-> Board {
        Board {
            grid: board_config,
            player_turn: PieceColor::White,
            move_history: vec![],
        }
    }
}