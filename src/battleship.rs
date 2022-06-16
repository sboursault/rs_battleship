use rand::Rng;

const GRID_SIZE: i8 = 10;


pub struct Board {
    pub boats: Vec<Boat>
}

impl Board {
    pub fn shoot(&mut self, coord: Coordinates) -> ShotResult {
        for boat in &mut self.boats {
            if boat.is_at(coord) {
                return boat.hit(coord);
            }
        }

        ShotResult::Missed
    }
    /*    pub fn shoot(&mut self, coord: Coordinates) -> ShotResult {
        let mut boat= self.find_boat_at_position(coord);
        if boat.is_some() {
            //let x : Option<&mut Boat> = boat.as_deref_mut();
            return boat.unwrap().hit(coord);
        }
        ShotResult::Missed
    }

    pub fn find_boat_at_position(&self, coord: Coordinates) -> Option<Boat> {

        for boat in self.boats {
            if boat.is_at(coord) {
                return Option::Some(boat);
            }
        }

        Option::None
    }
*/
    pub fn arrange_boat_with_size(&mut self, boat_size: i8) {
        let mut rng = rand::thread_rng();

        let i = rng.gen_range(1..GRID_SIZE + 1 - boat_size);
        let j = rng.gen_range(1..GRID_SIZE + 1);

        let orientation: bool = rand::random();

        let position = (i..(i + boat_size))
            .collect::<Vec<i8>>()
            .iter_mut()
            .map(|i| if orientation { Coordinates::new(i.clone(), j) } else { Coordinates::new(j, i.clone()) })
            .collect::<Vec<Coordinates>>();

        self.boats.push(Boat::new(position));
    }

    pub fn print_grid(&self) {
        print!("┌");
        self.print_vline();
        print!("┐\n");
        for i in 1..GRID_SIZE {
            print!("|");
            for j in 1..GRID_SIZE {
                print!("{}", self.render_cell(Coordinates::new(i, j)));
            }
            print!("|\n")
        }
        print!("└");
        self.print_vline();
        print!("┘\n");
    }

    fn print_vline(&self) {
        for _ in 1..GRID_SIZE {
            print!("---")
        }
    }

    fn render_cell(&self, coord: Coordinates) -> &str {
        for boat in &self.boats {
            if boat.is_at(coord) {
                return " ■ ";
            }
        }
        return "   ";
    }
}

#[derive(PartialEq, Debug)]
pub struct Boat {
    position: Vec<Coordinates>,
    hits: Vec<Coordinates>,
}

impl Boat {
    pub fn new(position: Vec<Coordinates>) -> Boat {
        return Boat { position, hits: vec![] };
    }

    pub fn is_at(&self, coord: Coordinates) -> bool {
        self.position.contains(&coord)
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

#[derive(PartialEq, Clone, Copy, Debug)]
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


    /*#[test]
    fn find_boat_at_position() {
        let boat_1 = Boat::new(vec![Coordinates::new(5, 2), Coordinates::new(6, 2)]);
        let boat_2 = Boat::new(vec![Coordinates::new(7, 3), Coordinates::new(7, 4), Coordinates::new(7, 5)]);
        let mut board = Board {
            boats: vec![
                boat_1,
                boat_2
            ]
        };
        assert_eq!(
            board.find_boat_at_position(Coordinates::new(7, 3)),
            Option::Some(Boat::new(vec![Coordinates::new(7, 3), Coordinates::new(7, 4), Coordinates::new(7, 5)]))
        );
        assert_eq!(
            board.find_boat_at_position(Coordinates::new(6, 2)),
            Option::Some(Boat::new(vec![Coordinates::new(5, 2), Coordinates::new(6, 2)]))
        );
        assert_eq!(
            board.find_boat_at_position(Coordinates::new(2, 2)),
            Option::None
        );
    }*/

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
