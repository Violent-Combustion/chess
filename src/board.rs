// Contains the board. And basically everthing that happens to the chess pieces happens here.

struct Board {
    grid: [ [ option; 8 ]; 8 ],
    turn: ColorEnum,
    draw: bool,
    history: vec<Str>,
}

mod coordinate_system {
    fn TODO( ) { println!("TODO"); }
}