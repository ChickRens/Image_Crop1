use crate::domain::value_object::point::Point;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct PointHistory {
    points: Vec<Point>,
    current_index: usize,
    max_points: usize,
}

impl PointHistory {
    pub fn new(max_history: usize) -> Self {
        Self {
            points: vec![],
            current_index: 0,
            max_points: max_history,
        }
    }

    pub fn add(&mut self, point: Point) {
        // current_index以降の履歴を削除（redo履歴を破棄）
        self.points.truncate(self.current_index);
        self.points.push(point);

        self.current_index = self.points.len();
    }

    pub fn current(&self) -> &[Point] {
        &self.points[0..self.current_index]
    }

    pub fn undo(&mut self) -> bool {
        if self.current_index <= 1 {
            false
        } else {
            self.current_index -= 1;
            true
        }
    }

    pub fn redo(&mut self) -> bool {
        if self.current_index >= self.points.len() {
            false
        } else {
            self.current_index += 1;
            true
        }
    }
}
