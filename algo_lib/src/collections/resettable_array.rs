#[derive(Clone)]
pub struct ResettableArray<T> {
    default: T,
    values: Vec<T>,
    values_time: Vec<i32>,
    current_time: i32,
}

impl<T: Clone> ResettableArray<T> {
    pub fn new(default: T, n: usize) -> Self {
        Self {
            default: default.clone(),
            values: vec![default; n],
            values_time: vec![0; n],
            current_time: 1,
        }
    }

    pub fn reset(&mut self) {
        self.current_time += 1;
        assert_ne!(self.current_time, std::i32::MAX);
    }

    pub fn get(&self, pos: usize) -> T {
        if self.current_time == self.values_time[pos] {
            return self.values[pos].clone();
        } else {
            return self.default.clone();
        }
    }

    pub fn set(&mut self, pos: usize, value: T) {
        self.values[pos] = value;
        self.values_time[pos] = self.current_time;
    }
}
