use crate::misc::num_traits::Number;

pub trait PrefSum<T> {
    fn pref_sum(&self) -> Vec<T>;
}

impl<T> PrefSum<T> for Vec<T>
where
    T: Number,
{
    fn pref_sum(&self) -> Vec<T> {
        let mut res = Vec::with_capacity(self.len() + 1);
        res.push(T::ZERO);
        let mut cur_sum = T::ZERO;
        for &val in self.iter() {
            cur_sum += val;
            res.push(cur_sum);
        }
        res
    }
}
