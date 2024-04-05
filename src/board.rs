use crate::variants::{ PieceColor, PieceType };

pub struct Board {
    grid: [ [ Option<( PieceColor, PieceType )>; 8]; 8],
    player_turn: PieceColor,
    move_history: Vec<String>,
}

impl Board {
    pub fn build( board_config: [ [ Option<( PieceColor, PieceType )>; 8 ]; 8 ] )-> Board {
        Board {
            grid: board_config,
            player_turn: PieceColor::White,
            move_history: vec![],
        }
    }
    pub fn build_default()-> Board {
        Board {
            grid: 
                [ [ Some( ( PieceColor::White, PieceType::Rook ) ),
                Some( ( PieceColor::White, PieceType::Bishop) ),
                Some( ( PieceColor::White, PieceType::Knight) ),
                Some( ( PieceColor::White, PieceType::Queen ) ),
                Some( ( PieceColor::White, PieceType::King ) ),
                Some( ( PieceColor::White, PieceType::Bishop) ),
                Some( ( PieceColor::White, PieceType::Knight) ),
                Some( ( PieceColor::White, PieceType::Rook ) ) ],
      
                [ Some( ( PieceColor::White, PieceType::Pawn ) ); 8 ],

                [ None; 8 ],
                [ None; 8 ],
                [ None; 8 ],
                [ None; 8 ],

                [ Some( ( PieceColor::Black, PieceType::Pawn ) ); 8 ],
            
                [ Some( ( PieceColor::Black, PieceType::Rook ) ),
                Some( ( PieceColor::Black, PieceType::Knight ) ),
                Some( ( PieceColor::Black, PieceType::Bishop ) ),
                Some( ( PieceColor::Black, PieceType::Queen ) ),
                Some( ( PieceColor::Black, PieceType::King ) ),
                Some( ( PieceColor::Black, PieceType::Bishop ) ),
                Some( ( PieceColor::Black, PieceType::Knight ) ),
                Some( ( PieceColor::Black, PieceType::Rook ) ) ], ],
            player_turn: PieceColor::White,
            move_history: vec![],
        }
    }
}