use crate::misc::rec_function::{Callable2, RecursiveFunction2};

pub struct MatchingResult {
    pub right: Vec<Option<usize>>,
    pub size: usize,
}

pub fn find_matching(g: &[Vec<usize>], right_size: usize) -> MatchingResult {
    let n = g.len();
    let mut seen_right = vec![0; right_size];
    let mut left = vec![None; right_size];
    let mut seen_iter = 0;
    let mut go = RecursiveFunction2::new(|f, v: usize, seen_iter: i32| -> bool {
        for &to in g[v].iter() {
            if seen_right[to] == seen_iter {
                continue;
            }
            seen_right[to] = seen_iter;
            if let Some(from_left) = left[to] {
                if f.call(from_left, seen_iter) {
                    left[to] = Some(v);
                    return true;
                }
            } else {
                left[to] = Some(v);
                return true;
            }
        }
        false
    });
    let mut size = 0;
    for v in 0..n {
        seen_iter += 1;
        if go.call(v, seen_iter) {
            size += 1;
        }
    }
    let mut right = vec![None; n];
    for (pos, val) in left.iter().enumerate() {
        if let Some(val) = val {
            right[*val] = Some(pos);
        }
    }
    MatchingResult { right, size }
}
