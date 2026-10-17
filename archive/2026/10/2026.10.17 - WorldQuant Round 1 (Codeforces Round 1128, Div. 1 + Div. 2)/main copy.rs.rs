use std::collections::{HashMap, HashSet};

#[allow(unused)]
use algo_lib::dbg;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::rand::Random;
use algo_lib::misc::rec_function::{Callable, RecursiveFunction};

fn solve(input: &mut Input, out: &mut Output) {}

pub(crate) fn run(mut input: Input, mut output: Output) -> bool {
    solve(&mut input, &mut output);
    output.flush();
    true
}

#[derive(Clone, Copy, Hash, PartialEq, Eq, Debug)]
struct State {
    n: usize,
    m: usize,
    r: usize,
    c: usize,
    cnt_zeroes: usize,
}

fn stress() {
    let n = 20;
    // UDLR
    // [x0, y0, x1, y1]
    // const CHANGES: [[i32; 4]; 4] = [[1, 0, 0, 0], [0, 1, 0, 0], [0, 0, -1, 0], [0, 0, 0, -1]];
    for it in 1..2 {
        dbg!(it);
        let mut rnd = Random::new(it);
        let mut by_s = vec![0; n * n + 1];
        for i in 0..n * n + 1 {
            by_s[i] = i % 4;
        }
        let mut ok_states = HashMap::new();
        for cnt_zeroes in 0..2usize {
            let s = State {
                n: 1,
                m: 1,
                r: 0,
                c: 0,
                cnt_zeroes,
            };
            ok_states.insert(s, true);
        }
        let mut rec = RecursiveFunction::new(|f, state: State| -> bool {
            if let Some(v) = ok_states.get(&state) {
                return *v;
            }
            if state.cnt_zeroes > state.n * state.m {
                return false;
            }
            let mut can_win = false;
            let ones = state.n * state.m - state.cnt_zeroes;
            let dir = by_s[ones];
            if dir == 0 {
                if state.r > 0 {
                    for cnt_zeroes_here in 0..=state.cnt_zeroes.min(state.m) {
                        let nstate = State {
                            n: state.n - 1,
                            m: state.m,
                            r: state.r - 1,
                            c: state.c,
                            cnt_zeroes: state.cnt_zeroes - cnt_zeroes_here,
                        };
                        if f.call(nstate) {
                            can_win = true;
                            break;
                        }
                    }
                }
            } else if dir == 1 {
                if state.r + 1 < state.n {
                    for cnt_zeroes_here in 0..=state.cnt_zeroes.min(state.m) {
                        let nstate = State {
                            n: state.n - 1,
                            m: state.m,
                            r: state.r,
                            c: state.c,
                            cnt_zeroes: state.cnt_zeroes - cnt_zeroes_here,
                        };
                        if f.call(nstate) {
                            can_win = true;
                            break;
                        }
                    }
                }
            } else if dir == 2 {
                if state.c > 0 {
                    for cnt_zeroes_here in 0..=state.cnt_zeroes.min(state.n) {
                        let nstate = State {
                            n: state.n,
                            m: state.m - 1,
                            r: state.r,
                            c: state.c - 1,
                            cnt_zeroes: state.cnt_zeroes - cnt_zeroes_here,
                        };
                        if f.call(nstate) {
                            can_win = true;
                            break;
                        }
                    }
                }
            } else if dir == 3 {
                if state.c + 1 < state.m {
                    for cnt_zeroes_here in 0..=state.cnt_zeroes.min(state.n) {
                        let nstate = State {
                            n: state.n,
                            m: state.m - 1,
                            r: state.r,
                            c: state.c,
                            cnt_zeroes: state.cnt_zeroes - cnt_zeroes_here,
                        };
                        if f.call(nstate) {
                            can_win = true;
                            break;
                        }
                    }
                }
            }
            ok_states.insert(state, can_win);
            can_win
        });
        for r in 0..n {
            for c in 0..n {
                let mut found_zeros = false;
                for cnt_zeroes in 0..=n * n {
                    let s = State {
                        n,
                        m: n,
                        r,
                        c,
                        cnt_zeroes,
                    };
                    if rec.call(s) {
                        found_zeros = true;
                        dbg!(r, c, cnt_zeroes);
                        assert!(cnt_zeroes <= 20);
                        break;
                    }
                }
                assert!(found_zeros);
            }
        }
    }
}

#[cfg(feature = "local")]
fn main() {
    const PROBLEM_NAME: &str = "f1_ii_";
    #[allow(unused)]
    use algo_lib::misc::dragon::run_dragon;
    use algo_lib::tester::helper::*;

    // run_tests(PROBLEM_NAME, run);
    // run_single_test(PROBLEM_NAME, run, "1");
    run_stress(stress);
    // run_locally(run);
    // run_dragon(solve, "W", 1..2);
}
//END MAIN

#[cfg(not(feature = "local"))]
fn main() {
    let input = algo_lib::io::input::Input::new_stdin();
    let mut output = algo_lib::io::output::Output::new_stdout();
    run(input, output);
}
