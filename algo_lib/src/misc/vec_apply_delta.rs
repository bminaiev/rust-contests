use crate::misc::num_traits::Number;

pub trait ApplyDelta<T> {
    fn add_to_all(self, delta: T) -> Self;
    fn sub_from_all(self, sub: T) -> Self;
}

impl<T> ApplyDelta<T> for Vec<T>
where
    T: Number,
{
    fn add_to_all(mut self, delta: T) -> Self {
        self.iter_mut().for_each(|val| *val += delta);
        self
    }

    fn sub_from_all(mut self, sub: T) -> Self {
        self.iter_mut().for_each(|val| *val -= sub);
        self
    }
}
