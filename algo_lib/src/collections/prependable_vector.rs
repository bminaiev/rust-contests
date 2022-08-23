use std::ops;

pub struct PrependableVector<T> {
    data: Vec<T>,
    start: usize,
}

impl<T: Default + Clone> PrependableVector<T> {
    pub fn new(size: usize) -> Self {
        let mut data = Vec::with_capacity(size * 2);
        data.resize(size, T::default());
        Self { start: size, data }
    }

    pub fn insert(&mut self, pos: usize, value: T) {
        if pos * 2 < self.data.len() && self.start > 0 && self.start != self.data.len() {
            unsafe {
                std::ptr::copy(&self.data[self.start], &mut self.data[self.start - 1], pos);
            }
            self.start -= 1;
            self.data[self.start + pos] = value;
            return;
        }
        self.data.insert(self.start + pos, value);
    }

    pub fn push(&mut self, value: T) {
        self.data.push(value)
    }
}

impl<T> ops::Deref for PrependableVector<T> {
    type Target = [T];

    #[inline]
    fn deref(&self) -> &[T] {
        &self.data[self.start..]
    }
}

impl<T> ops::DerefMut for PrependableVector<T> {
    #[inline]
    fn deref_mut(&mut self) -> &mut [T] {
        &mut self.data[self.start..]
    }
}
