#[cfg(test)]
mod mask_history_tests {
    use crate::domain::value_object::coordinate::Coordinate;
    use crate::domain::value_object::point::{Point, PointLabel};
    use crate::domain::value_object::point_history::PointHistory;

    fn create_point(x: u16, y: u16, label: PointLabel) -> Point {
        Point::new(Coordinate::new(x, y), label)
    }

    #[test]
    fn normal_add_test() {
        let mut history = PointHistory::new(5);

        history.add(create_point(100, 100, PointLabel::BACKGROUND));
        assert_eq!(
            *history.current().unwrap(),
            create_point(100, 100, PointLabel::BACKGROUND)
        );

        history.add(create_point(200, 200, PointLabel::FOREGROUND));
        assert_eq!(
            *history.current().unwrap(),
            create_point(200, 200, PointLabel::FOREGROUND)
        );

        history.add(create_point(300, 300, PointLabel::BACKGROUND));
        assert_eq!(
            *history.current().unwrap(),
            create_point(300, 300, PointLabel::BACKGROUND)
        );
    }

    #[test]
    fn undo_test() {
        let mut history = PointHistory::new(5);

        history.add(create_point(100, 100, PointLabel::BACKGROUND));
        history.add(create_point(200, 200, PointLabel::FOREGROUND));
        history.add(create_point(300, 300, PointLabel::BACKGROUND));

        // undoで1つ戻る
        let result = history.undo();
        assert_eq!(
            *result.unwrap(),
            create_point(200, 200, PointLabel::FOREGROUND)
        );
    }

    #[test]
    fn undo_at_first_test() {
        let mut history = PointHistory::new(5);

        history.add(create_point(100, 100, PointLabel::BACKGROUND));

        // 先頭でundo
        let mask = history.undo();
        assert!(mask.is_none());
    }

    #[test]
    fn redo_test() {
        let mut history = PointHistory::new(5);

        history.add(create_point(100, 100, PointLabel::BACKGROUND));
        history.add(create_point(200, 200, PointLabel::FOREGROUND));

        let _ = history.undo();
        // redoで1つ進む
        let result = history.redo();
        assert_eq!(
            *result.unwrap(),
            create_point(200, 200, PointLabel::FOREGROUND)
        );
    }

    #[test]
    fn redo_at_last_test() {
        let mut history = PointHistory::new(5);

        history.add(create_point(100, 100, PointLabel::FOREGROUND));
        history.add(create_point(200, 200, PointLabel::FOREGROUND));

        // 末尾でredoするとエラー
        let result = history.redo();
        assert!(result.is_none());
    }

    #[test]
    fn add_after_undo_test() {
        let mut history = PointHistory::new(5);

        history.add(create_point(100, 100, PointLabel::BACKGROUND));
        assert_eq!(
            *history.current().unwrap(),
            create_point(100, 100, PointLabel::BACKGROUND)
        );

        history.add(create_point(200, 200, PointLabel::FOREGROUND));
        assert_eq!(
            *history.current().unwrap(),
            create_point(200, 200, PointLabel::FOREGROUND)
        );

        history.add(create_point(300, 300, PointLabel::BACKGROUND));
        assert_eq!(
            *history.current().unwrap(),
            create_point(300, 300, PointLabel::BACKGROUND)
        );

        // 1つundo
        let _ = history.undo();

        // undo状態でaddすると、redo履歴が破棄される
        history.add(create_point(400, 400, PointLabel::FOREGROUND));

        // masksは [100, 200, 400] になる（300は破棄）
        assert_eq!(
            *history.current().unwrap(),
            create_point(400, 400, PointLabel::FOREGROUND)
        );

        let _ = history.undo();
        assert_eq!(
            *history.current().unwrap(),
            create_point(200, 200, PointLabel::FOREGROUND)
        );
    }

    #[test]
    fn over_capacity_test() {
        let mut history = PointHistory::new(2);
        history.add(create_point(100, 100, PointLabel::BACKGROUND));
        history.add(create_point(200, 200, PointLabel::BACKGROUND));
        history.add(create_point(300, 300, PointLabel::FOREGROUND));

        let _ = history.undo();

        assert_eq!(
            *history.current().unwrap(),
            create_point(200, 200, PointLabel::BACKGROUND)
        );
        assert_eq!(
            *history.redo().unwrap(),
            create_point(300, 300, PointLabel::FOREGROUND)
        )
    }

    #[test]
    fn undo_3consecutive_test() {
        let mut history = PointHistory::new(5);
        history.add(create_point(100, 100, PointLabel::FOREGROUND));
        history.add(create_point(200, 200, PointLabel::BACKGROUND));
        history.add(create_point(300, 300, PointLabel::FOREGROUND));
        history.add(create_point(400, 400, PointLabel::BACKGROUND));

        let _ = history.undo();
        let _ = history.undo();
        let _ = history.undo();

        assert_eq!(
            *history.current().unwrap(),
            create_point(100, 100, PointLabel::FOREGROUND)
        )
    }
}
