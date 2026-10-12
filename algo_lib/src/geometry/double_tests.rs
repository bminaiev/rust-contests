use super::convex_polygon_intersection::convex_polygon_intersection;
use super::dcel::dcel;
use super::half_plane_intersection::half_plane_intersection;
use super::line::LineT;
use super::point::{PointT, PointWithIdT};
use super::polygon::PolygonT;
use super::segment::SegmentT;
use super::segment_intersection::{inside_bounding_box, segment_intersection};
use crate::misc::double_double::DoubleDouble;
use crate::misc::num_traits::Double;
use crate::misc::ord_f64::OrdF64;

fn point<T: Double>(x: i64, y: i64) -> PointT<T> {
    PointT::new(T::from(x), T::from(y))
}

fn check_lines_and_distances<T: Double>() {
    let origin = point::<T>(0, 0);
    let end = point::<T>(3, 4);
    let diagonal = SegmentT::new(origin, end);
    assert_eq!(diagonal.len2(), T::from(25_i64));
    assert!(diagonal.len().eq_with_default_eps(&T::from(5_i64)));
    assert!(origin.dist(&end).eq_with_default_eps(&T::from(5_i64)));
    assert!(PointWithIdT::new(origin, 0)
        .dist(&PointWithIdT::new(end, 1))
        .eq_with_default_eps(&T::from(5_i64)));
    assert_eq!(end * T::TWO, point(6, 8));

    let mut line = LineT::<T>::new3(0, -2, 4);
    line.norm();
    assert_eq!(line, LineT::new_gcd((0, 2), (-4, 2)));
    assert!(line.on_line(&point(3, 2)));
    assert!(!line.on_line(&origin));
    assert_eq!(line.closest_to_zero(), point(0, 2));
    assert_eq!(line.closest_on_line(end), point(3, 2));
    assert_eq!(line.mirror(end), point(3, 0));
    assert_eq!(line.signed_dist(&origin), -T::TWO);
    assert_eq!(line.abs_dist(&origin), T::TWO);
    assert_eq!(line.abs_dist2(&origin), T::from(4_i64));
    assert_eq!(line.project(end), T::from(-3_i64));
}

fn check_segment_intersections<T: Double>() {
    let horizontal = SegmentT::new(point::<T>(0, 0), point(4, 0));
    let reversed = SegmentT::new(horizontal.to, horizontal.from);
    let vertical = SegmentT::new(point(2, -1), point(2, 1));
    let crossing = Some(point(2, 0));
    assert_eq!(segment_intersection(&horizontal, &vertical), crossing);
    assert_eq!(segment_intersection(&vertical, &reversed), crossing);

    let endpoint = SegmentT::new(point(4, 0), point(4, 2));
    assert_eq!(
        segment_intersection(&horizontal, &endpoint),
        Some(horizontal.to)
    );
    let outside = SegmentT::new(point(5, -1), point(5, 1));
    assert!(segment_intersection(&horizontal, &outside).is_none());
    let parallel = SegmentT::new(point(0, 1), point(4, 1));
    assert!(segment_intersection(&horizontal, &parallel).is_none());
    // The API returns only unique intersections, not collinear overlaps.
    assert!(segment_intersection(&horizontal, &reversed).is_none());
    let degenerate = SegmentT::new(horizontal.from, horizontal.from);
    assert!(segment_intersection(&horizontal, &degenerate).is_none());

    let near = PointT::new(T::ONE, T::EPS / T::from(8_i64));
    assert!(reversed.contains(&near));
    assert!(!horizontal.contains(&PointT::new(T::ONE, T::EPS)));
    assert!(inside_bounding_box(
        &reversed,
        &PointT::new(-T::EPS / T::TWO, T::ZERO)
    ));
    assert!(!inside_bounding_box(
        &horizontal,
        &PointT::new(-T::EPS * T::TWO, T::ZERO)
    ));
}

fn check_polygon_intersections<T: Double>() {
    let rectangle = PolygonT::new_rect(point::<T>(0, 0), point(4, 3));
    assert_eq!(rectangle.area(), T::from(12_i64));
    let planes: Vec<_> = rectangle.edges().collect();
    let intersection = half_plane_intersection(planes, None).unwrap();
    assert_eq!(intersection.area(), rectangle.area());
    let bounded = half_plane_intersection(Vec::new(), Some(T::TWO)).unwrap();
    assert_eq!(bounded.area(), T::from(16_i64));

    let other = PolygonT::new_rect(point(2, 1), point(5, 4));
    let overlap = convex_polygon_intersection(&rectangle, &other).unwrap();
    assert_eq!(overlap.area(), T::from(4_i64));
    assert_eq!(overlap.min_x(), T::TWO);
    assert_eq!(overlap.max_x(), T::from(4_i64));
    let outside = PolygonT::new_rect(point(5, 1), point(6, 4));
    assert!(convex_polygon_intersection(&rectangle, &outside).is_none());

    let lines: Vec<_> = rectangle.edges().map(|edge| edge.to_line()).collect();
    let faces = dcel(&lines);
    assert_eq!(faces.len(), 1);
    assert_eq!(faces[0].area(), rectangle.area());
}

fn check_double_comparisons<T: Double>() {
    assert!(T::ZERO.eq_with_default_eps(&T::EPS));
    assert!(!T::ZERO.eq_with_default_eps(&(T::EPS * T::TWO)));
    assert!(T::from(1e10).eq_with_default_eps(&T::from(1e10 + 1.0)));
    let inf = T::from(f64::INFINITY);
    let nan = T::from(f64::NAN);
    assert!(inf.eq_with_default_eps(&inf));
    assert!(!inf.eq_with_default_eps(&-inf));
    assert!(!T::ONE.eq_with_default_eps(&inf));
    assert!(!nan.eq_with_default_eps(&T::ZERO));
    assert!(!T::ZERO.eq_with_default_eps(&nan));
}

#[test]
fn geometry_with_ord_f64() {
    check_double_comparisons::<OrdF64>();
    check_lines_and_distances::<OrdF64>();
    check_segment_intersections::<OrdF64>();
    check_polygon_intersections::<OrdF64>();
}

#[test]
fn geometry_with_double_double() {
    check_double_comparisons::<DoubleDouble>();
    check_lines_and_distances::<DoubleDouble>();
    check_segment_intersections::<DoubleDouble>();
    check_polygon_intersections::<DoubleDouble>();
}

#[test]
fn generic_intersections_preserve_double_double_low_components() {
    type D = DoubleDouble;
    let base = D::from(1_i128 << 100);
    let x = base + D::ONE;
    let horizontal = SegmentT::new(
        PointT::<D>::new(base, D::ZERO),
        PointT::new(base + D::from(4), D::ZERO),
    );
    let vertical = SegmentT::new(PointT::new(x, -D::ONE), PointT::new(x, D::ONE));
    let intersection = segment_intersection(&horizontal, &vertical).unwrap();
    assert_eq!(intersection.x - base, D::ONE);
    assert_eq!(intersection.y, D::ZERO);
    assert_eq!(horizontal.len(), D::from(4));

    let integer = (1_i64 << 60) + 1;
    let line = LineT::<D>::new3(1, 0, -integer);
    assert_eq!(line.closest_to_zero().x.to_i128(), integer as i128);
}
