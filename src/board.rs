use crate::variants::{ PieceColor, PieceType };

#[derive(Debug, Copy, Clone)]
pub struct Board {
    grid: [ [ Option<( PieceColor, PieceType )>; 8]; 8],
    player_turn: PieceColor,
    //move_history: Vec<String>,
}

impl Board {
    pub fn build( board_config: [ [ Option<( PieceColor, PieceType )>; 8 ]; 8 ] )-> Board {
        Board {
            grid: board_config,
            player_turn: PieceColor::White,
            //move_history: vec![],
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
                Some( ( PieceColor::Black, PieceType::Rook ) ) ] ],
            player_turn: PieceColor::White,
            //move_history: vec![],
        }
    }

    pub fn get_piece( &self, ( x, y ): ( usize, usize ) )-> Option<( PieceColor, PieceType )> {
        let piece: Option<( PieceColor, PieceType )> = self.grid[y-1][x-1];
        piece
    }

    pub fn set_piece( mut self, ( x, y ): ( usize, usize ), piece: Option<( PieceColor, PieceType )> )-> Board {
        self.grid[y-1][x-1] = piece;
        self
    }

    pub fn move_piece( mut self, ( x1, y1 ): ( usize, usize ), ( x2, y2 ): ( usize, usize ) )-> Board {
        self = self.set_piece( ( x2, y2 ), self.get_piece( ( x1, y1 ) ) );
        self = self.set_piece( ( x1, y1 ), None );
        self
    }

    pub fn move_piece_checked( mut self, ( x1, y1 ): ( usize, usize ), ( x2, y2 ): ( usize, usize ) )-> Board {
        match self.get_piece( ( x1, y1 ) ) {
            None => false,
            Some( ( color, ptype) ) => ( color, ptype ),
        }
        self = self.set_piece( ( x2, y2 ), self.get_piece( ( x1, y1 ) ) );
        self = self.set_piece( ( x1, y1 ), None );
        self
    }
}