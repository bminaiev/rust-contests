use crate::misc::binary_search::binary_search_first_true;
use crate::misc::min_max::UpdateMinMax;

pub fn longest_increasing_subsequence<T>(a: &[T]) -> usize
where
    T: Ord + Clone,
{
    let mut res = Vec::with_capacity(a.len());
    for val in a.iter() {
        let first_bigger_or_equal = binary_search_first_true(0..res.len(), |pos| res[pos] >= val);
        if first_bigger_or_equal == res.len() {
            res.push(val);
        } else {
            res[first_bigger_or_equal].update_min(val);
        }
    }
    res.len()
}
