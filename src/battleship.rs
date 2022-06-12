struct Board {
    boats: [Boat]
}

impl Board {
    fn shoot(&mut self, coord: Coordinates) -> ShotResult {
        ShotResult::Miss
    }
}


struct Boat {
    position: Vec<Coordinates>  // could be an array ?
}

impl Boat {
    fn new(position: Vec<Coordinates>) -> Boat {
        return Boat { position };
    }
}

struct Coordinates {
    x: i32,
    y: i32,
}

fn new_coord(x: i32, y: i32) -> Coordinates {
    return Coordinates { x, y };
}


enum ShotResult {
    Missed,
    Hit,
    Destroyed,
}


#[cfg(test)] // Only compiles when running tests
mod tests {
    use super::*;

    #[test]
    fn shoot_missed() {
        let Board {
            boats: [
            Boat::new(vec![new_coord(5, 2), new_coord(6, 2)]),
            Boat::new(vec![new_coord(7, 3), new_coord(7, 4), new_coord(7, 5)])
            ]
        };
        assert_eq!(details, "Taupiqueur: 10hp 5mp")
    }
}
