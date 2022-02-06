#[cfg(test)]
pub mod tests {
    use crate::geometry::half_plane_intersection::half_plane_intersection;
    use crate::geometry::point::PointT;
    use crate::geometry::segment::SegmentT;
    use crate::misc::num_traits::ConvI32;
    use crate::misc::ord_f64::OrdF64;
    use crate::misc::rand::Random;

    type Point = PointT<OrdF64>;

    #[test]
    fn simple() {
        for test in 0..1000 {
            let mut rnd = Random::new(787788 + test);
            let n = rnd.gen_in_range(1..10);
            let mut segs = vec![];
            const MAX_C: i32 = 20000;
            let mut gen_coord = || -> OrdF64 { OrdF64(rnd.gen_in_range(-MAX_C..MAX_C) as f64) };
            let mut gen_point = || -> Point { Point::new(gen_coord(), gen_coord()) };
            for _ in 0..n {
                let p1 = gen_point();
                let p2 = gen_point();
                if p1 != p2 {
                    segs.push(SegmentT::new(p1, p2));
                }
            }
            if let Some(poly) = half_plane_intersection(segs, Some(OrdF64::from_i32(MAX_C * 2))) {
                let _area = poly.area();
            }
        }
    }

    #[test]
    fn small_coord() {
        for test in 0..10000 {
            let mut rnd = Random::new(1787788 + test);
            let n = rnd.gen_in_range(1..6);
            let mut segs = vec![];
            const MAX_C: i32 = 5;
            let mut gen_coord = || -> OrdF64 { OrdF64(rnd.gen_in_range(-MAX_C..MAX_C) as f64) };
            let mut gen_point = || -> Point { Point::new(gen_coord(), gen_coord()) };
            for _ in 0..n {
                let p1 = gen_point();
                let p2 = gen_point();
                if p1 != p2 {
                    segs.push(SegmentT::new(p1, p2));
                }
            }
            if let Some(poly) = half_plane_intersection(segs, Some(OrdF64::from_i32(MAX_C * 2))) {
                let _area = poly.area();
            }
        }
    }
}
