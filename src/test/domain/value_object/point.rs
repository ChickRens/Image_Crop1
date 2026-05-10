#[cfg(test)]
mod point_tests {
    use crate::domain::value_object::{coordinate::Coordinate, point::Point, point::PointLabel};

    #[test]
    fn test_point_new_and_accessors() {
        let coordinate = Coordinate::new(3, 4);
        let point = Point::new(coordinate.clone(), PointLabel::FOREGROUND);

        assert_eq!(point.coordinate(), &coordinate);
        assert_eq!(point.label(), &PointLabel::FOREGROUND);
    }
}
