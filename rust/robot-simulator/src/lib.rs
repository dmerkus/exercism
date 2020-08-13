// The code below is a stub. Just enough to satisfy the compiler.
// In order to pass the tests you can add-to or change any of this code.

#[derive(PartialEq, Debug)]
pub enum Direction {
    North,
    East,
    South,
    West,
}

pub struct Location {
    x: i32,
    y: i32,
}

pub struct Robot {
    location: Location,
    direction: Direction,
}

impl Robot {
    pub fn new(x: i32, y: i32, d: Direction) -> Self {
        Robot {
            location: Location { x, y },
            direction: d,
        }
    }

    pub fn turn_right(mut self) -> Self {
        self.direction = match self.direction {
            Direction::North => Direction::East,
            Direction::East => Direction::South,
            Direction::South => Direction::West,
            Direction::West => Direction::North,
        };

        self
    }

    pub fn turn_left(mut self) -> Self {
        self.direction = match self.direction {
            Direction::North => Direction::West,
            Direction::East => Direction::North,
            Direction::South => Direction::East,
            Direction::West => Direction::South,
        };

        self
    }

    pub fn advance(mut self) -> Self {
        match self.direction {
            Direction::North => self.location.y += 1,
            Direction::East => self.location.x += 1,
            Direction::South => self.location.y -= 1,
            Direction::West => self.location.x -= 1,
        }

        self
    }

    pub fn instructions(self, instructions: &str) -> Self {
        instructions
            .chars()
            .fold(self, |robot, instruction| match instruction {
                'A' => robot.advance(),
                'L' => robot.turn_left(),
                'R' => robot.turn_right(),
                _ => panic!("unrecognized instruction"),
            })
    }

    pub fn position(&self) -> (i32, i32) {
        (self.location.x, self.location.y)
    }

    pub fn direction(&self) -> &Direction {
        &self.direction
    }
}
