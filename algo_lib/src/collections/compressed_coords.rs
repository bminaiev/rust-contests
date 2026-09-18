pub struct CompressedCoords<T: Ord> {
    values: Vec<T>,
}

impl<T: Ord + Clone> CompressedCoords<T> {
    pub fn new(values: &[T]) -> Self {
        let mut values = values.to_vec();
        values.sort();
        values.dedup();
        Self { values }
    }

    pub fn size(&self) -> usize {
        self.values.len()
    }

    pub fn get(&self, x: T) -> usize {
        self.values.binary_search(&x).unwrap()
    }
}
