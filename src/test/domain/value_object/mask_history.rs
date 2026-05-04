#[cfg(test)]
mod mask_history_tests {
    use crate::domain::value_object::image_size::ImageSize;
    use crate::domain::value_object::mask::Mask;
    use crate::domain::value_object::mask_history::MaskHistory;

    fn create_mask(width: u16, height: u16) -> Mask {
        Mask::new(ImageSize::new(width, height).unwrap())
    }

    #[test]
    fn normal_add_test() {
        let mut history = MaskHistory::new(5);

        history.add(create_mask(100, 100));
        assert_eq!(*history.current().unwrap(), create_mask(100, 100));

        history.add(create_mask(200, 200));
        assert_eq!(*history.current().unwrap(), create_mask(200, 200));

        history.add(create_mask(300, 300));
        assert_eq!(*history.current().unwrap(), create_mask(300, 300));
    }

    #[test]
    fn undo_test() {
        let mut history = MaskHistory::new(5);

        history.add(create_mask(100, 100));
        history.add(create_mask(200, 200));
        history.add(create_mask(300, 300));

        // undoで1つ戻る
        let result = history.undo();
        assert_eq!(*result.unwrap(), create_mask(200, 200));
    }

    #[test]
    fn undo_at_first_test() {
        let mut history = MaskHistory::new(5);

        history.add(create_mask(100, 100));

        // 先頭でundo
        let mask = history.undo();
        assert!(mask.is_none());
    }

    #[test]
    fn redo_test() {
        let mut history = MaskHistory::new(5);

        history.add(create_mask(100, 100));
        history.add(create_mask(200, 200));

        let _ = history.undo();
        // redoで1つ進む
        let result = history.redo();
        assert_eq!(*result.unwrap(), create_mask(200, 200));
    }

    #[test]
    fn redo_at_last_test() {
        let mut history = MaskHistory::new(5);

        history.add(create_mask(100, 100));
        history.add(create_mask(200, 200));

        // 末尾でredoするとエラー
        let result = history.redo();
        assert!(result.is_none());
    }

    #[test]
    fn add_after_undo_test() {
        let mut history = MaskHistory::new(5);

        history.add(create_mask(100, 100));
        assert_eq!(*history.current().unwrap(), create_mask(100, 100));

        history.add(create_mask(200, 200));
        assert_eq!(*history.current().unwrap(), create_mask(200, 200));

        history.add(create_mask(300, 300));
        assert_eq!(*history.current().unwrap(), create_mask(300, 300));

        // 1つundo
        let _ = history.undo();

        // undo状態でaddすると、redo履歴が破棄される
        history.add(create_mask(400, 400));

        // masksは [100, 200, 400] になる（300は破棄）
        assert_eq!(*history.current().unwrap(), create_mask(400, 400));

        let _ = history.undo();
        assert_eq!(*history.current().unwrap(), create_mask(200, 200));
    }

    #[test]
    fn over_capacity_test() {
        let mut history = MaskHistory::new(2);
        history.add(create_mask(100, 100));
        history.add(create_mask(200, 200));
        history.add(create_mask(300, 300));

        let _ = history.undo();

        assert_eq!(*history.current().unwrap(), create_mask(200, 200));
        assert_eq!(*history.redo().unwrap(), create_mask(300, 300))
    }

    #[test]
    fn undo_3consecutive_test() {
        let mut history = MaskHistory::new(5);
        history.add(create_mask(100, 100));
        history.add(create_mask(200, 200));
        history.add(create_mask(300, 300));
        history.add(create_mask(400, 400));

        let _ = history.undo();
        let _ = history.undo();
        let _ = history.undo();

        assert_eq!(*history.current().unwrap(), create_mask(100, 100))
    }
}
