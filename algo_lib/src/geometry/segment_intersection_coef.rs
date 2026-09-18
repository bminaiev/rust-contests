use crate::{geometry::point::PointT, math::frac::FracT};

type Point = PointT<i64>;
type Frac = FracT<i128>;

fn c(p: Point) -> PointT<i128> {
    PointT::new(p.x as i128, p.y as i128)
}

// s1 = [A, B], s2 = [C, D]
// finds such `u` that A + u * (B - A) = C + v * (D - C) for some `v`
// None if segment are parallel
#[allow(non_snake_case)]
pub fn segment_interection_coef(s1: [Point; 2], s2: [Point; 2]) -> Option<Frac> {
    // s(ADC) / s(BCD) = u / (1 - u)
    // u = s(ADC) / (s(ADC) + s(BCD))
    let A = c(s1[0]);
    let B = c(s1[1]);
    let C = c(s2[0]);
    let D = c(s2[1]);
    let s_adc = PointT::vect_mul(&A, &D, &C);
    let s_bcd = PointT::vect_mul(&B, &C, &D);
    if s_adc + s_bcd == 0 {
        return None;
    }
    Some(Frac::new(s_adc, s_adc + s_bcd))
}
