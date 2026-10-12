use super::double_double::DoubleDouble as DD;
use super::num_traits::{ConvSimple, HasConstants, Number};
use crate::geometry::line_dd::Line;
use crate::geometry::point::PointT;
use crate::geometry::segment::SegmentT;
use crate::geometry::segment_intersection_dd::segment_intersection_dd;
use std::cmp::Ordering;

#[test]
fn implements_number_and_works_in_generic_geometry() {
    fn exercise<T: Number>(x: T) -> T {
        let mut result = T::ZERO;
        result += x;
        result *= T::TWO;
        result /= T::TWO;
        result -= T::ONE;
        result
    }
    let large = DD::from(1_i128 << 100);
    assert_eq!(exercise(large + DD::ONE), large);
    assert_eq!(<DD as HasConstants<DD>>::ZERO, DD::default());
    assert_eq!(<DD as ConvSimple<DD>>::from_i32(7), DD::from(7));
    assert_eq!(<DD as ConvSimple<DD>>::to_f64(DD::TWO), 2.0);
    assert_eq!(<DD as ConvSimple<DD>>::to_i128(large), 1_i128 << 100);

    // These products cancel to -1, which ordinary f64 cannot retain.
    let k = DD::from(1_i64 << 52);
    let a = PointT::<DD>::new(k + DD::ONE, k);
    let b = PointT::<DD>::new(k, k - DD::ONE);
    assert_eq!(PointT::vect_mul2(&a, &b), -DD::ONE);
}

#[test]
fn arithmetic_keeps_low_components() {
    let large = DD::from(1_i128 << 100);
    assert_eq!((large + DD::ONE) - large, DD::ONE);
    assert_eq!((large - DD::ONE) - large, -DD::ONE);
    let tiny = DD::from(2.0_f64.powi(-52));
    assert_eq!(
        (DD::ONE + tiny) * (DD::ONE - tiny) - DD::ONE,
        -(tiny * tiny)
    );
    let third = DD::ONE / DD::from(3);
    assert!((third * DD::from(3) - DD::ONE).abs() < DD::from(1e-30));
    assert!(third != DD::from(1.0 / 3.0));
    assert_eq!(f64::from(DD::from(1.25)), 1.25);
}

#[test]
fn comparisons_use_the_low_component_and_normalize_inputs() {
    let large = DD::from(1_i128 << 100);
    assert!(large + DD::ONE > large);
    assert!(large - DD::ONE < large);
    assert_eq!(DD::new(1.0, 2.0_f64.powi(100)), large + DD::ONE);
    assert_eq!(DD::new(1.0, -1.0), DD::ZERO);
    assert_eq!(DD::from(-0.0), DD::ZERO);
    assert_eq!(DD::NAN.partial_cmp(&DD::ONE), None);
    assert_eq!(DD::ONE.partial_cmp(&DD::NAN), None);
    assert_ne!(DD::NAN, DD::NAN);
    assert_eq!(DD::INFINITY.partial_cmp(&DD::MAX), Some(Ordering::Greater));
}

#[test]
fn ordering_sorts_using_both_components() {
    let large = DD::from(1_i128 << 100);
    let mut values = [
        large + DD::ONE,
        DD::INFINITY,
        large - DD::ONE,
        DD::ZERO,
        DD::NEG_INFINITY,
        large,
        -DD::ONE,
    ];
    values.sort();
    assert_eq!(
        values,
        [
            DD::NEG_INFINITY,
            -DD::ONE,
            DD::ZERO,
            large - DD::ONE,
            large,
            large + DD::ONE,
            DD::INFINITY,
        ]
    );
    assert_eq!(DD::from(-0.0).cmp(&DD::ZERO), Ordering::Equal);
    assert_eq!(large.cmp(&large), Ordering::Equal);
}

#[test]
#[should_panic]
fn ordering_panics_on_nan() {
    let _ = DD::ONE.cmp(&DD::NAN);
}

