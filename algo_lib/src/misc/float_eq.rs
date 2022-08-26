use crate::misc::float_min_max::fmax;

pub fn feq(x: f64, y: f64, eps: f64) -> bool {
    let abs_diff = (x - y).abs();
    let max_ok = fmax(1.0, fmax(x.abs(), y.abs())) * eps;
    abs_diff <= max_ok
}
