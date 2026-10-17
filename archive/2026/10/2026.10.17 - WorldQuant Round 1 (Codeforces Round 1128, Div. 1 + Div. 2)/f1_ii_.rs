use std::collections::{HashMap, HashSet};

use algo_lib::collections::array_2d::Array2D;
#[allow(unused)]
use algo_lib::dbg;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::rand::Random;
use algo_lib::misc::rec_function::{Callable, RecursiveFunction};
use algo_lib::misc::simulated_annealing::SearchFor::MinimumScore;
use algo_lib::misc::simulated_annealing::SimulatedAnnealing;

fn solve(input: &mut Input, out: &mut Output) {
    let mut by_s = [
        0, 0, 1, 2, 3, 2, 1, 3, 0, 3, 2, 2, 3, 0, 2, 1, 1, 3, 1, 0, 1, 3, 3, 2, 0, 3, 0, 1, 2, 3,
        1, 3, 2, 2, 0, 3, 3, 0, 2, 3, 1, 3, 1, 2, 2, 0, 3, 1, 2, 3, 0, 0, 3, 0, 1, 3, 3, 1, 0, 1,
        3, 1, 2, 3, 0, 3, 1, 3, 0, 1, 2, 1, 1, 2, 2, 3, 0, 0, 2, 2, 3, 3, 2, 0, 0, 0, 1, 0, 2, 2,
        1, 3, 3, 0, 2, 0, 3, 0, 1, 1, 0,
    ]
    .to_vec();
    let n = input.usize();
    if n == 3 {
        by_s = [2, 2, 3, 1, 0, 3, 1, 0, 2, 2].to_vec()
    }
    let sol = gen_solution(n, by_s.to_vec(), 3).unwrap();
    // dbg!(sol.cnt_bad());
    // dbg!(sol.max_zeros());
    for r in 0..n {
        for c in 0..n {
            for i in 0..n {
                for j in 0..n {
                    out.print(sol.fields[r][c].as_ref().unwrap()[i][j]);
                }
                out.println("");
            }
            out.println("");
        }
    }
    let str = b"UDLR";
    for &dir in sol.by_s[..n * n + 1].iter() {
        let ch = str[dir];
        out.print(ch as char);
    }
    out.println("");
}

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

struct Solution {
    n: usize,
    fields: Array2D<Option<Array2D<usize>>>,
    by_s: Vec<usize>,
}

impl Solution {
    fn max_zeros(&self) -> usize {
        let mut res = 0;
        for r in 0..self.n {
            for c in 0..self.n {
                let mut cnt = 0;
                for i in 0..self.n {
                    for j in 0..self.n {
                        if self.fields[r][c].as_ref().unwrap()[i][j] == 0 {
                            cnt += 1;
                        }
                    }
                }
                dbg!(r, c, cnt);
                res = res.max(cnt);
            }
        }
        res
    }

    fn cnt_bad(&self) -> usize {
        let mut res = 0;
        for r in 0..self.n {
            for c in 0..self.n {
                if self.fields[r][c].is_none() {
                    let mut score = 1;
                    if r == 0 || r == self.n - 1 {
                        if c == 0 || c == self.n - 1 {
                            score *= 1000;
                        }
                    }
                    res += score;
                }
            }
        }
        res
    }

    fn get_path(&self, r: usize, c: usize) -> Vec<[usize; 3]> {
        let mut path = vec![];
        let field = self.fields[r][c].as_ref().unwrap();
        let mut r1 = 0;
        let mut r2 = self.n;
        let mut c1 = 0;
        let mut c2 = self.n;
        while r2 - r1 > 1 || c2 - c1 > 1 {
            let mut cnt_ones = 0;
            for i in r1..r2 {
                for j in c1..c2 {
                    if field[i][j] == 1 {
                        cnt_ones += 1;
                    }
                }
            }
            let dir = self.by_s[cnt_ones];
            let cnt_zeros = (r2 - r1) * (c2 - c1) - cnt_ones;
            path.push([(r2 - r1), (c2 - c1), cnt_zeros]);
            if dir == 0 {
                r1 += 1;
            } else if dir == 1 {
                r2 -= 1;
            } else if dir == 2 {
                c1 += 1;
            } else if dir == 3 {
                c2 -= 1;
            }
        }
        assert!(r1 == r && c1 == c);
        path
    }
}

