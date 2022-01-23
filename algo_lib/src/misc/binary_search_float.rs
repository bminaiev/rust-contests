use crate::misc::ord_f64::OrdF64;

pub fn float_binary_search_first_true(
    mut left: OrdF64,
    mut right: OrdF64,
    iters: usize,
    mut f: impl FnMut(OrdF64) -> bool,
) -> OrdF64 {
    for _ in 0..iters {
        let mid = (left + right) / OrdF64(2.0);
        if f(mid) {
            right = mid;
        } else {
            left = mid;
        }
    }
    (left + right) / OrdF64(2.0)
}

pub fn float_binary_search_first_false(
    left: OrdF64,
    right: OrdF64,
    iters: usize,
    mut f: impl FnMut(OrdF64) -> bool,
) -> OrdF64 {
    float_binary_search_first_true(left, right, iters, |x| !f(x))
}
