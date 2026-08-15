#[derive(Debug, PartialEq, Eq, Clone)]
pub struct InferenceContextHistory<T> {
    items: Vec<T>,
    current_index: usize,
    max_items: usize,
}

impl<T> InferenceContextHistory<T> {
    pub fn new(max_history: usize) -> Self {
        Self {
            items: vec![],
            current_index: 0,
            max_items: max_history,
        }
    }

    pub fn add(&mut self, item: T) {
        // current_index以降の履歴を削除（redo履歴を破棄）
        self.items.truncate(self.current_index);
        self.items.push(item);

        self.current_index = self.items.len();
    }

    pub fn current(&self) -> &T {
        &self.items[self.current_index - 1]
    }

    pub fn can_undo(&self) -> bool {
        if self.current_index <= 1 {
            return false;
        }
        true
    }
    
    pub fn undo(&mut self) {
        self.current_index -= 1;
    }

    pub fn can_redo(&self) -> bool {
        if self.current_index >= self.items.len() {
            return false;
        }
        true
    }

    pub fn redo(&mut self) {
        self.current_index += 1;
    }
}
