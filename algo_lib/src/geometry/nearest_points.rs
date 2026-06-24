use crate::{
    geometry::{
        line::Line,
        point::{PointT, PointWithIdT},
    },
    misc::{ord_f64::OrdF64, rand::Random},
};

type Point = PointT<i64>;
type PointId = PointWithIdT<i64>;
type PointD = PointT<OrdF64>;

pub fn find_nearest_points(points: &[Point]) -> Vec<usize> {
    // for each point, returns the index of the nearest point
    let n = points.len();
    let mut res = vec![usize::MAX; n];

    let mut rnd = Random::new_time_seed();
    let a = rnd.gen_range(1..1_000_000);
    let b = rnd.gen_range(1..1_000_000);
    let line = Line::new(&PointD::ZERO, &PointD::new(a as f64, b as f64));

    let mut points_with_ids = points
        .iter()
        .enumerate()
        .map(|(i, p)| {
            let np = PointId::new(*p, i);
            let dist = line.signed_dist(&PointD::new(np.p.x, np.p.y));
            (dist, np)
        })
        .collect::<Vec<_>>();

    points_with_ids.sort_by_key(|(dist, _)| *dist);

    for i in 0..n {
        let mut best_dist2 = i64::MAX;
        let mut best_id = usize::MAX;
        for j in (0..i).rev() {
            let dist_by_line = (points_with_ids[i].0 - points_with_ids[j].0).abs();
            if dist_by_line * dist_by_line >= OrdF64(best_dist2 as f64) {
                break;
            }
            let dist2 = points_with_ids[i].1.p.dist2(&points_with_ids[j].1.p);
            if dist2 < best_dist2 {
                best_dist2 = dist2;
                best_id = points_with_ids[j].1.id();
            }
        }
        for j in (i + 1)..n {
            let dist_by_line = (points_with_ids[i].0 - points_with_ids[j].0).abs();
            if dist_by_line * dist_by_line >= OrdF64(best_dist2 as f64) {
                break;
            }
            let dist2 = points_with_ids[i].1.p.dist2(&points_with_ids[j].1.p);
            if dist2 < best_dist2 {
                best_dist2 = dist2;
                best_id = points_with_ids[j].1.id();
            }
        }
        res[points_with_ids[i].1.id()] = best_id;
    }

    res
}
