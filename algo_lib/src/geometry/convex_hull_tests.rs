#[cfg(test)]
pub mod tests {
    use crate::geometry::{convex_hull::convex_hull, point::PointT};

    type Point = PointT<i64>;

    fn square_test(size: i64) {
        let mut pts = vec![];
        for x in 0..size {
            for y in 0..size {
                pts.push(Point::new(x, y));
            }
        }
        let hull = convex_hull(&pts);
        let expected = vec![
            Point::ZERO,
            Point::new(size - 1, 0),
            Point::new(size - 1, size - 1),
            Point::new(0, size - 1),
        ];
        assert_eq!(hull, expected);
    }

    #[test]
    fn square_2() {
        square_test(2);
    }

    #[test]
    fn square_3() {
        square_test(3);
    }

    #[test]
    fn square_10() {
        square_test(10);
    }

    fn same_test(a: Vec<Point>) {
        let hull = convex_hull(&a);
        assert_eq!(hull, a);
    }

    #[test]
    fn one() {
        same_test(vec![Point::new(3, 5)]);
    }

    #[test]
    fn two() {
        same_test(vec![Point::new(3, 5), Point::new(10, 12)]);
    }

    #[test]
    fn same_pt() {
        let p = Point::new(3, 5);
        let hull = convex_hull(&[p, p]);
        assert_eq!(hull, vec![p]);
    }

    #[test]
    fn spec() {
        let a = vec![
            Point::ZERO,
            Point::new(10, 0),
            Point::new(10, 10),
            Point::new(5, 5),
        ];
        let hull = convex_hull(&a);
        let expected = vec![Point::ZERO, Point::new(10, 0), Point::new(10, 10)];
        assert_eq!(hull, expected);
    }

    #[test]
    fn spec2() {
        let a = vec![Point::ZERO, Point::new(10, 10), Point::new(5, 5)];
        let hull = convex_hull(&a);
        let expected = vec![Point::ZERO, Point::new(10, 10)];
        assert_eq!(hull, expected);
    }

    #[test]
    fn spec3() {
        let a = vec![
            Point::ZERO,
            Point::new(10, 10),
            Point::new(5, 5),
            Point::new(3, 3),
        ];
        let hull = convex_hull(&a);
        let expected = vec![Point::ZERO, Point::new(10, 10)];
        assert_eq!(hull, expected);
    }
}
