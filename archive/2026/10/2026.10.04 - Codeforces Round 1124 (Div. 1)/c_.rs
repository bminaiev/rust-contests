use std::ops::Range;

use algo_lib::collections::sparse_table_max::SparseTableMax;
#[allow(unused)]
use algo_lib::dbg;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::binary_search::binary_search_first_true;
use algo_lib::misc::rand::Random;

struct State {
    glob_iter: u32,
    positions: Vec<Vec<(u32, usize)>>,
}
const MX: usize = 1 << 18;

impl State {
    fn new() -> Self {
        Self {
            glob_iter: 0,
            positions: vec![vec![]; MX],
        }
    }

    fn incr_iter(&mut self) {
        self.glob_iter += 1;
    }

    fn add(&mut self, value: usize, pos: usize) {
        while !self.positions[value].is_empty()
            && self.positions[value].last().unwrap().0 < self.glob_iter
        {
            self.positions[value].pop();
        }
        self.positions[value].push((self.glob_iter, pos));
    }

    fn exist(&mut self, value: usize, r: Range<usize>) -> bool {
        while !self.positions[value].is_empty()
            && self.positions[value].last().unwrap().0 < self.glob_iter
        {
            self.positions[value].pop();
        }
        let idx = binary_search_first_true(0..self.positions[value].len(), |i| {
            self.positions[value][i].1 >= r.start
        });
        if idx < self.positions[value].len() && self.positions[value][idx].1 < r.end {
            return true;
        }
        false
    }
}

struct Solver {
    a: Vec<u32>,
    pref_xor: Vec<u32>,
    talbe: SparseTableMax<u32>,
    res: u32,
}

impl Solver {
    fn new(a: Vec<u32>) -> Self {
        let n = a.len();
        let mut pref_xor = vec![0; n + 1];
        for i in 0..n {
            pref_xor[i + 1] = pref_xor[i] ^ a[i];
        }
        let talbe = SparseTableMax::new(&a);
        Self {
            a,
            pref_xor,
            talbe,
            res: 0,
        }
    }

    fn round(&mut self, rnd: &mut Random) {
        let n = self.a.len();
        for _ in 0..n {
            let l = rnd.gen_range(0..n);
            let r = rnd.gen_range(l + 1..n + 1);
            let len = r - l;
            if len < 2 {
                continue;
            }
            let mx_pos = self.talbe.find_max_pos(l..r);
            let mx = self.a[mx_pos];
            if mx < self.res {
                continue;
            }
            let xor = self.pref_xor[r] ^ self.pref_xor[l];
            let cur_res = xor & mx;
            self.res = self.res.max(cur_res);
        }
    }

    fn solve_real(&mut self, state: &mut State) {
        let n = self.a.len();
        for bit in (0..20).rev() {
            let check_ans = self.res | (1 << bit);
            let mut found = false;

            state.incr_iter();
            for i in 0..=self.a.len() {
                let cur = self.pref_xor[i] & check_ans;
                state.add(cur as usize, i);
            }
            if self.find(state, check_ans, 0..n) {
                found = true;
            }
            if found {
                self.res = check_ans;
            }
        }
    }

    fn find(&mut self, state: &mut State, check_ans: u32, range: Range<usize>) -> bool {
        if range.len() < 2 {
            return false;
        }
        let mx_pos = self.talbe.find_max_pos(range.clone());
        let mx = self.a[mx_pos];
        if (mx & check_ans) == check_ans {
            let left_len = mx_pos - range.start;
            let right_len = range.end - mx_pos - 1;
            if left_len < right_len {
                for start in range.start..=mx_pos {
                    let my_prefix_xor = self.pref_xor[start] & check_ans;
                    let need = (my_prefix_xor ^ check_ans) as usize;
                    let mut need_pos_at_least = mx_pos + 1;
                    if start == mx_pos {
                        need_pos_at_least += 1;
                    }
                    if state.exist(need, need_pos_at_least..range.end + 1) {
                        return true;
                    }
                }
            } else {
                for end in mx_pos..range.end {
                    let my_prefix_xor = self.pref_xor[end + 1] & check_ans;
                    let need = (my_prefix_xor ^ check_ans) as usize;
                    let mut need_pos_at_most = mx_pos + 1;
                    if end == mx_pos {
                        need_pos_at_most -= 1;
                    }
                    if state.exist(need, range.start..need_pos_at_most) {
                        return true;
                    }
                }
            }
        }
        if self.find(state, check_ans, range.start..mx_pos) {
            return true;
        }
        if self.find(state, check_ans, mx_pos + 1..range.end) {
            return true;
        }
        false
    }
}

fn solve(input: &mut Input, out: &mut Output) {
    let tc = input.usize();
    let mut solvers = vec![];
    for _ in 0..tc {
        let n = input.usize();
        let a = input.vec::<u32>(n);
        solvers.push(Solver::new(a));
    }
    // let mut start = Instant::now();
    // let mut rnd = Random::new(787788);
    // while start.elapsed().as_secs_f32() < 1.5 {
    //     for solver in solvers.iter_mut() {
    //         solver.round(&mut rnd);
    //     }
    // }
    let mut state = State::new();
    for solver in solvers.iter_mut() {
        solver.solve_real(&mut state);
        out.println(solver.res);
    }
}

fn stress() {
    for it in 1.. {
        dbg!(it);
        let mut rnd = Random::new(it);
        let n = rnd.gen_range(1..10);
        
    }
}

pub(crate) fn run(mut input: Input, mut output: Output) -> bool {
    solve(&mut input, &mut output);
    output.flush();
    true
}

#[cfg(feature = "local")]
fn main() {
    const PROBLEM_NAME: &str = "c_";
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
