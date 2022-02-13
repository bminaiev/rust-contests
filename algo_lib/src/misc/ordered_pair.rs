use std::cmp::{max, min};

#[derive(Hash, Clone, Copy, PartialEq, PartialOrd, Ord, Eq, Debug)]
pub struct OrderedPair<T> {
    pub min: T,
    pub max: T,
}

impl<T: Ord + Clone> OrderedPair<T> {
    pub fn new(x: T, y: T) -> Self {
        Self {
            min: min(x.clone(), y.clone()),
            max: max(x, y),
        }
    }
}
