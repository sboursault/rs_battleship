mod battleship;

use battleship::*;

fn main() {

    println!("\n    !! rs_battleship !!");

    let mut board = Board {
        boats: vec![
            Boat::new(vec![Coordinates::new(5, 2), Coordinates::new(6, 2)]),
            Boat::new(vec![Coordinates::new(7, 3), Coordinates::new(7, 4), Coordinates::new(7, 5), Coordinates::new(7, 6)])
        ]
    };

    board.print_grid();


}
