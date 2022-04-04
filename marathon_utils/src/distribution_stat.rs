use std::cmp::min;
use std::fmt::{Debug, Display, Formatter};

use algo_lib::collections::sorted::SortedTrait;

#[derive(Clone)]
pub struct DistributionStat<T: Ord + Clone> {
    pub name: String,
    vals: Vec<T>,
}

impl<T: Ord + Clone> DistributionStat<T> {
    pub fn new(name: &str) -> Self {
        Self {
            vals: vec![],
            name: name.to_owned(),
        }
    }

    pub fn add(&mut self, value: T) {
        self.vals.push(value);
    }

    pub fn data(&self) -> &[T] {
        &self.vals
    }

    pub fn f64_data(&self) -> Vec<f64>
    where
        f64: From<T>,
    {
        self.vals.iter().map(|x| x.clone().into()).collect()
    }

    pub fn to_text_format(&self) -> String
    where
        T: Display,
    {
        let data = self.vals.sorted();
        if data.is_empty() {
            return "[]".to_owned();
        }
        let get_percentile = |percent: usize| -> T {
            let pos = percent * data.len() / 100;
            let pos = min(pos, data.len() - 1);
            data[pos].clone()
        };
        format!(
            "[min = {}; 25% = {}; 50% = {}, 75% = {}, max = {}]",
            get_percentile(0),
            get_percentile(25),
            get_percentile(50),
            get_percentile(75),
            get_percentile(100)
        )
    }
}

impl<T: Ord + Clone + Display> Debug for DistributionStat<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut values = self.vals.clone();
        values.sort();
        f.write_fmt(format_args!(
            "[name = {}, cnt = {}]\n",
            self.name,
            values.len()
        ))?;
        if !values.is_empty() {
            for proc in (0..=100).step_by(10) {
                let pos = min(values.len() - 1, proc * values.len() / 100);
                f.write_fmt(format_args!("{}% -> {}\n", proc, values[pos]))?;
            }
        }
        Ok(())
    }
}
