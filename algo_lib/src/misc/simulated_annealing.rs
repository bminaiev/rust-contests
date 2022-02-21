use std::{cmp::max, time::Instant};

use crate::f;

use super::{num_traits::HasConstants, ord_f64::OrdF64, rand::Random};

pub enum SearchFor {
    MinimumScore,
    MaximumScore,
}

pub struct SimulatedAnnealing {
    rnd: Random,
    instant: Instant,
    max_time_millis: u128,
    search_for: SearchFor,
    start_temp: OrdF64,
    finish_temp: OrdF64,
    current_temperature: OrdF64,
    last_score: OrdF64,
    last_delta: OrdF64,
    last_printed_status_iter: usize,
    max_num_status_updates: usize,
    iterations_passed: usize,
}

impl SimulatedAnnealing {
    pub fn new(
        max_time_sec: f64,
        search_for: SearchFor,
        start_temp: f64,
        finish_temp: f64,
    ) -> Self {
        let last_score = match search_for {
            SearchFor::MinimumScore => OrdF64::MAX,
            SearchFor::MaximumScore => OrdF64::ZERO,
        };
        assert!(start_temp >= finish_temp);
        Self {
            rnd: Random::new(787788),
            instant: Instant::now(),
            max_time_millis: (max_time_sec * 1000.0) as u128,
            search_for,
            start_temp: f!(start_temp),
            finish_temp: f!(finish_temp),
            current_temperature: f!(start_temp),
            last_score,
            last_delta: f!(0.0),
            last_printed_status_iter: 0,
            max_num_status_updates: max(max_time_sec as usize, 10),
            iterations_passed: 0,
        }
    }

    pub fn elapsed_ms(&self) -> f64 {
        self.instant.elapsed().as_secs_f64() * 1000.0
    }

    fn print_status(&self) {
        let elapsed_ms = self.instant.elapsed().as_millis();
        eprintln!(
            "After {}ms ({} iters) score is: {}",
            elapsed_ms, self.iterations_passed, self.last_score
        );
    }

    pub fn should_continue(&mut self) -> bool {
        // TODO: do not call `elapsed` so often.
        let elapsed = self.instant.elapsed().as_millis();

        let part_time_elapsed =
            self.instant.elapsed().as_millis() as f64 / self.max_time_millis as f64;

        // when [part_time_elapsed] = 0.0, should be equal to [self.start_temp]
        // when [part_time_elapsed] = 1.0, should be equal to [self.finish_temp]
        self.current_temperature =
            self.start_temp * (self.finish_temp / self.start_temp).powf(part_time_elapsed);

        let status_iter = (part_time_elapsed * (self.max_num_status_updates as f64)) as usize;
        if status_iter != self.last_printed_status_iter {
            self.last_printed_status_iter = status_iter;
            self.print_status();
        }

        self.iterations_passed += 1;
        elapsed < self.max_time_millis
    }

    pub fn should_go<T>(&mut self, prev_score: T, new_score: T) -> bool
    where
        OrdF64: From<T>,
    {
        let prev_score: OrdF64 = prev_score.into();
        let new_score: OrdF64 = new_score.into();

        let delta_if_positive_is_good = {
            let delta: OrdF64 = new_score - prev_score;

            match self.search_for {
                SearchFor::MinimumScore => -delta,
                SearchFor::MaximumScore => delta,
            }
        };

        self.last_delta = delta_if_positive_is_good;
        if delta_if_positive_is_good >= f!(0.0) {
            self.last_score = new_score;
            return true;
        }

        let accept_probability =
            std::f64::consts::E.powf((delta_if_positive_is_good / self.current_temperature).0);
        assert!(accept_probability <= 1.0 + 1e-9);
        assert!(accept_probability >= 0.0);

        if self.rnd.gen_double() <= accept_probability {
            self.last_score = new_score;
            true
        } else {
            false
        }
    }

    /// Get the simulated annealing's current temperature.
    pub fn current_temperature(&self) -> f64 {
        self.current_temperature.0
    }

    /// Get the simulated annealing's last delta.
    pub fn last_delta(&self) -> f64 {
        self.last_delta.0
    }
}
