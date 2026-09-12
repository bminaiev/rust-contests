use std::collections::{BTreeSet, HashMap};

use algo_lib::collections::array_2d::Array2D;
use algo_lib::collections::permutation::Permutation;
#[allow(unused)]
use algo_lib::dbg;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::math::gcd::lcm;
use algo_lib::misc::rand::Random;
use algo_lib::misc::rec_function::{Callable, RecursiveFunction};
use algo_lib::misc::simulated_annealing::SimulatedAnnealing;

fn solve_local(a: &mut [usize]) {
    let n = a.len();
    loop {
        let mut ch = false;
        for i in 0..n - 1 {
            let mut prev = 0;
            let mut next = 0;
            if i > 0 {
                prev += lcm(a[i - 1], a[i]);
                next += lcm(a[i - 1], a[i + 1]);
            }
            if i + 2 < n {
                prev += lcm(a[i + 1], a[i + 2]);
                next += lcm(a[i], a[i + 2]);
            }
            if next > prev {
                ch = true;
                a.swap(i, i + 1);
            }
        }
        if !ch {
            break;
        }
    }
}

fn cost(a: &[usize]) -> usize {
    let n = a.len();
    let mut res = 0;
    for i in 0..n - 1 {
        res += lcm(a[i], a[i + 1]);
    }
    res
}

fn cost_cycle(a: &[usize]) -> usize {
    let n = a.len();
    let mut res = 0;
    for i in 0..n {
        res += lcm(a[i], a[(i + 1) % n]);
    }
    res
}

const MAX_V: usize = 7;

fn solve_smarter(a: &[usize]) {
    let mut start_state = vec![0; MAX_V + 1];
    for i in 0..a.len() {
        start_state[a[i]] += 1;
    }
    let mut all = vec![];
    for mask in 1..(1 << MAX_V) {
        let mut vals = vec![];
        for i in 0..MAX_V {
            if (mask >> i) & 1 == 1 {
                vals.push(i + 1);
            }
        }
        let mut perm = Permutation::new(vals.len());
        loop {
            let mut real_vals = vec![];
            for i in 0..vals.len() {
                real_vals.push(vals[perm[i]]);
            }
            all.push(real_vals);
            if !perm.next() {
                break;
            }
        }
    }
    dbg!(all.len());
    let mut hm = HashMap::<Vec<usize>, usize>::new();
    RecursiveFunction::new(|f, state: Vec<usize>| -> usize {
        if let Some(&res) = hm.get(&state) {
            return res;
        }
        let mut res = 0;
        for perm in &all {
            let mut new_state = state.clone();
            let mut ok = true;
            for &v in perm {
                if new_state[v] == 0 {
                    ok = false;
                    break;
                }
                new_state[v] -= 1;
            }
            if !ok {
                continue;
            }
            res = res.max(f.call(new_state) + cost_cycle(perm));
        }
        hm.insert(state, res);
        res
    })
    .call(start_state.clone());

    let mut useful_perms = BTreeSet::new();
    let mut cur_state = start_state.clone();
    loop {
        let mut ch = false;
        let val = hm[&cur_state];
        for perm in &all {
            let mut new_state = cur_state.clone();
            let mut ok = true;
            for &v in perm {
                if new_state[v] == 0 {
                    ok = false;
                    break;
                }
                new_state[v] -= 1;
            }
            if !ok {
                continue;
            }
            if val == hm[&new_state] + cost_cycle(perm) {
                useful_perms.insert(perm.clone());
                ch = true;
                cur_state = new_state;
                break;
            }
        }
        if !ch {
            break;
        }
    }
    for perm in useful_perms {
        dbg!(perm);
    }
}

struct State {
    a: Array2D<usize>,
}

impl State {
    fn new(a: &[usize]) -> Self {
        let mut res = Array2D::new(0, MAX_V + 1, MAX_V + 1);
        for i in 0..a.len() - 1 {
            let min = a[i].min(a[i + 1]);
            let max = a[i].max(a[i + 1]);
            res[min][max] += 1;
        }
        Self { a: res }
    }

    fn remove_edge(&mut self, i: usize, j: usize) {
        let min = i.min(j);
        let max = i.max(j);
        assert!(self.a[min][max] > 0);
        self.a[min][max] -= 1;
    }

