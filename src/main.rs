mod battleship;

use battleship::*;

fn main() {
    println!("\n         !! rs_battleship !!");

    let mut board = Board {
        boats: vec![]
    };

    board.arrange_boats_with_size(vec![5, 4, 3, 3, 2, 2]);
    board.print_grid();
}
