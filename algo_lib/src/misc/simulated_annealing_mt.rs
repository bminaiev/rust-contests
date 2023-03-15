use std::{
    fmt::Display,
    ops::{AddAssign, Range},
    sync::{Arc, Mutex},
    thread,
    time::Instant,
};

use crate::{
    collections::array_2d::Array2D,
    f,
    geometry::point::PointT,
    misc::{
        gen_vector::gen_vec, num_traits::HasConstants, ord_f64::OrdF64, rand::Random,
        simulated_annealing::SearchFor,
    },
};

pub trait SaState: Send + Clone {
    type Change;
    type Score: Into<OrdF64> + Display + AddAssign + Copy + Send;

    fn apply(&mut self, change: &Self::Change);
    fn change_score_delta(&self, change: &Self::Change) -> Self::Score;
}

pub fn should_go<T: Into<OrdF64>>(
    score_delta: T,
    search_for: SearchFor,
    current_temperature: OrdF64,
    rnd: &mut Random,
) -> bool {
    let delta: OrdF64 = score_delta.into();

    let delta_if_positive_is_good = match search_for {
        SearchFor::MinimumScore => -delta,
        SearchFor::MaximumScore => delta,
    };

    if delta_if_positive_is_good >= f!(0.0) {
        return true;
    }

    let accept_probability =
        std::f64::consts::E.powf((delta_if_positive_is_good / current_temperature).0);
    assert!(accept_probability <= 1.0 + 1e-9);
    assert!(accept_probability >= 0.0);

    rnd.gen_double() <= accept_probability
}

pub fn simulated_annealing_mt<State: SaState>(
    max_time_sec: f64,
    start_state: State,
    start_score: State::Score,
    gen_change: impl FnMut(&State, &mut Random) -> Option<State::Change> + Send + Clone,
    searching_for: SearchFor,
    start_temp: f64,
    finish_temp: f64,
) {
    let num_threads = 20;

    let start_temp = f!(start_temp);
    let finish_temp = f!(finish_temp);
    let start = Instant::now();

    const UPDATE_EVERY: f64 = 0.01;

    let best_solution = Arc::new(Mutex::new(OrdF64::MAX));

    thread::scope(|scope| {
        for thread_id in 0..num_threads {
            let best_solution_clone = Arc::clone(&best_solution);

            let mut state = start_state.clone();
            let mut gen_change = gen_change.clone();

            scope.spawn(move || {
                let mut cur_score = start_score;
                let mut rnd = Random::new(787788 + thread_id);
                let mut last_updated = 0.0;

                loop {
                    let part_time_elapsed = start.elapsed().as_secs_f64() / max_time_sec;
                    if part_time_elapsed >= 1.0 {
                        break;
                    }

                    if part_time_elapsed > last_updated + UPDATE_EVERY {
                        let mut best_solution_guard = best_solution_clone.lock().unwrap();
                        let cur_score = cur_score.into();
                        if cur_score < *best_solution_guard {
                            *best_solution_guard = cur_score;
                        }
                        if thread_id == 0 {
                            eprintln!(
                                "Time(s):\t{}\tCurrent score:\t{}",
                                start.elapsed().as_secs_f64(),
                                *best_solution_guard
                            );
                        }

                        last_updated = part_time_elapsed;
                    }
                    // when [part_time_elapsed] = 0.0, should be equal to [self.start_temp]
                    // when [part_time_elapsed] = 1.0, should be equal to [self.finish_temp]
                    let current_temperature =
                        start_temp * (finish_temp / start_temp).powf(part_time_elapsed);

                    if let Some(change) = gen_change(&state, &mut rnd) {
                        let score_delta = State::change_score_delta(&state, &change);
                        if should_go(score_delta, searching_for, current_temperature, &mut rnd) {
                            cur_score += score_delta;
                            state.apply(&change);
                        }
                    }
                }
            });
        }
    });

    eprintln!("Done!")
}

// TODO: move to a separate file
#[test]
fn test() {
    eprintln!("Hello!");
    let mut rnd = Random::new(787788);
    let n = 500;
    type Point = PointT<OrdF64>;
    let pts = gen_vec(n, |_| {
        Point::new(f!(rnd.gen_double()), f!(rnd.gen_double()))
    });
    let d = Array2D::new_f(n, n, |id1, id2| pts[id1].dist2(&pts[id2]));

    #[derive(Clone)]
    struct State {
        perm: Vec<usize>,
    }

    let calc_score = |state: &State| -> OrdF64 {
        let mut res = OrdF64::ZERO;
        for i in 0..state.perm.len() {
            let x = state.perm[i];
            let y = state.perm[(i + 1) % state.perm.len()];
            res += d[x][y];
        }
        res
    };

    struct Change {
        rev: Range<usize>,
        score_delta: OrdF64,
    }

    impl SaState for State {
        type Change = Change;

        type Score = OrdF64;

        fn apply(&mut self, change: &Self::Change) {
            self.perm[change.rev.clone()].reverse()
        }

        fn change_score_delta(&self, change: &Self::Change) -> Self::Score {
            change.score_delta
        }
    }

    let perm = rnd.gen_permutation(n);
    let start_state = State { perm };
    let start_score = calc_score(&start_state);

    simulated_annealing_mt(
        10.0,
        start_state,
        start_score,
        |state: &State, rnd: &mut Random| {
            let rev = rnd.gen_nonempty_range(n);
            let mut new_state = state.clone();
            new_state.perm[rev.clone()].reverse();
            let score_delta = calc_score(&new_state) - calc_score(state);
            Some(Change { rev, score_delta })
        },
        SearchFor::MinimumScore,
        1.0,
        0.0001,
    );
}