    fn add_edge(&mut self, i: usize, j: usize) {
        let min = i.min(j);
        let max = i.max(j);
        self.a[min][max] += 1;
    }
}

fn solve_smarter2(a: &[usize]) {
    let mut state = State::new(a);
    loop {
        let mut ch = false;
        for i in 1..=MAX_V {
            for j in i..=MAX_V {
                for k in 1..=MAX_V {
                    for l in k..=MAX_V {
                        // replace (i, j), (k, l) with (i, l), (k, j)
                        if state.a[i][j] > 0 && state.a[k][l] > 0 {
                            let prev = lcm(i, j) + lcm(k, l);
                            let next = lcm(i, k) + lcm(j, l);
                            if next > prev {
                                ch = true;
                                state.remove_edge(i, j);
                                state.remove_edge(k, l);
                                state.add_edge(i, k);
                                state.add_edge(j, l);
                            }
                        }
                    }
                }
            }
        }
        if !ch {
            break;
        }
    }
    for i in 1..=MAX_V {
        for j in 1..=MAX_V {
            if state.a[i][j] > 0 {
                dbg!(i, j, state.a[i][j]);
            }
        }
    }
}

fn solve_smarter3(a: &[usize]) {
    let mut state = State::new(a);
    let mut start_score = cost(a) as f64;
    let mut sa = SimulatedAnnealing::new(
        1.0,
        algo_lib::misc::simulated_annealing::SearchFor::MaximumScore,
        10.0,
        1e-1,
        start_score,
    );
    while sa.should_continue() {
        let mut ch = false;
        for i in 1..=MAX_V {
            for j in i..=MAX_V {
                for k in 1..=MAX_V {
                    for l in k..=MAX_V {
                        // replace (i, j), (k, l) with (i, l), (k, j)
                        if state.a[i][j] > 0 && state.a[k][l] > 0 {
                            if i == k && j == l {
                                continue;
                            }
                            let prev = lcm(i, j) + lcm(k, l);
                            let next = lcm(i, k) + lcm(j, l);
                            let new_score = start_score - prev as f64 + next as f64;
                            if sa.should_go(new_score) {
                                ch = true;
                                start_score = new_score;
                                state.remove_edge(i, j);
                                state.remove_edge(k, l);
                                state.add_edge(i, k);
                                state.add_edge(j, l);
                            }
                        }
                    }
                }
            }
        }
        if !ch {
            break;
        }
    }
    for i in 1..=MAX_V {
        for j in 1..=MAX_V {
            if state.a[i][j] > 0 {
                dbg!(i, j, state.a[i][j]);
            }
        }
    }
}

fn stress() {
    for it in 1..20 {
        dbg!(it);
        let mut n = 10000;
        let mut rnd = Random::new(787788 + it);
        let mut a = rnd.gen_vec(n, 1..MAX_V + 1);
        let mut probs = vec![0.0; MAX_V + 1];
        for i in 1..=MAX_V {
            probs[i] = rnd.gen_double();
        }
        let sum: f64 = probs.iter().sum();
        for i in 1..=MAX_V {
            probs[i] /= sum;
        }
        for i in 0..n {
            let mut val = rnd.gen_double();
            let mut v = 1;
            while v <= MAX_V && val > probs[v] {
                val -= probs[v];
                v += 1;
            }
            a[i] = v;
        }
        // solve_smarter2(&mut a);
        rnd.shuffle(&mut a);
        dbg!("!!");
        solve_smarter3(&mut a);
        // rnd.shuffle(&mut a);
        // dbg!("!!");
        // solve_smarter3(&mut a);
        // dbg!(&a);
    }
}

fn solve(input: &mut Input, out: &mut Output) {
    let tc = input.usize();
    for _ in 0..tc {
        let n = input.usize();
        let mut a = input.vec::<usize>(n);
        solve_local(&mut a);
        out.println(cost(&a));
    }
}

pub(crate) fn run(mut input: Input, mut output: Output) -> bool {
    solve(&mut input, &mut output);
    output.flush();
    true
}

#[cfg(feature = "local")]
fn main() {
    const PROBLEM_NAME: &str = "c";
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
