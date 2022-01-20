pub trait ReversedTrait {
    fn reversed(&self) -> Self;
}

impl<T> ReversedTrait for Vec<T>
where
    T: Clone,
{
    fn reversed(&self) -> Self {
        let mut res = self.to_vec();
        res.reverse();
        res
    }
}
