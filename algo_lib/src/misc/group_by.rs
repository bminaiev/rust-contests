pub struct GroupBy<'a, T, P> {
    slice: &'a [T],
    predicate: P,
}
pub trait GroupByTrait<T> {
    fn group_by_<P>(&self, p: P) -> GroupBy<T, P>
    where
        P: FnMut(&T, &T) -> bool;
}

impl<T> GroupByTrait<T> for [T] {
    fn group_by_<P>(&self, p: P) -> GroupBy<T, P>
    where
        P: FnMut(&T, &T) -> bool,
    {
        GroupBy {
            slice: self,
            predicate: p,
        }
    }
}

impl<'a, T, P> Iterator for GroupBy<'a, T, P>
where
    P: FnMut(&T, &T) -> bool,
{
    type Item = &'a [T];

    fn next(&mut self) -> Option<Self::Item> {
        if self.slice.is_empty() {
            return None;
        }
        let mut next = 1;
        while next != self.slice.len() && (self.predicate)(&self.slice[next - 1], &self.slice[next])
        {
            next += 1;
        }
        let res = &self.slice[0..next];
        self.slice = &self.slice[next..];
        Some(res)
    }
}
