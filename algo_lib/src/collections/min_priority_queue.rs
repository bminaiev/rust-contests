use std::cmp::Reverse;
use std::collections::BinaryHeap;

pub struct MinPriorityQueue<T>(BinaryHeap<Reverse<T>>)
where
    T: Ord;

impl<T> MinPriorityQueue<T>
where
    T: Ord,
{
    pub fn new() -> Self {
        Self(BinaryHeap::new())
    }

    pub fn with_capacity(n: usize) -> Self {
        Self(BinaryHeap::with_capacity(n))
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub fn peek(&self) -> Option<&T> {
        match self.0.peek() {
            None => None,
            Some(elem) => Some(&elem.0),
        }
    }

    pub fn push(&mut self, elem: T) {
        self.0.push(Reverse(elem))
    }

    pub fn pop(&mut self) -> Option<T> {
        match self.0.pop() {
            None => None,
            Some(elem) => Some(elem.0),
        }
    }
}
