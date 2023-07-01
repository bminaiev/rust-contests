use crate::{geometry::point::PointT, misc::num_traits::Number};

pub fn convex_hull<T: Number>(a: &[PointT<T>]) -> Vec<PointT<T>> {
    if a.is_empty() {
        return vec![];
    }
    let mut a = a.to_vec();
    a.sort();
    a.dedup();
    let start = a[0];
    a[1..].sort_by(|p1, p2| {
        PointT::vect_mul(&start, p1, p2)
            .cmp(&T::ZERO)
            .reverse()
            .then(start.dist2(p1).cmp(&start.dist2(p2)))
    });
    let mut res = vec![start];
    for &p in a[1..].iter() {
        while res.len() >= 2 {
            let sz = res.len();
            let p2 = res[sz - 2];
            let p1 = res[sz - 1];
            if PointT::vect_mul(&p2, &p1, &p) > T::ZERO {
                break;
            } else {
                res.pop();
            }
        }
        res.push(p);
    }
    res
}
