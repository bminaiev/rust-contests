use crate::misc::num_traits::Number;

pub fn rev_permutation<T>(a: &[T]) -> Vec<T>
where
    T: Number,
{
    let n = a.len();
    let mut res = vec![T::ZERO; n];
    for (pos, value) in a.iter().enumerate() {
        res[value.to_i32() as usize] = T::from_i32(pos as i32);
    }
    res
}
