use crate::misc::num_traits::HasConstants;
use crate::misc::ord_f64::OrdF64;
use std::cmp::Ordering;
use std::fmt::Debug;

#[derive(Ord, PartialOrd, Eq, PartialEq, Debug)]
pub struct TernarySearchResult<T>
where
    T: Ord,
{
    pub value: T,
    pub key: OrdF64,
}

fn check_function_is_convex<T>(from: OrdF64, to: OrdF64, f: &mut impl FnMut(OrdF64) -> T)
where
    T: Ord + Debug,
{
    const PARTS: usize = 100;
    let values: Vec<(OrdF64, T)> = (0..=PARTS)
        .map(|id| {
            let key = from + (to - from) / OrdF64(PARTS as f64) * OrdF64(id as f64);
            (key, f(key))
        })
        .collect();
    let expected_order = [Ordering::Less, Ordering::Equal, Ordering::Greater];
    let mut cur_iter = 0;
    let print_debug = |index: usize| {
        for (cur_index, (key, value)) in values.iter().enumerate() {
            eprint!("{} -> {:?}", key, value);
            if cur_index == index {
                eprintln!("  <- !!!!! ");
            } else {
                eprintln!();
            }
        }
        panic!("Function is not convex!");
    };
    for (index, w) in values.windows(2).enumerate() {
        let cmp = w[0].cmp(&w[1]);
        if cmp == expected_order[cur_iter] {
            continue;
        }
        cur_iter += 1;
        if cur_iter == expected_order.len() {
            print_debug(index);
        }
        if expected_order[cur_iter] != cmp {
            print_debug(index);
        }
    }
}

pub fn ternary_search_find_max<T>(
    mut from: OrdF64,
    mut to: OrdF64,
    num_iters: usize,
    mut f: impl FnMut(OrdF64) -> T,
) -> TernarySearchResult<T>
where
    T: Ord + Debug,
{
    if cfg!(debug_assertions) {
        check_function_is_convex(from, to, &mut f);
    }
    for _ in 0..num_iters {
        let m1 = from + (to - from) / OrdF64(3.0);
        let m2 = to - (to - from) / OrdF64(3.0);
        let val1 = f(m1);
        let val2 = f(m2);
        if val1 > val2 {
            to = m2;
        } else {
            from = m1;
        }
    }
    let res_key = (from + to) / OrdF64::TWO;
    TernarySearchResult {
        key: res_key,
        value: f(res_key),
    }
}
