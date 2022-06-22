extern crate colored;

mod battleship;

use battleship::*;

fn main() {
    println!("\n         !! rs_battleship !!");

    let mut board = Board::new();
    board.arrange_boats_with_size(vec![5, 4, 3, 3, 2, 2]);
    board.shoot(Coordinates::new(3, 4));
    board.shoot(Coordinates::new(7, 7));
    board.print_grid();
}
