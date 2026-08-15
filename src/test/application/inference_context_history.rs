#[cfg(test)]
mod inference_context_history_tests {
    use crate::application::types::inference_context_history::InferenceContextHistory;

    #[test]
    fn normal_add_test() {
        let mut history = InferenceContextHistory::<i32>::new(5);

        let value1 = 1;
        let value2 = 2;
        let value3 = 3;

        history.add(value1.clone());
        assert_eq!(history.current(), &value1.clone());

        history.add(value2.clone());
        assert_eq!(history.current(), &value2.clone());

        history.add(value3.clone());
        assert_eq!(history.current(), &value3.clone());
    }

    #[test]
    fn undo_test() {
        let mut history = InferenceContextHistory::<i32>::new(5);

        let value1 = 1;
        let value2 = 2;
        let value3 = 3;

        history.add(value1.clone());
        history.add(value2.clone());
        history.add(value3.clone());

        // undoで1つ戻る
        assert!(history.can_undo());
        history.undo();

        assert_eq!(history.current(), &value2.clone());
    }

    #[test]
    fn undo_at_first_test() {
        let mut history = InferenceContextHistory::<i32>::new(5);

        let value1 = 1;
        history.add(value1);

        // 先頭でundo
        assert!(!history.can_undo());
    }

    #[test]
    fn redo_test() {
        let mut history = InferenceContextHistory::<i32>::new(5);

        let value1 = 1;
        let value2 = 2;

        history.add(value1.clone());
        history.add(value2.clone());

        assert!(history.can_undo());
        history.undo();
        // redoで1つ進む
        assert!(history.can_redo());
        history.redo();

        assert_eq!(history.current(), &value2.clone());
    }

    #[test]
    fn redo_at_last_test() {
        let mut history = InferenceContextHistory::<i32>::new(5);

        let value1 = 1;
        let value2 = 2;

        history.add(value1.clone());
        history.add(value2.clone());

        // 末尾でredoすると状態は変わらない
        assert!(!history.can_redo());
        history.redo();
        assert!(!history.can_redo());
    }

    #[test]
    fn add_after_undo_test() {
        let mut history = InferenceContextHistory::<i32>::new(5);

        let value1 = 1;
        let value2 = 2;
        let value3 = 3;

        history.add(value1.clone());
        history.add(value2.clone());
        history.add(value3.clone());

        // 1つundo
        assert!(history.can_undo());
        history.undo();

        let value4 = 4;
        // undo状態でaddすると、redo履歴が破棄される
        history.add(value4.clone());
        assert_eq!(history.current(), &value4.clone());

        assert!(!history.can_redo());

        assert!(history.can_undo());
        history.undo();
        assert_eq!(history.current(), &value2.clone());

        assert!(history.can_undo());
        history.undo();
        assert_eq!(history.current(), &value1.clone());
    }

    // #[test]
    // fn over_capacity_test() {
    //     let mut history = InferenceContextHistory::<i32>::new(3);

    //     let value1 = 1;
    //     let value2 = 2;
    //     let value3 = 3;

    //     history.add(value1.clone());
    //     history.add(value2.clone());
    //     history.add(value3.clone());

    //     history.undo();

    //     assert_eq!(history.current(), [value1.clone(), value2.clone()]);

    //     history.redo();

    //     assert_eq!(
    //         history.current(),
    //         [point1.clone(), point2.clone(), value3.clone()]
    //     )
    // }

    #[test]
    fn undo_3consecutive_test() {
        let mut history = InferenceContextHistory::<i32>::new(5);

        let value1 = 1;
        let value2 = 2;
        let value3 = 3;
        let value4 = 4;

        history.add(value1.clone());
        history.add(value2.clone());
        history.add(value3.clone());
        history.add(value4.clone());

        assert!(history.can_undo());
        history.undo();

        assert!(history.can_undo());
        history.undo();

        assert!(history.can_undo());
        history.undo();

        assert_eq!(history.current(), &value1.clone());

        assert!(!history.can_undo());
    }
}
