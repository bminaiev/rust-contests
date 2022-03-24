pub struct PairsIter<I: Iterator> {
    first_iter: I,
    second_iter: I,
    first_elem: Option<I::Item>,
}

fn new_pairs_iter<I: Iterator + Clone>(mut iter: I) -> PairsIter<I> {
    let first_elem = iter.next();
    PairsIter {
        second_iter: iter.clone(),
        first_iter: iter,
        first_elem,
    }
}

impl<I: Iterator + Clone> Iterator for PairsIter<I>
where
    I::Item: Copy,
{
    type Item = (I::Item, I::Item);

    fn next(&mut self) -> Option<Self::Item> {
        while self.first_elem.is_some() {
            if let Some(second_elem) = self.second_iter.next() {
                return Some((self.first_elem.unwrap(), second_elem));
            }
            self.first_elem = self.first_iter.next();
            self.second_iter = self.first_iter.clone();
        }
        None
    }
}

pub trait PairsIterTrait {
    type Iter: Iterator;

    fn pairs(self) -> PairsIter<Self::Iter>;
}

impl<I: Iterator + Clone> PairsIterTrait for I {
    type Iter = I;

    fn pairs(self) -> PairsIter<Self::Iter> {
        new_pairs_iter(self)
    }
}
