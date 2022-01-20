use crate::misc::num_traits::Number;

pub trait VecBinarySearch<T> {
    fn higher_or_equal(&self, val: &T) -> Option<T>
    where
        T: Number;
    fn lower(&self, val: &T) -> Option<T>
    where
        T: Number;
}

impl<T> VecBinarySearch<T> for Vec<T>
where
    T: Number,
{
    fn higher_or_equal(&self, val: &T) -> Option<T>
    where
        T: Number,
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
        T: Number,
    {
        match self.binary_search(&val) {
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
    T: Number,
{
    fn higher_or_equal(&self, val: &T) -> Option<T>
    where
        T: Number,
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
        T: Number,
    {
        match self.binary_search(&val) {
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
