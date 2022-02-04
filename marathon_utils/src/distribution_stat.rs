use std::cmp::min;
use std::fmt::{Debug, Display, Formatter};

pub struct DistributionStat<T: Ord + Clone> {
    vals: Vec<T>,
}

impl<T: Ord + Clone> DistributionStat<T> {
    pub fn new() -> Self {
        Self { vals: vec![] }
    }

    pub fn add(&mut self, value: T) {
        self.vals.push(value);
    }
}

impl<T: Ord + Clone + Display> Debug for DistributionStat<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut values = self.vals.clone();
        values.sort();
        f.write_fmt(format_args!("[cnt = {}]\n", values.len()))?;
        if !values.is_empty() {
            for proc in (0..=100).step_by(10) {
                let pos = min(values.len() - 1, proc * values.len() / 100);
                f.write_fmt(format_args!("{}% -> {}\n", proc, values[pos]))?;
            }
        }
        Ok(())
    }
}