#[test]
fn approximate_equality_uses_absolute_and_relative_tolerances() {
    assert!(DD::ZERO.eq_with_default_eps(&DD::EPS));
    assert!(!DD::ZERO.eq_with_default_eps(&(DD::EPS * DD::TWO)));

    let large = DD::from(1_i128 << 100);
    let next = large + DD::ONE;
    let eps = DD::from(2.0_f64.powi(-100));
    assert!(large.eq_with_eps(&next, eps));
    assert!(next.eq_with_eps(&large, eps));
    assert!((-large).eq_with_eps(&-next, eps));
    assert!(!large.eq_with_eps(&next, eps / DD::TWO));
    assert!(!large.eq_with_eps(&next, DD::ZERO));
    assert!(DD::from(-0.0).eq_with_eps(&DD::ZERO, DD::ZERO));

    assert!(DD::INFINITY.eq_with_default_eps(&DD::INFINITY));
    assert!(DD::NEG_INFINITY.eq_with_default_eps(&DD::NEG_INFINITY));
    assert!(!DD::INFINITY.eq_with_default_eps(&DD::NEG_INFINITY));
    assert!(!DD::INFINITY.eq_with_default_eps(&DD::ONE));
    assert!(!DD::ONE.eq_with_default_eps(&DD::INFINITY));
    assert!(!DD::NAN.eq_with_default_eps(&DD::ONE));
    assert!(!DD::ONE.eq_with_default_eps(&DD::NAN));
    assert!(!DD::NAN.eq_with_default_eps(&DD::NAN));
}

#[test]
fn square_root_keeps_low_components() {
    assert_eq!(DD::from(9).sqrt(), DD::from(3));
    let tiny = 2.0_f64.powi(-60);
    assert_eq!(DD::new(1.0, tiny).sqrt(), DD::new(1.0, tiny / 2.0));
    assert_eq!(DD::new(1.0, -tiny).sqrt(), DD::new(1.0, -tiny / 2.0));

    let root = DD::TWO.sqrt();
    assert_ne!(root, DD::from(2.0_f64.sqrt()));
    assert!((root * root - DD::TWO).abs() < DD::from(1e-30));

    // Scaling by even powers of two must preserve both components of sqrt(2).
    // This covers subnormal inputs, the scaling boundary, and huge inputs.
    for exponent in [-537, -511, -486, -485, -484, -200, 200, 511] {
        let value = DD::from(if exponent == -537 {
            f64::from_bits(2)
        } else {
            2.0_f64.powi(2 * exponent + 1)
        });
        let scale = DD::from(2.0_f64.powi(exponent));
        assert!(
            (value.sqrt() / scale - root).abs() < DD::from(1e-30),
            "exponent = {exponent}"
        );
    }
    assert_eq!(
        DD::from(f64::from_bits(1)).sqrt(),
        DD::from(2.0_f64.powi(-537))
    );
    let max_root = DD::MAX.sqrt();
    assert!(max_root.is_finite());
    assert!(max_root > DD::ZERO);
}

#[test]
fn square_root_handles_special_values() {
    assert_eq!(DD::ZERO.sqrt(), DD::ZERO);
    assert_eq!(DD::ONE / DD::from(-0.0).sqrt(), DD::NEG_INFINITY);
    assert_eq!(DD::INFINITY.sqrt(), DD::INFINITY);
    assert!(DD::NEG_INFINITY.sqrt().is_nan());
    assert!(DD::NAN.sqrt().is_nan());
    assert!((-DD::ONE).sqrt().is_nan());
}

#[test]
fn double_double_geometry_uses_sqrt_and_tolerances() {
    let origin = PointT::<DD>::new(DD::ZERO, DD::ZERO);
    let diagonal = SegmentT::new(origin, PointT::new(DD::from(3), DD::from(4)));
    assert_eq!(diagonal.len(), DD::from(5));
    let line = diagonal.to_line();
    assert!(line.on_line(&PointT::new(DD::from(6), DD::from(8))));
    assert!(line
        .abs_dist(&PointT::new(DD::from(4), DD::from(-3)))
        .eq_with_default_eps(&DD::from(5)));

    let horizontal = SegmentT::new(origin, PointT::new(DD::from(4), DD::ZERO));
    assert!(horizontal.contains(&PointT::new(DD::ONE, DD::EPS / DD::from(8))));
    assert!(!horizontal.contains(&PointT::new(DD::ONE, DD::EPS)));
    assert!(!horizontal.contains(&PointT::new(DD::from(5), DD::ZERO)));
    let vertical = SegmentT::new(
        PointT::new(DD::TWO, -DD::ONE),
        PointT::new(DD::TWO, DD::ONE),
    );
    assert_eq!(
        segment_intersection_dd(&horizontal, &vertical),
        Some(PointT::new(DD::TWO, DD::ZERO))
    );
    let parallel = Line::new(
        &PointT::new(DD::ZERO, DD::ONE),
        &PointT::new(DD::ONE, DD::ONE),
    );
    assert!(horizontal.to_line().intersect(&parallel).is_none());
}

