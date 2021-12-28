use crate::misc::num_traits::Number;
use std::ops::Range;

pub fn binary_search_first_true<T>(range: Range<T>, mut f: impl FnMut(T) -> bool) -> T
where
    T: Number,
{
    // we can't store [range.start - 1] into [left], because it could overflow
    let mut left_plus_one = range.start;
    let mut right = range.end;
    while right > left_plus_one {
        let mid = left_plus_one + (right - left_plus_one) / T::TWO;
        if f(mid) {
            right = mid;
        } else {
            left_plus_one = mid + T::ONE;
        }
    }
    right
}

pub fn binary_search_last_true<T>(range: Range<T>, mut f: impl FnMut(T) -> bool) -> Option<T>
where
    T: Number,
{
    let first_false = binary_search_first_true(range.clone(), |x| !f(x));
    if first_false == range.start {
        None
    } else {
        Some(first_false - T::ONE)
    }
}

#[test]
fn simple_stress() {
    const N: usize = 50;
    for n in 1..N {
        for cnt_false in 0..=n {
            let mut a = vec![false; cnt_false];
            a.resize(n, true);
            let mut max_f_calls = ((n + 1) as f64).log2().ceil() as i32;
            let f_is_true = |id: usize| -> bool {
                max_f_calls -= 1;
                assert!(max_f_calls >= 0);
                a[id]
            };
            let result = binary_search_first_true(0..n, f_is_true);
            assert_eq!(result, cnt_false);
        }
    }
}