fn gen_solution(n: usize, by_s: Vec<usize>, allowed_zeros: usize) -> Option<Solution> {
    // let mut by_s = vec![0; n * n + 1];
    // let mut rnd = Random::new(it);
    // for i in 0..n * n + 1 {
    //     by_s[i] = rnd.gen_range(0..4);
    // }
    let by_s_clone = by_s.clone();
    let mut ok_states = HashMap::new();
    for cnt_zeroes in 0..2usize {
        let s = State {
            n: 1,
            m: 1,
            r: 0,
            c: 0,
            cnt_zeroes,
        };
        let res = Array2D::new(1 - cnt_zeroes, 1, 1);
        ok_states.insert(s, Some(res));
    }
    let mut rec = RecursiveFunction::new(|f, state: State| -> Option<Array2D<usize>> {
        if let Some(v) = ok_states.get(&state) {
            return v.clone();
        }
        if state.cnt_zeroes > state.n * state.m {
            return None;
        }
        let mut can_win = false;
        let ones = state.n * state.m - state.cnt_zeroes;
        let dir = by_s[ones];
        let mut res = Array2D::new(0, state.n, state.m);
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
                    if let Some(v) = f.call(nstate) {
                        for i in 0..state.n - 1 {
                            for j in 0..state.m {
                                res[i + 1][j] = v[i][j];
                            }
                        }
                        for j in 0..(state.m - cnt_zeroes_here) {
                            res[0][j] = 1;
                        }
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
                    if let Some(v) = f.call(nstate) {
                        for i in 0..state.n - 1 {
                            for j in 0..state.m {
                                res[i][j] = v[i][j];
                            }
                        }
                        for j in 0..(state.m - cnt_zeroes_here) {
                            res[state.n - 1][j] = 1;
                        }
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
                    if let Some(v) = f.call(nstate) {
                        for i in 0..state.n {
                            for j in 0..state.m - 1 {
                                res[i][j + 1] = v[i][j];
                            }
                        }
                        for i in 0..(state.n - cnt_zeroes_here) {
                            res[i][0] = 1;
                        }
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
                    if let Some(v) = f.call(nstate) {
                        for i in 0..state.n {
                            for j in 0..state.m - 1 {
                                res[i][j] = v[i][j];
                            }
                        }
                        for i in 0..(state.n - cnt_zeroes_here) {
                            res[i][state.m - 1] = 1;
                        }
                        can_win = true;
                        break;
                    }
                }
            }
        }
        if can_win {
            ok_states.insert(state, Some(res.clone()));
            Some(res)
        } else {
            ok_states.insert(state, None);
            None
        }
    });
    let mut sol = Solution {
        n,
        fields: Array2D::new(None, n, n),
        by_s: by_s_clone,
    };
    for r in 0..n {
        for c in 0..n {
            let mut found_zeros = false;
            for cnt_zeroes in 0..=allowed_zeros {
                let s = State {
                    n,
                    m: n,
                    r,
                    c,
                    cnt_zeroes,
                };
                if let Some(v) = rec.call(s) {
                    found_zeros = true;
                    // dbg!(r, c, cnt_zeroes);
                    sol.fields[r][c] = Some(v);
                    // assert!(cnt_zeroes <= 20);
                    break;
                }
            }
            // if !found_zeros {
            //     return None;
            // }
            // assert!(found_zeros);
        }
    }
    Some(sol)
}

fn stress() {
    let mut a = vec![0, 2, 3, 4];
    for n in 1..10 {
        dbg!(n * n);
        dbg!(n * (n + 1));
    }
}

fn stress123() {
    // let n = 20;
    // UDLR
    // [x0, y0, x1, y1]
    // const CHANGES: [[i32; 4]; 4] = [[1, 0, 0, 0], [0, 1, 0, 0], [0, 0, -1, 0], [0, 0, 0, -1]];

    for n in 11..12 {
        dbg!(n);
        let mut rnd = Random::new(123);
        let mut allowed_zeros = 3;
        let mut by_s = [
            0, 0, 1, 2, 3, 2, 1, 3, 0, 3, 2, 2, 3, 0, 2, 1, 1, 3, 1, 0, 1, 3, 3, 2, 0, 3, 0, 1, 2,
            3, 1, 3, 2, 2, 0, 3, 3, 0, 2, 3, 1, 3, 1, 2, 2, 0, 3, 1, 2, 3, 0, 0, 3, 0, 1, 3, 3, 1,
            0, 1, 3, 1, 2, 3, 0, 3, 1, 3, 0, 1, 2, 1, 1, 2, 2, 3, 0, 0, 2, 2, 3, 3, 2, 0, 0, 0, 1,
            0, 2, 2, 1, 3, 3, 0, 2, 0, 3, 0, 1, 1, 0,
        ]
        .to_vec();
        // let sol = gen_solution(n, by_s.clone(), allowed_zeros).unwrap();

        // for (r, c) in [(0, 0), (0, n - 1), (n - 1, 0), (n - 1, n - 1)] {
        //     let path = sol.get_path(r, c);
        //     dbg!(r, c);
        //     for chunk in path {
        //         let n = chunk[0];
        //         let m = chunk[1];
        //         let cnt_zeroes = chunk[2];
        //         let sum = n * m - cnt_zeroes;
        //         dbg!(n, m, cnt_zeroes, sum);
        //     }
        // }

        by_s.resize(n * n + 1, 0);
        for i in 101..by_s.len() {
            by_s[i] = rnd.gen_range(0..4);
        }
        let mut sa = SimulatedAnnealing::new(60.0, MinimumScore, 100.0, 1e-1, 100999);
        while sa.should_continue() {
            let idx = rnd.gen_range(101..by_s.len());
            let prev_val = by_s[idx];
            by_s[idx] = rnd.gen_range(0..4);
            if let Some(sol) = gen_solution(n, by_s.clone(), allowed_zeros) {
                let next_score = sol.cnt_bad();
                if next_score == 0 {
                    dbg!("FOUND!");
                    break;
                }
                // dbg!(n, sol.max_zeros());
                if !sa.should_go(next_score) {
                    by_s[idx] = prev_val;
                }
            } else {
                by_s[idx] = prev_val;
            }
        }
        dbg!(by_s);
    }
}

#[cfg(feature = "local")]
fn main() {
    const PROBLEM_NAME: &str = "f1_ii_";
    #[allow(unused)]
    use algo_lib::misc::dragon::run_dragon;
    use algo_lib::tester::helper::*;

    // run_tests(PROBLEM_NAME, run);
    // run_single_test(PROBLEM_NAME, run, "2");
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
