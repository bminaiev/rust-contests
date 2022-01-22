use crate::misc::num_traits::Number;

pub struct StepDownIterator<T>
where
    T: Number,
{
    last_returned: T,
    till_inclusive: T,
    step_by: T,
}

impl<T> Iterator for StepDownIterator<T>
where
    T: Number,
{
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        return if self.last_returned == self.till_inclusive {
            None
        } else {
            self.last_returned -= self.step_by;
            Some(self.last_returned)
        };
    }
}

pub fn step_down<T>(from: T, to_incl: T, step_by: T) -> impl Iterator<Item = T>
where
    T: Number,
{
    assert!(from >= to_incl);
    let cnt = T::ONE + (from - to_incl) / step_by;
    let till_inclusive = from - (cnt - T::ONE) * step_by;
    StepDownIterator {
        last_returned: from + step_by,
        till_inclusive,
        step_by,
    }
}
