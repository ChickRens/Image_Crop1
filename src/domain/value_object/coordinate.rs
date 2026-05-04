#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Coordinate {
    x: u16,
    y: u16,
}

impl Coordinate {
    pub fn new(x: u16, y: u16) -> Self {
        Self { x, y }
    }

    pub fn x(&self) -> u16 {
        self.x
    }

    pub fn y(&self) -> u16 {
        self.y
    }
}
