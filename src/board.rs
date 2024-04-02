struct Board {
    grid: [ [ option<(PieceColor, PieceType)>; 8]; 8],
    player_turn: Bool,
    move_history: Vec<String>,
}