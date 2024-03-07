mod piece;
use piece::GamePiece;

fn main( ) {
    let x = GamePiece::build( piece::variants::PieceEnum::Pawn, 1 );
}
