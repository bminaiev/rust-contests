pub trait SortedTrait {
    fn sorted(&self) -> Self;
}

impl<T> SortedTrait for Vec<T>
where
    T: Clone + Ord,
{
    fn sorted(&self) -> Self {
        let mut res = self.to_vec();
        res.sort();
        res
    }
}
