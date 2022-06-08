//{"name":"Win As Second","group":"Google Coding Competitions - Round 3 2022 - Code Jam 2022","url":"https://codingcompetitions.withgoogle.com/codejam/round/00000000008779b4/0000000000b4518a","interactive":false,"timeLimit":60000,"tests":[],"testType":"multiNumber","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"WinAsSecond"}}}

use std::collections::{HashMap, HashSet};

use algo_lib::graph::dsu::Dsu;
use algo_lib::io::output::output;
use algo_lib::io::task_io_settings::TaskIoType;
use algo_lib::io::task_runner::run_task;
use algo_lib::io::{input::Input, task_io_settings::TaskIoSettings};
use algo_lib::misc::rand::Random;
use algo_lib::misc::rec_function::{Callable, Callable2, RecursiveFunction, RecursiveFunction2};
use algo_lib::misc::vec_apply_delta::ApplyDelta;
#[allow(unused)]
use algo_lib::{dbg, out, out_line};

fn can_win(g: &[Vec<usize>]) -> bool {
    let n = g.len();
    let mut can_win = HashMap::<i64, usize>::default();
    let start = (1 << n) - 1;

    let mut rec = RecursiveFunction::new(|f, alive: i64| -> usize {
        if alive == 0 {
            return 0;
        }
        if let Some(r) = can_win.get(&(alive)) {
            return *r;
        }

        let mut dsu = Dsu::new(n);
        for v in 0..n {
            if ((1 << v) & alive) != 0 {
                for &to in g[v].iter() {
                    if ((1 << to) & alive) != 0 {
                        dsu.unite(v, to);
                    }
                }
            }
        }
        let mut by_comp = vec![vec![]; n];
        for v in 0..n {
            if ((1 << v) & alive) != 0 {
                by_comp[dsu.get(v)].push(v);
            }
        }
        let mut xor = 0;
        let mut any = false;
        for v in 0..n {
            if by_comp[v].len() != 0 {
                let mut next_mask = 0;
                for x in by_comp[v].iter() {
                    next_mask |= 1 << *x;
                }
                if next_mask != alive {
                    any = true;
                    xor ^= f.call(next_mask);
                }
            }
        }
        if any {
            can_win.insert((alive), xor);
            return xor;
        }

        let mut seen = HashSet::new();
        for v in 0..n {
            if ((1 << v) & alive) != 0 {
                let mut alive_near = vec![];
                for &to in g[v].iter() {
                    if ((1 << to) & alive) != 0 {
                        alive_near.push(to);
                    }
                }
                for mask in 0i32..(1 << alive_near.len()) {
                    let mut next_alive = alive ^ (1 << v);
                    for i in 0..alive_near.len() {
                        if ((1 << i) & mask) != 0 {
                            next_alive ^= 1 << alive_near[i];
                        }
                    }
                    let next = f.call(next_alive);
                    seen.insert(next);
                }
            }
        }

        let mut res = 0;
        while seen.contains(&res) {
            res += 1;
        }
        can_win.insert((alive), res);
        res
    });

    rec.call(start) != 0
}

fn stress() {
    let n = 31;
    for seed in 16.. {
        dbg!(seed);
        let mut rnd = Random::new(seed);
        let mut prev = vec![0; n];
        for i in 1..n {
            const S: usize = 2;
            let from = if i > S { i - S } else { 0 };
            prev[i] = rnd.gen(from..i);
        }
        let mut g = vec![vec![]; n];
        for i in 1..n {
            let p = prev[i];
            g[i].push(p);
            g[p].push(i);
        }
        let can_win = can_win(&g);
        if !can_win {
            dbg!("wow");
            break;
        } else {
            dbg!("oops...");
        }
    }
}

