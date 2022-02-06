pub trait PartialCmp<T>
where
    T: PartialOrd,
{
    fn sort_partial_cmp(&mut self);
}

impl<T> PartialCmp<T> for &mut [T]
where
    T: PartialOrd,
{
    fn sort_partial_cmp(&mut self) {
        self.sort_by(|x, y| x.partial_cmp(&y).expect("Partial_cmp failed :("))
    }
}

impl<T> PartialCmp<T> for Vec<T>
where
    T: PartialOrd,
{
    fn sort_partial_cmp(&mut self) {
        self.sort_by(|x, y| x.partial_cmp(&y).expect("Partial cmp failed"))
    }
}
