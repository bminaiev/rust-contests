use std::time::Instant;

pub struct ExpIter {
    start_t: f64,
    end_t: f64,
    max_time_s: f64,
}

impl ExpIter {
    pub fn new(temperature: std::ops::Range<f64>, time_s: f64) -> Self {
        Self {
            start_t: temperature.start,
            end_t: temperature.end,
            max_time_s: time_s,
        }
    }
}

impl IntoIterator for ExpIter {
    type Item = f64;

    type IntoIter = ExpIterIterator;

    fn into_iter(self) -> Self::IntoIter {
        ExpIterIterator {
            start_t: self.start_t,
            end_t: self.end_t,
            max_time_s: self.max_time_s,
            instant: Instant::now(),
        }
    }
}

pub struct ExpIterIterator {
    start_t: f64,
    end_t: f64,
    max_time_s: f64,
    instant: Instant,
}

impl Iterator for ExpIterIterator {
    type Item = f64;

    fn next(&mut self) -> Option<Self::Item> {
        let elapsed = self.instant.elapsed().as_secs_f64();
        if elapsed > self.max_time_s {
            return None;
        }
        let part_time_elapsed = elapsed / self.max_time_s;
        let t = self.start_t * (self.end_t / self.start_t).powf(part_time_elapsed);
        Some(t)
    }
}
