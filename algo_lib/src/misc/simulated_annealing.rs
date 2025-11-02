use std::{cmp::max, collections::VecDeque, time::Instant};

use crate::f;
use crate::misc::human_readable_usize::HumanReadableUsize;

use super::{num_traits::HasConstants, ord_f64::OrdF64, rand::Random};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SearchFor {
    MinimumScore,
    MaximumScore,
}

struct AcceptRate {
    accepted_on: VecDeque<usize>,
}

impl AcceptRate {
    pub fn new() -> Self {
        Self {
            accepted_on: VecDeque::default(),
        }
    }

    pub fn add(&mut self, iter: usize) {
        self.accepted_on.push_back(iter);
        if self.accepted_on.len() > 100 {
            self.accepted_on.pop_front();
        }
    }

    pub fn get_accept_percent(&self, iter: usize) -> f64 {
        if self.accepted_on.is_empty() {
            return 0.0;
        }
        let first = *self.accepted_on.front().unwrap();
        let cnt_iters = iter - first + 1;
        let accepted = self.accepted_on.len();
        (accepted as f64) / (cnt_iters as f64) * 100.0
    }
}

struct SaveChecker {
    saved_score: OrdF64,
    saved_at_ms: f64,
}

impl SaveChecker {
    pub fn new() -> Self {
        Self {
            saved_score: OrdF64::ZERO,
            saved_at_ms: 0.0,
        }
    }

    pub fn should_save(&mut self, score: OrdF64, search_for: SearchFor, time_ms: f64) -> bool {
        if time_ms < self.saved_at_ms + 10000.0 {
            return false;
        }

        match search_for {
            SearchFor::MaximumScore => {
                if score <= self.saved_score {
                    return false;
                }
            }
            SearchFor::MinimumScore => {
                if score >= self.saved_score {
                    return false;
                }
            }
        }

        eprintln!("Save score = {}", score);
        self.saved_at_ms = time_ms;
        self.saved_score = score;
        true
    }
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
    best_seen_score: OrdF64,
    last_delta: OrdF64,
    last_printed_status_iter: usize,
    max_num_status_updates: usize,
    iterations_passed: usize,
    silent: bool,
    accept_rate: AcceptRate,
    save_cheker: SaveChecker,
    out_prefix: String,
}

impl SimulatedAnnealing {
    ///
    /// Read:
    /// - https://apps.topcoder.com/forums/?module=Thread&threadID=696596&start=0
    /// - https://codeforces.com/blog/entry/94437
    ///
    pub fn new<T>(
        max_time_sec: f64,
        search_for: SearchFor,
        start_temp: f64,
        finish_temp: f64,
        start_score: T,
    ) -> Self
    where
        OrdF64: From<T>,
    {
        assert_ne!(start_temp, 0.0);
        assert_ne!(finish_temp, 0.0);
        let last_score: OrdF64 = start_score.into();
        assert!(start_temp >= finish_temp);
        let mut save_cheker = SaveChecker::new();

        save_cheker.saved_score = last_score;
        Self {
            rnd: Random::new(787788),
            instant: Instant::now(),
            max_time_millis: (max_time_sec * 1000.0) as u128,
            search_for,
            start_temp: f!(start_temp),
            finish_temp: f!(finish_temp),
            current_temperature: f!(start_temp),
            best_seen_score: last_score,
            last_score,
            last_delta: f!(0.0),
            last_printed_status_iter: 0,
            max_num_status_updates: max(max_time_sec as usize, 10),
            iterations_passed: 0,
            silent: false,
            accept_rate: AcceptRate::new(),
            save_cheker,
            out_prefix: String::new(),
        }
    }

    pub fn set_silent(&mut self, silent: bool) {
        self.silent = silent;
    }

    pub fn elapsed_ms(&self) -> f64 {
        self.instant.elapsed().as_secs_f64() * 1000.0
    }

    pub fn with_out_prefix(&mut self, prefix: String) {
        self.out_prefix = prefix;
    }

    fn print_status(&self) {
        let elapsed_ms = self.instant.elapsed().as_millis();
        eprintln!(
            "{}After {}ms ({:?} iters), % of accepted changes = {:.3}%, score is: {}, best: {}",
            self.out_prefix,
            elapsed_ms,
            HumanReadableUsize(self.iterations_passed),
            self.acceptance_percent(),
            self.last_score,
            self.best_seen_score,
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
        if status_iter != self.last_printed_status_iter && !self.silent {
            self.last_printed_status_iter = status_iter;
            self.print_status();
        }

        elapsed < self.max_time_millis
    }

    pub fn should_go<T>(&mut self, new_score: T) -> bool
    where
        OrdF64: From<T>,
    {
        self.iterations_passed += 1;

        let prev_score = self.last_score;
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
            match self.search_for {
                SearchFor::MaximumScore => {
                    if new_score > self.best_seen_score {
                        self.best_seen_score = new_score
                    }
                }
                SearchFor::MinimumScore => {
                    if new_score < self.best_seen_score {
                        self.best_seen_score = new_score;
                    }
                }
            }
            if delta_if_positive_is_good != f!(0.0) {
                self.accept_rate.add(self.iterations_passed);
            }
            return true;
        }

        let accept_probability =
            std::f64::consts::E.powf((delta_if_positive_is_good / self.current_temperature).0);
        assert!(accept_probability <= 1.0 + 1e-9);
        assert!(accept_probability >= 0.0);

        if self.rnd.gen_double() <= accept_probability {
            self.last_score = new_score;
            self.accept_rate.add(self.iterations_passed);
            true
        } else {
            false
        }
    }

    pub fn should_save(&mut self, last_time: bool) -> bool {
        let time = if last_time {
            f64::MAX
        } else {
            self.elapsed_ms()
        };
        self.save_cheker
            .should_save(self.last_score, self.search_for, time)
    }

    /// Get the simulated annealing's current temperature.
    pub fn current_temperature(&self) -> f64 {
        self.current_temperature.0
    }

    /// Get the simulated annealing's last delta.
    pub fn last_delta(&self) -> f64 {
        self.last_delta.0
    }

    pub fn last_score(&self) -> f64 {
        self.last_score.0
    }

    pub fn acceptance_percent(&self) -> f64 {
        self.accept_rate.get_accept_percent(self.iterations_passed)
    }
}
