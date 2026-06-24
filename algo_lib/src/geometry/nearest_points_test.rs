#[cfg(test)]
mod tests {
    use crate::{
        geometry::{nearest_points::find_nearest_points, point::PointT},
        misc::rand::Random,
    };

    type Point = PointT<i64>;

    fn find_nearest_points_slow(points: &[Point]) -> Vec<usize> {
        let n = points.len();
        let mut res = vec![usize::MAX; n];
        for i in 0..n {
            let mut best_dist2 = i64::MAX;
            for j in 0..n {
                if i == j {
                    continue;
                }
                let dist2 = points[i].dist2(&points[j]);
                if dist2 < best_dist2 {
                    best_dist2 = dist2;
                    res[i] = j;
                }
            }
        }
        res
    }

    #[test]
    fn stress() {
        let mut rnd = Random::new(123);

        for _ in 0..30 {
            for n in [2, 5, 10, 100, 1000] {
                let mut points = vec![];
                for _ in 0..n {
                    points.push(Point::new(
                        rnd.gen_range(-1_000_000..1_000_000),
                        rnd.gen_range(-1_000_000..1_000_000),
                    ));
                }
                let res = find_nearest_points(&points);
                let res_slow = find_nearest_points_slow(&points);
                for i in 0..n {
                    let d1 = points[i].dist2(&points[res[i]]);
                    let d2 = points[i].dist2(&points[res_slow[i]]);
                    assert_eq!(d1, d2);
                }
            }
        }
    }

    #[test]
    fn speed() {
        let mut rnd = Random::new(123);
        let n = 100_000;
        let mut points = vec![];
        for _ in 0..n {
            points.push(Point::new(
                rnd.gen_range(-1_000_000..1_000_000),
                rnd.gen_range(-1_000_000..1_000_000),
            ));
        }
        find_nearest_points(&points);
    }
}
