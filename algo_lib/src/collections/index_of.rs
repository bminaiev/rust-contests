pub trait IndexOf<T> {
    fn index_of(&self, elem: &T) -> Option<usize>;
}

impl<T: Eq> IndexOf<T> for &[T] {
    fn index_of(&self, elem: &T) -> Option<usize> {
        self.iter().position(|cur| cur == elem)
    }
}

impl<T: Eq> IndexOf<T> for Vec<T> {
    fn index_of(&self, elem: &T) -> Option<usize> {
        self.iter().position(|cur| cur == elem)
    }
}