fn solve(input: &mut Input, test_case: usize) {
    let n = input.usize();
    let seed = 16;
    let mut rnd = Random::new(seed);
    let mut prev = vec![0; n];
    for i in 1..n {
        const S: usize = 2;
        let from = if i > S { i - S } else { 0 };
        prev[i] = rnd.gen(from..i);
    }
    let mut g = vec![vec![]; n];
    for i in 1..n {
        let p = prev[i];
        g[i].push(p);
        g[p].push(i);
    }
    for i in 0..n {
        for &to in g[i].iter() {
            if to > i {
                out_line!(i + 1, to + 1);
            }
        }
    }
    output().flush();

    let n = g.len();
    let mut can_win = HashMap::<i64, usize>::default();
    let start = (1 << n) - 1;

    let mut rec = RecursiveFunction::new(|f, alive: i64| -> usize {
        if alive == 0 {
            return 0;
        }
        if let Some(r) = can_win.get(&(alive)) {
            return *r;
        }

        let mut dsu = Dsu::new(n);
        for v in 0..n {
            if ((1 << v) & alive) != 0 {
                for &to in g[v].iter() {
                    if ((1 << to) & alive) != 0 {
                        dsu.unite(v, to);
                    }
                }
            }
        }
        let mut by_comp = vec![vec![]; n];
        for v in 0..n {
            if ((1 << v) & alive) != 0 {
                by_comp[dsu.get(v)].push(v);
            }
        }
        let mut xor = 0;
        let mut any = false;
        for v in 0..n {
            if by_comp[v].len() != 0 {
                let mut next_mask = 0;
                for x in by_comp[v].iter() {
                    next_mask |= 1 << *x;
                }
                if next_mask != alive {
                    any = true;
                    xor ^= f.call(next_mask);
                }
            }
        }
        if any {
            can_win.insert((alive), xor);
            return xor;
        }

        let mut seen = HashSet::new();
        for v in 0..n {
            if ((1 << v) & alive) != 0 {
                let mut alive_near = vec![];
                for &to in g[v].iter() {
                    if ((1 << to) & alive) != 0 {
                        alive_near.push(to);
                    }
                }
                for mask in 0i32..(1 << alive_near.len()) {
                    let mut next_alive = alive ^ (1 << v);
                    for i in 0..alive_near.len() {
                        if ((1 << i) & mask) != 0 {
                            next_alive ^= 1 << alive_near[i];
                        }
                    }
                    let next = f.call(next_alive);
                    seen.insert(next);
                }
            }
        }

        let mut res = 0;
        while seen.contains(&res) {
            res += 1;
        }
        can_win.insert((alive), res);
        res
    });

    let n_games = input.usize();
    for _ in 0..n_games {
        let mut alive = (1 << n) - 1;
        loop {
            let cnt = input.usize();
            let rem = input.vec::<usize>(cnt).sub_from_all(1);
            for &v in rem.iter() {
                alive ^= 1 << v;
            }

            assert_ne!(alive, 0);

            let mut found = false;

            for v in 0..n {
                if ((1 << v) & alive) != 0 {
                    if found {
                        break;
                    }
                    let mut alive_near = vec![];
                    for &to in g[v].iter() {
                        if ((1 << to) & alive) != 0 {
                            alive_near.push(to);
                        }
                    }
                    for mask in 0i32..(1 << alive_near.len()) {
                        let mut next_alive = alive ^ (1 << v);
                        let mut used = vec![];
                        for i in 0..alive_near.len() {
                            if ((1 << i) & mask) != 0 {
                                next_alive ^= 1 << alive_near[i];
                                used.push(1 + alive_near[i]);
                            }
                        }
                        if rec.call(next_alive) == 0 {
                            out_line!(1 + mask.count_ones());
                            out_line!(v + 1, used);
                            output().flush();
                            alive = next_alive;
                            found = true;
                            break;
                        }
                    }
                }
            }
            assert!(found);
            if alive == 0 {
                break;
            }
        }
    }
}

pub(crate) fn run(mut input: Input) -> bool {
    let t = input.read();
    for i in 0usize..t {
        solve(&mut input, i + 1);
    }
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
    tester::run_stress(stress);
    // tester::run_single_test("1");
}
//END MAIN
