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

pub trait ApplyDelta2<T> {
    fn add_to_all(&mut self, delta: T);
    fn sub_from_all(&mut self, sub: T);
}

impl<T> ApplyDelta2<T> for [T]
where
    T: Number,
    T: Sized,
{
    fn add_to_all(self: &mut [T], delta: T) {
        self.iter_mut().for_each(|x| *x += delta);
    }

    fn sub_from_all(&mut self, sub: T) {
        self.iter_mut().for_each(|x| *x -= sub);
    }
}
