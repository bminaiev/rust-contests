//{"name":"c","group":"Manual","url":"","interactive":false,"timeLimit":2000,"tests":[{"input":"","output":""},{"input":"","output":""}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"c"}}}

use algo_lib::collections::fx_hash_map::FxHashMap;
use algo_lib::collections::index_of::IndexOf;
use algo_lib::io::output::output;
use algo_lib::io::task_io_settings::TaskIoType;
use algo_lib::io::task_runner::run_task;
use algo_lib::io::{input::Input, task_io_settings::TaskIoSettings};
#[allow(unused)]
use algo_lib::{dbg, out, out_line};

#[derive(Hash, Eq, PartialEq, Clone, Debug, PartialOrd, Ord)]
struct State {
    hits: Vec<i32>,
    alive: Vec<i32>,
    more_moves: i32,
}

impl State {
    fn ways(&self) -> f64 {
        let mut res = 1.0;
        let mut it = 0;
        for (h_id, &h) in self.hits.iter().enumerate() {
            while it != self.alive.len() && self.alive[it] > h {
                it += 1;
            }
            res *= it as f64 - h_id as f64;
        }
        res
    }

    fn new(mut hits: Vec<i32>, mut alive: Vec<i32>, more_moves: i32) -> Self {
        hits.sort();
        hits.reverse();
        alive.sort();
        alive.reverse();
        Self {
            hits,
            alive,
            more_moves,
        }
    }
}

fn calc_rec(hm: &mut FxHashMap<State, f64>, s: State) -> f64 {
    if s.more_moves == 0 || s.ways() == 0.0 {
        return 0.0;
    }
    if let Some(&res) = hm.get(&s) {
        return res;
    }
    let mut res = 0.0;
    for pos in 0..s.hits.len() {
        if pos > 0 && s.hits[pos] == s.hits[pos - 1] {
            continue;
        }
        let mut new_hits = s.hits.clone();
        new_hits[pos] += 1;
        let new_hits_val = new_hits[pos];
        let base_state = State::new(new_hits, s.alive.clone(), s.more_moves - 1);
        let base_res = calc_rec(hm, base_state.clone());
        let cur_res = if let Some(index_to_remove) = s.alive.index_of(&new_hits_val) {
            let mut mult = 1;
            while index_to_remove + mult < s.alive.len()
                && s.alive[index_to_remove + mult] == new_hits_val
            {
                mult += 1;
            }
            let mut new_alive = s.alive.clone();
            new_alive.remove(index_to_remove);
            let mut new_hits: Vec<i32> = s.hits.clone();
            new_hits.remove(pos);
            let new_state = State::new(new_hits, new_alive, s.more_moves - 1);

            let new_ways = new_state.ways() * (mult as f64);
            let base_ways = base_state.ways();
            let new_res = calc_rec(hm, new_state) + 1.0;
            (new_ways * new_res + base_ways * base_res) / (new_ways + base_ways)
        } else {
            base_res
        };
        if cur_res > res {
            res = cur_res;
        }
    }
    hm.insert(s, res);
    res
}

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.usize();
    let moves = input.i32();
    let a = input.vec::<i32>(n);
    let s = State::new(vec![0; n], a, moves);
    let mut hm = FxHashMap::default();
    let res = calc_rec(&mut hm, s);
    out_line!(res);
}

fn stress() {
    for n in 1..=10 {
        let mut a = vec![10; n];
        for i in 0..n {
            a[i] = (i + 1) as i32;
        }
        a = vec![6, 6, 7, 7, 8, 8, 9, 9, 10, 10];
        let s = State::new(vec![0; n], a.clone(), a.iter().sum());
        let mut hm = FxHashMap::default();
        let res = calc_rec(&mut hm, s);
        dbg!(n, hm.len(), res);
        // let mut all_keys: Vec<_> = hm.keys().collect();
        // all_keys.sort();
        // for key in all_keys.iter() {
        //     dbg!(key);
        // }
    }
}

pub(crate) fn run(mut input: Input) -> bool {
    solve(&mut input, 1);
    output().flush();
    true
}

#[allow(unused)]
pub fn submit() -> bool {
    let io = TaskIoSettings {
        is_interactive: false,
        input: TaskIoType::Std,
        output: TaskIoType::Std,
    };

    run_task(io, run)
}

//START MAIN
mod tester;

fn main() {
    // tester::run_tests();
    // tester::run_single_test("3");
    tester::run_stress(stress);
    // tester::run_locally();
    // tester::run_with_last_downloaded_file();
}
//END MAIN
