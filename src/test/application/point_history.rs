#[cfg(test)]
mod point_history_tests {
    use crate::application::types::point_history::PointHistory;
    use crate::domain::value_object::coordinate::Coordinate;
    use crate::domain::value_object::point::PointLabel::{BACKGROUND, FOREGROUND};
    use crate::domain::value_object::point::{Point, PointLabel};

    fn create_point(x: u16, y: u16, label: PointLabel) -> Point {
        Point::new(Coordinate::new(x, y), label)
    }

    #[test]
    fn normal_add_test() {
        let mut history = PointHistory::new(5);

        let point1 = create_point(100, 100, BACKGROUND);
        let point2 = create_point(200, 200, FOREGROUND);
        let point3 = create_point(300, 300, BACKGROUND);

        history.add(point1.clone());
        assert_eq!(history.current(), Some(&[point1.clone()][..]));

        history.add(point2.clone());
        assert_eq!(history.current(), Some(&[point1.clone(), point2.clone()][..]));

        history.add(point3.clone());
        assert_eq!(
            history.current(),
            Some(&[point1.clone(), point2.clone(), point3.clone()][..])
        );
    }

    #[test]
    fn undo_test() {
        let mut history = PointHistory::new(5);

        let point1 = create_point(100, 100, BACKGROUND);
        let point2 = create_point(200, 200, FOREGROUND);
        let point3 = create_point(300, 300, BACKGROUND);

        history.add(point1.clone());
        history.add(point2.clone());
        history.add(point3.clone());

        // undoで1つ戻る
        assert!(history.can_undo());
        history.undo();

        assert_eq!(history.current(), Some(&[point1.clone(), point2.clone()][..]));
    }

    #[test]
    fn undo_at_first_test() {
        let mut history = PointHistory::new(5);

        let point1 = create_point(100, 100, BACKGROUND);
        history.add(point1);

        // 先頭でundo
        assert!(history.can_undo());
        history.undo();
        assert_eq!(history.current(), Some(&[] as &[Point]));

        assert!(!history.can_undo());
    }

    #[test]
    fn redo_test() {
        let mut history = PointHistory::new(5);

        let point1 = create_point(100, 100, BACKGROUND);
        let point2 = create_point(200, 200, FOREGROUND);

        history.add(point1.clone());
        history.add(point2.clone());

        assert!(history.can_undo());
        history.undo();
        // redoで1つ進む
        assert!(history.can_redo());
        history.redo();

        assert_eq!(history.current(), Some(&[point1.clone(), point2.clone()][..]));
    }

    #[test]
    fn redo_at_last_test() {
        let mut history = PointHistory::new(5);

        let point1 = create_point(100, 100, FOREGROUND);
        let point2 = create_point(200, 200, FOREGROUND);

        history.add(point1.clone());
        history.add(point2.clone());

        assert!(!history.can_redo());
    }

    #[test]
    fn add_after_undo_test() {
        let mut history = PointHistory::new(5);

        let point1 = create_point(100, 100, BACKGROUND);
        let point2 = create_point(200, 200, FOREGROUND);
        let point3 = create_point(300, 300, BACKGROUND);

        history.add(point1.clone());
        history.add(point2.clone());
        history.add(point3.clone());

        // 1つundo
        history.undo();

        let point4 = create_point(400, 400, FOREGROUND);
        // undo状態でaddすると、redo履歴が破棄される
        history.add(point4.clone());

        // masksは [100, 200, 400] になる（300は破棄）
        assert_eq!(
            history.current(),
            Some(&[point1.clone(), point2.clone(), point4.clone()][..])
        );

        history.undo();
        assert_eq!(history.current(), Some(&[point1.clone(), point2.clone()][..]));
    }

    // #[test]
    // fn over_capacity_test() {
    //     let mut history = PointHistory::new(2);

    //     let point1 = create_point(100, 100, PointLabel::BACKGROUND);
    //     let point2 = create_point(200, 200, PointLabel::BACKGROUND);
    //     let point3 = create_point(300, 300, PointLabel::FOREGROUND);

    //     history.add(point1.clone());
    //     history.add(point2.clone());
    //     history.add(point3.clone());

    //     let _ = history.undo();

    //     assert_eq!(history.current(), [point1.clone(), point2.clone()]);

    //     history.redo();

    //     assert_eq!(
    //         history.current(),
    //         [point1.clone(), point2.clone(), point3.clone()]
    //     )
    // }

    #[test]
    fn undo_3consecutive_test() {
        let mut history = PointHistory::new(5);

        let point1 = create_point(100, 100, FOREGROUND);
        let point2 = create_point(200, 200, BACKGROUND);
        let point3 = create_point(300, 300, FOREGROUND);
        let point4 = create_point(400, 400, BACKGROUND);

        history.add(point1.clone());
        history.add(point2.clone());
        history.add(point3.clone());
        history.add(point4.clone());

        history.undo();
        history.undo();
        history.undo();

        assert_eq!(history.current(), Some(&[point1.clone()][..]))
    }
}
