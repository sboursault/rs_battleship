use rand::Rng;

const GRID_SIZE: i8 = 10;


pub struct Board {
    pub boats: Vec<Boat>
}

impl Board {
    pub fn shoot(&mut self, coord: Coordinates) -> ShotResult {
        return match self.find_boat_at_coord_as_mut(coord) {
            Some(boat) => boat.hit(coord),
            None => ShotResult::Missed
        };
    }

    fn find_boat_at_coord_as_mut(&mut self, coord: Coordinates) -> Option<&mut Boat> {
        return self.boats.iter_mut()
            .filter(|boat| boat.is_at(coord))
            .nth(0);
    }

    fn find_boat_at_coord(&self, coord: Coordinates) -> Option<&Boat> {
        return self.boats.iter()
            .filter(|boat| boat.is_at(coord))
            .nth(0);
    }

    pub fn arrange_boat_with_size(&mut self, boat_size: i8) {
        let mut rng = rand::thread_rng();

        let mut is_free_position: bool = false;

        let mut position: Vec<Coordinates> = vec![];  // there's probably a better way...

        while is_free_position == false {
            let i = rng.gen_range(1..GRID_SIZE + 1 - boat_size);
            let j = rng.gen_range(1..GRID_SIZE + 1);

            let orientation: bool = rand::random();

            position = (i..(i + boat_size))
                .collect::<Vec<i8>>()
                .iter_mut()
                .map(|i| if orientation { Coordinates::new(i.clone(), j) } else { Coordinates::new(j, i.clone()) })
                .collect::<Vec<Coordinates>>();

            is_free_position = self.is_free_position(&position);
        }

        self.boats.push(Boat::new(position));
    }

    fn is_free_position(&self, position: &Vec<Coordinates>) -> bool {
        let overlap: Option<&Coordinates> =
            position.iter()
                .find(|coord| self.find_boat_at_coord(**coord).is_some()); // MAIS CEST QUOI CE DEREFERENCEMENT ??!!!! possible grace au trait Copy ?
        return overlap.is_none();
    }

    pub fn print_grid(&self) {
        print!("   ┌");
        self.print_line();
        print!("┐\n");
        for i in 1..GRID_SIZE + 1 {
            print!("{: >2} |", i);  // left padd number with white space
            for j in 1..GRID_SIZE + 1 {
                print!("{}", self.render_cell(Coordinates::new(i, j)));
            }
            print!("|\n")
        }
        print!("   └");
        self.print_line();
        print!("┘\n");
        println!("     A  B  C  D  E  F  G  H  I  J");
    }

    fn print_line(&self) {
        for _ in 1..GRID_SIZE + 1 {
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

    #[test]
    fn is_free_position() {
        let mut board = Board {
            boats: vec![
                Boat::new(vec![Coordinates::new(5, 2), Coordinates::new(6, 2)]),
                Boat::new(vec![Coordinates::new(7, 3), Coordinates::new(7, 4), Coordinates::new(7, 5)])
            ]
        };
        assert_eq!(board.is_free_position(&vec![Coordinates::new(5, 1), Coordinates::new(5, 2)]), false);
        assert_eq!(board.is_free_position(&vec![Coordinates::new(8, 4), Coordinates::new(8, 5)]), true);
    }
}
