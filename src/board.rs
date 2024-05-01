#[ derive( Debug ) ]
pub struct Board {
    grid: [ Option<isize>; 8 ],
}

impl Board {
    pub fn build( grid: [ Option<isize>; 8 ] )-> Board {
        Board { grid: grid }
    }
    pub fn change_grid( board: &mut Self, grid: [ Option<isize>; 8 ] )-> &Board {
        board.grid = grid;
        board
    }
    pub fn get_first( board: &Self ) {
        println!( "{:?}", board.grid[0] );
    }
}