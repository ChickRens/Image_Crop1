use crate::domain::value_object::coordinate::Coordinate;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Point {
    coordinate: Coordinate,
    label: PointLabel,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum PointLabel {
    BACKGROUND = 0,
    FOREGROUND = 1,
}

impl Point {
    pub fn new(coordinate: Coordinate, label: PointLabel) -> Self {
        Self { coordinate, label }
    }

    pub fn coordinate(&self) -> &Coordinate {
        &self.coordinate
    }

    pub fn label(&self) -> &PointLabel {
        &self.label
    }
}
