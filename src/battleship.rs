
struct Board {
    boats: Vec<Boat>
}

impl Board {
    fn shoot(&mut self, coord: Coordinates) -> ShotResult {

        for boat in &self.boats {
            if boat.is_at(&coord) {
                return ShotResult::Hit
            }
        }

        ShotResult::Missed
    }

}


struct Boat {
    position: Vec<Coordinates>
}

impl Boat {
    fn new(position: Vec<Coordinates>) -> Boat {
        return Boat { position };
    }

    fn is_at(&self, coord: &Coordinates) -> bool {
        self.position.contains(coord)
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
    fn shoot_missed() {
        let mut board = Board {
            boats: vec![
                Boat::new(vec![Coordinates::new(5, 2), Coordinates::new(6, 2)]),
                Boat::new(vec![Coordinates::new(7, 3), Coordinates::new(7, 4), Coordinates::new(7, 5)])
            ]
        };
        assert_eq!(board.shoot(Coordinates::new(2,2)), ShotResult::Missed);
    }

    #[test]
    fn shoot_hit() {
        let mut board = Board {
            boats: vec![
                Boat::new(vec![Coordinates::new(5, 2), Coordinates::new(6, 2)]),
                Boat::new(vec![Coordinates::new(7, 3), Coordinates::new(7, 4), Coordinates::new(7, 5)])
            ]
        };
        assert_eq!(board.shoot(Coordinates::new(6,2)), ShotResult::Hit);
    }
}
