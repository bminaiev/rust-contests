pub trait VecBinarySearch<T> {
    fn higher_or_equal(&self, val: &T) -> Option<T>
    where
        T: Ord + Clone;
    fn lower(&self, val: &T) -> Option<T>
    where
        T: Ord + Clone;
}

impl<T> VecBinarySearch<T> for Vec<T>
where
    T: Ord + Clone,
{
    fn higher_or_equal(&self, val: &T) -> Option<T>
    where
        T: Ord + Clone,
    {
        match self.binary_search(val) {
            Ok(pos) => Some(self[pos].clone()),
            Err(pos) => {
                if pos == self.len() {
                    None
                } else {
                    Some(self[pos].clone())
                }
            }
        }
    }

    fn lower(&self, val: &T) -> Option<T>
    where
        T: Ord + Clone,
    {
        match self.binary_search(val) {
            Ok(pos) | Err(pos) => {
                if pos == 0 {
                    None
                } else {
                    Some(self[pos - 1].clone())
                }
            }
        }
    }
}

impl<T> VecBinarySearch<T> for &[T]
where
    T: Ord + Clone,
{
    fn higher_or_equal(&self, val: &T) -> Option<T>
    where
        T: Ord + Clone,
    {
        match self.binary_search(val) {
            Ok(pos) => Some(self[pos].clone()),
            Err(pos) => {
                if pos == self.len() {
                    None
                } else {
                    Some(self[pos].clone())
                }
            }
        }
    }

    fn lower(&self, val: &T) -> Option<T>
    where
        T: Ord + Clone,
    {
        match self.binary_search(val) {
            Ok(pos) | Err(pos) => {
                if pos == 0 {
                    None
                } else {
                    Some(self[pos - 1].clone())
                }
            }
        }
    }
}
