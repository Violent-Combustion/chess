// Contains the board, and manages the logic.

struct Board {
    grid: [ [ option; 8 ]; 8 ],
    // turn: ColorEnum,
    game_over: bool,
}