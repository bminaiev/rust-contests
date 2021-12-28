use crate::misc::num_traits::Number;

pub fn facts<T>(n: usize) -> Vec<T>
where
    T: Number,
{
    let mut res = Vec::with_capacity(n);
    res.push(T::ONE);
    for x in 1..=n {
        let num = T::try_from(x as i32).unwrap_or(T::ZERO);
        res.push(*res.last().unwrap() * num);
    }
    res
}
