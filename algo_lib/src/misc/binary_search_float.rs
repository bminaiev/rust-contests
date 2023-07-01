use crate::misc::ord_f64::OrdF64;

pub fn float_binary_search_first_true(
    mut left: OrdF64,
    mut right: OrdF64,
    iters: usize,
    mut f: impl FnMut(OrdF64) -> bool,
) -> OrdF64 {
    if cfg!(debug_assertions) {
        struct FuncRes {
            key: OrdF64,
            value: bool,
        }
        let mut values = vec![];
        for it in 0..iters {
            let key =
                left + (right - left) / OrdF64((iters + 1) as f64) * (OrdF64((it + 1) as f64));
            let value = f(key);
            values.push(FuncRes { key, value });
        }
        let first_true = values
            .iter()
            .position(|elem| elem.value)
            .unwrap_or(values.len());
        for (pos, val) in values.iter().enumerate() {
            let expected = pos >= first_true;
            if expected != val.value {
                for (pos2, val) in values.iter().enumerate() {
                    if pos2 == pos || pos2 == first_true {
                        eprint!("!!! ");
                    }
                    eprintln!("{} --> {}", val.key, val.value);
                }
                unreachable!("Function is not monotonic!");
            }
        }
    }
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
