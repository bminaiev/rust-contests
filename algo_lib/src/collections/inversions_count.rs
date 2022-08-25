use crate::seg_trees::fenwick::Fenwick;

pub fn inversions_count(a: &[usize]) -> i64 {
    let mut seen = Fenwick::new(a.len());
    let mut res = 0i64;
    for &x in a.iter().rev() {
        res += seen.get_sum(x);
        seen.add(x, 1);
    }
    res
}
