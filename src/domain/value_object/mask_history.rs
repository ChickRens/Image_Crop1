use crate::domain::value_object::mask::Mask;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct MaskHistory {
    masks: Vec<Mask>,
    current_index: usize,
    max_masks: usize,
}

impl MaskHistory {
    pub fn new(max_history: usize) -> Self {
        Self {
            masks: vec![],
            current_index: 0,
            max_masks: max_history,
        }
    }

    pub fn add(&mut self, mask: Mask) {
        // current_index以降の履歴を削除（redo履歴を破棄）
        self.masks.truncate(self.current_index + 1);
        self.masks.push(mask);

        // self.masksがmax_masksを超えたら先頭を削除
        if self.masks.len() > self.max_masks {
            self.masks.remove(0);
        }
        self.current_index = self.masks.len() - 1;
    }

    pub fn current(&self) -> Option<&Mask> {
        self.masks.get(self.current_index)
    }

    pub fn undo(&mut self) -> Option<&Mask> {
        if self.current_index == 0 {
            None
        } else {
            self.current_index -= 1;
            let undid_mask: &Mask = &self.masks[self.current_index];
            Some(&undid_mask)
        }
    }

    pub fn redo(&mut self) -> Option<&Mask> {
        self.current_index += 1;
        self.masks.get(self.current_index)
    }
}