#[test]
fn integer_conversions_cover_the_full_range() {
    for value in [
        i128::MIN,
        i128::MIN + 1,
        i128::MAX,
        i128::MAX - 1,
        -(1_i128 << 100) - 12345,
        (1_i128 << 100) + 12345,
        i64::MIN as i128,
        i64::MAX as i128,
        u64::MAX as i128,
        -1,
        0,
        1,
    ] {
        assert_eq!(DD::from(value).to_i128(), value, "{value}");
    }
    let power = DD::from(2.0_f64.powi(128));
    assert_eq!(DD::from(u128::MAX) - power, -DD::ONE);
    assert_eq!(DD::from(u128::MAX).to_i128(), i128::MAX);
    assert_eq!(DD::from(u64::MAX), DD::from(u64::MAX as i128));
}

#[test]
fn integer_conversions_truncate_and_saturate() {
    let tiny = 2.0_f64.powi(-60);
    for (value, expected) in [
        (DD::new(1.0, -tiny), 0),
        (DD::new(-1.0, tiny), 0),
        (DD::new(2.0, -tiny), 1),
        (DD::new(-2.0, tiny), -1),
        (DD::from(1.75), 1),
        (DD::from(-1.75), -1),
        (DD::new(2.0_f64.powi(127), -1.5), i128::MAX - 1),
        (DD::new(-2.0_f64.powi(127), 1.5), i128::MIN + 2),
        (DD::INFINITY, i128::MAX),
        (DD::NEG_INFINITY, i128::MIN),
        (DD::NAN, 0),
    ] {
        assert_eq!(value.to_i128(), expected, "{value:?}");
    }
    assert_eq!(DD::from(i128::MAX).to_i32(), i32::MAX);
    assert_eq!(DD::from(i128::MIN).to_i32(), i32::MIN);
    assert_eq!(DD::new(i32::MAX as f64, -tiny).to_i32(), i32::MAX - 1);
}

#[test]
fn iterator_operations_keep_precision() {
    let large = DD::from(1_i128 << 100);
    let mut values = vec![DD::ONE; 100001];
    values[0] = large;
    values[100000] = -large;
    assert_eq!(values.iter().sum::<DD>(), DD::from(99999));
    assert_eq!(values.into_iter().sum::<DD>(), DD::from(99999));
    let factors = [DD::from(3), DD::from(4), DD::from(5)];
    assert_eq!(factors.iter().product::<DD>(), DD::from(60));
    assert_eq!(factors.into_iter().product::<DD>(), DD::from(60));
    assert_eq!(std::iter::empty::<DD>().sum::<DD>(), DD::ZERO);
    assert_eq!(std::iter::empty::<DD>().product::<DD>(), DD::ONE);
}

#[test]
fn special_values_follow_float_semantics() {
    assert_eq!(DD::INFINITY + DD::ONE, DD::INFINITY);
    assert!((DD::INFINITY - DD::INFINITY).is_nan());
    assert_eq!(DD::INFINITY * -DD::ONE, DD::NEG_INFINITY);
    assert!((DD::INFINITY * DD::ZERO).is_nan());
    assert_eq!(DD::ONE / DD::ZERO, DD::INFINITY);
    assert_eq!(-DD::ONE / DD::ZERO, DD::NEG_INFINITY);
    assert!((DD::ZERO / DD::ZERO).is_nan());
    assert_eq!(DD::ONE / DD::INFINITY, DD::ZERO);
    assert_eq!(DD::NEG_INFINITY.abs(), DD::INFINITY);
    assert!(DD::MAX.is_finite());
    assert_eq!(DD::new(f64::MAX, 9.979201547673598e291), DD::MAX);
    assert_eq!(DD::MAX.abs(), DD::MIN.abs());
}
