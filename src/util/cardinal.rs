#[derive(Copy, Clone)]
pub enum Turn {
    Right,
    Left,
}

#[derive(Copy, Clone)]
pub enum Cardinal {
    North,
    East,
    South,
    West,
}

impl Cardinal {
    pub fn turn(self, turn: Turn) -> Self {
        match (self, turn) {
            (Cardinal::North, Turn::Right) => Cardinal::East,
            (Cardinal::North, Turn::Left) => Cardinal::West,
            (Cardinal::East, Turn::Right) => Cardinal::South,
            (Cardinal::East, Turn::Left) => Cardinal::North,
            (Cardinal::South, Turn::Right) => Cardinal::West,
            (Cardinal::South, Turn::Left) => Cardinal::East,
            (Cardinal::West, Turn::Right) => Cardinal::North,
            (Cardinal::West, Turn::Left) => Cardinal::South,
        }
    }
}
