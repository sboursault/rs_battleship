mod battleship;

use battleship::*;

fn main() {

    println!("\n    !! rs_battleship !!");

    let mut board = Board {
        boats: vec![]
    };

    board.arrange_boat_with_size(3);
    board.print_grid();


}
