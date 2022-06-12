
struct Board {
    boats: Vec<Boat>
}

impl Board {
    fn shoot(&mut self, coord: Coordinates) -> ShotResult {
        for boat in &mut self.boats {
            if boat.is_at(&coord) {
                return boat.hit(coord);
            }
        }

        ShotResult::Missed
    }
}


struct Boat {
    position: Vec<Coordinates>,
    hits: Vec<Coordinates>,
}

impl Boat {
    fn new(position: Vec<Coordinates>) -> Boat {
        return Boat { position, hits: vec![] };
    }

    fn is_at(&self, coord: &Coordinates) -> bool {
        self.position.contains(coord)
    }

    fn hit(&mut self, coord: Coordinates) -> ShotResult {
        self.hits.push(coord);
        for boat_coord in &self.position {
            if !self.hits.contains(boat_coord) {
                return ShotResult::Hit;
            }
        }
        return ShotResult::Destroyed
    }
}

#[derive(PartialEq, Debug)]
struct Coordinates {
    x: i32,
    y: i32,
}

impl Coordinates {
    fn new(x: i32, y: i32) -> Coordinates {
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
