use crate::misc::num_traits::Number;

pub fn gen_facts<T>(n: usize) -> Vec<T>
where
    T: Number,
{
    let mut res = Vec::with_capacity(n);
    res.push(T::ONE);
    for x in 1..=n {
        let num = T::from_i32(x as i32);
        res.push(*res.last().unwrap() * num);
    }
    res
}
