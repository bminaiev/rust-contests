#[cfg(test)]
#[allow(non_snake_case)]
mod tests {
    use crate::{
        geometry::{point::PointT, segment_intersection_coef::segment_interection_coef},
        math::frac::FracT,
    };

    type Point = PointT<i64>;
    type Frac = FracT<i128>;

    fn p(x: i64, y: i64) -> Point {
        Point::new(x, y)
    }

    fn f(x: i128, y: i128) -> Frac {
        Frac::new(x, y)
    }

    #[test]
    fn simple() {
        let A = p(0, 0);
        let B = p(2, 0);
        let C = p(0, 2);

        {
            let D = p(1, 0);
            let u = segment_interection_coef([A, B], [C, D]);
            assert_eq!(u, Some(f(1, 2)));
        }

        {
            let D = p(1, 1);
            let u = segment_interection_coef([A, B], [C, D]);
            assert_eq!(u, Some(f(1, 1)));
        }

        {
            let D = p(0, 1);
            let u = segment_interection_coef([A, B], [C, D]);
            assert_eq!(u, Some(f(0, 1)));
        }

        {
            let D = p(1, 2);
            let u = segment_interection_coef([A, B], [C, D]);
            assert_eq!(u, None);
        }

        {
            let D = p(-1, 3);
            let u = segment_interection_coef([A, B], [C, D]);
            assert_eq!(u, Some(f(1, 1)));
        }

        {
            let D = p(3, 0);
            let u = segment_interection_coef([A, B], [C, D]);
            assert_eq!(u, Some(f(3, 2)));
        }

        {
            let D = p(-1, 0);
            let u = segment_interection_coef([A, B], [C, D]);
            assert_eq!(u, Some(f(-1, 2)));
        }
    }

    #[test]
    fn real() {
        let A = p(-1, 2);
        let B = p(-2, 4);
        let C = p(0, -1);
        let D = p(6, 5);
        let u = segment_interection_coef([A, B], [C, D]);
        let u = u.unwrap();
        assert!(u < f(0, 1));
    }
}
