const GRID_SIZE: i8 = 10;

pub struct Board {
    pub boats: Vec<Boat>
}

impl Board {
    pub fn shoot(&mut self, coord: Coordinates) -> ShotResult {
        for boat in &mut self.boats {
            if boat.is_at(&coord) {
                return boat.hit(coord);
            }
        }

        ShotResult::Missed
    }

    pub fn arrange(boat_sizes: Vec<i8>) {
        let board = Board { boats: vec![] };
        for boat_size in boat_sizes {}
    }

    pub fn print_grid(&self) {
        print!("┌");
        Board::render_vline();
        print!("┐\n");
        for i in 1..GRID_SIZE {
            print!("|");
            for j in 1..GRID_SIZE {
                print!("{}", self.render_cell(Coordinates::new(i, j)));
            }
            print!("|\n")
        }
        print!("└");
        Board::render_vline();
        print!("┘\n");
    }

    fn render_vline() {

        for j in 1..GRID_SIZE {
            print!("---")
        }
    }

    fn render_cell(&self, coord: Coordinates) -> &str {
        for boat in &self.boats {
            if boat.is_at(&coord) {
                return " X ";
            }
        }
        return "   ";
    }
}


pub struct Boat {
    position: Vec<Coordinates>,
    hits: Vec<Coordinates>,
}

impl Boat {
    pub fn new(position: Vec<Coordinates>) -> Boat {
        return Boat { position, hits: vec![] };
    }

    pub fn is_at(&self, coord: &Coordinates) -> bool {
        self.position.contains(coord)
    }

    pub fn hit(&mut self, coord: Coordinates) -> ShotResult {
        self.hits.push(coord);
        for boat_coord in &self.position {
            if !self.hits.contains(boat_coord) {
                return ShotResult::Hit;
            }
        }
        return ShotResult::Destroyed;
    }
}

#[derive(PartialEq, Debug)]
pub struct Coordinates {
    x: i8,
    y: i8,
}

impl Coordinates {
    pub fn new(x: i8, y: i8) -> Coordinates {
        return Coordinates { x, y };
    }
}

#[derive(PartialEq, Debug)]
enum ShotResult {
    Missed,
    Hit,
    Destroyed,
}


#[cfg(test)]  // Only compiles when running tests
mod tests {
    use super::*;


    #[test]
    fn shoot() {
        let mut board = Board {
            boats: vec![
                Boat::new(vec![Coordinates::new(5, 2), Coordinates::new(6, 2)]),
                Boat::new(vec![Coordinates::new(7, 3), Coordinates::new(7, 4), Coordinates::new(7, 5)])
            ]
        };
        assert_eq!(board.shoot(Coordinates::new(5, 2)), ShotResult::Hit);
        assert_eq!(board.shoot(Coordinates::new(4, 2)), ShotResult::Missed);
        assert_eq!(board.shoot(Coordinates::new(7, 4)), ShotResult::Hit);
        assert_eq!(board.shoot(Coordinates::new(6, 2)), ShotResult::Destroyed);
    }
}
