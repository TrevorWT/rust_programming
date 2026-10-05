//! A simple robot simulator: a robot has a position, a facing direction,
//! and can turn left/right or advance one step in the direction it faces.

/// The four compass directions a [`Robot`] can face.
#[derive(PartialEq, Eq, Debug)]
pub enum Direction {
    North,
    East,
    South,
    West,
}

/// A robot on an infinite 2D grid, tracked as `(x, y, facing)`.
pub struct Robot(i32, i32, Direction);

impl Robot {
    /// Creates a new robot at position `(x, y)` facing direction `d`.
    pub fn new(x: i32, y: i32, d: Direction) -> Self {
        Robot(x, y, d)
    }

    /// Returns the robot turned 90 degrees clockwise, keeping its position.
    #[must_use]
    pub fn turn_right(mut self) -> Self {
        match self.2 {
            Direction::North => self.2 = Direction::East,
            Direction::East => self.2 = Direction::South,
            Direction::South => self.2 = Direction::West,
            Direction::West => self.2 = Direction::North,
        }
        self
    }

    /// Returns the robot turned 90 degrees counter-clockwise, keeping its
    /// position.
    #[must_use]
    pub fn turn_left(mut self) -> Self {
        match self.2 {
            Direction::North => self.2 = Direction::West,
            Direction::East => self.2 = Direction::North,
            Direction::South => self.2 = Direction::East,
            Direction::West => self.2 = Direction::South,
        }
        self
    }

    /// Returns the robot moved one step forward in the direction it is
    /// currently facing.
    #[must_use]
    pub fn advance(mut self) -> Self {
        match self.2 {
            Direction::North => self.1 += 1,
            Direction::East => self.0 += 1,
            Direction::South => self.1 -= 1,
            Direction::West => self.0 -= 1,
        }
        self
    }

    /// Applies a sequence of instructions to the robot, returning the
    /// resulting robot.
    ///
    /// `'L'` turns left, `'R'` turns right, `'A'` advances one step; any
    /// other character is ignored.
    #[must_use]
    pub fn instructions(mut self, instructions: &str) -> Self {
        for c in instructions.chars() {
            self = match c {
                'L' => self.turn_left(),
                'R' => self.turn_right(),
                'A' => self.advance(),
                _ => self,
            };
        }
        self
    }

    /// Returns the robot's current `(x, y)` position.
    pub fn position(&self) -> (i32, i32) {
        (self.0, self.1)
    }

    /// Returns the robot's current facing direction.
    pub fn direction(&self) -> &Direction {
        &self.2
    }
}
