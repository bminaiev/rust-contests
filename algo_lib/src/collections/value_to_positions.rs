use crate::misc::num_traits::Number;

pub fn calc_value_occurrences<T>(a: &[T]) -> Vec<usize>
where
    T: Number,
{
    let max_val = a.iter().max().unwrap().to_i32() as usize;
    let mut res = vec![0; max_val + 1];
    for val in a.iter() {
        res[val.to_i32() as usize] += 1;
    }
    res
}

pub fn calc_value_to_positions<T>(a: &[T]) -> Vec<Vec<T>>
where
    T: Number,
{
    let mut res: Vec<_> = calc_value_occurrences(a)
        .into_iter()
        .map(|sz| Vec::with_capacity(sz))
        .collect();
    for val in a.iter() {
        res[val.to_i32() as usize].push(*val);
    }
    res
}
