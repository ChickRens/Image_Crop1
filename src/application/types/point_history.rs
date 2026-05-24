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
        self.points.truncate(self.current_index + 1);
        self.points.push(point);

        // self.points がmax_pointsを超えたら先頭を削除
        if self.points.len() > self.max_points {
            self.points.remove(0);
        }
        self.current_index = self.points.len() - 1;
    }

    pub fn current(&self) -> Option<&Point> {
        self.points.get(self.current_index)
    }

    pub fn undo(&mut self) -> Option<&Point> {
        if self.current_index == 0 {
            None
        } else {
            self.current_index -= 1;
            let undid_mask: &Point = &self.points[self.current_index];
            Some(&undid_mask)
        }
    }

    pub fn redo(&mut self) -> Option<&Point> {
        self.current_index += 1;
        self.points.get(self.current_index)
    }
}
