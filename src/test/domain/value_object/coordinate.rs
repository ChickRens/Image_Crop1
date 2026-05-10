#[cfg(test)]
mod coordinate_tests {
    use crate::domain::value_object::coordinate::Coordinate;

    #[test]
    fn test_coordinate_new() {
        let coordinate = Coordinate::new(10, 20);
        assert_eq!(coordinate.x(), 10);
        assert_eq!(coordinate.y(), 20);
    }
}
