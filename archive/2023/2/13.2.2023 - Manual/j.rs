//{"name":"j","group":"Manual","url":"","interactive":false,"timeLimit":2000,"tests":[{"input":"","output":""},{"input":"","output":""},{"input":"","output":""}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"j"}}}

use std::collections::VecDeque;

use algo_lib::io::output::output;
use algo_lib::io::task_io_settings::TaskIoType;
use algo_lib::io::task_runner::run_task;
use algo_lib::io::{input::Input, task_io_settings::TaskIoSettings};
use algo_lib::math::modulo::Mod_998_244_353;
use algo_lib::misc::rec_function::{Callable2, RecursiveFunction2};
#[allow(unused)]
use algo_lib::{dbg, out, out_line};

type Mod = Mod_998_244_353;

fn calc_hashes(n: usize, input: &mut Input) -> Vec<Mod> {
    let mut g = vec![vec![]; n];
    for _ in 0..n {
        let fr = input.usize() - 1;
        let to = input.usize() - 1;
        g[fr].push(to);
        g[to].push(fr);
    }
    let mut cnt = vec![0; n];
    for i in 0..n {
        cnt[i] = g[i].len();
    }
    let mut queue = VecDeque::new();
    for i in 0..n {
        if cnt[i] == 1 {
            queue.push_back(i);
        }
    }
    let mut on_cycle = vec![true; n];
    while let Some(v) = queue.pop_front() {
        on_cycle[v] = false;
        for &to in g[v].iter() {
            cnt[to] -= 1;
            if cnt[to] == 1 {
                queue.push_back(to);
            }
        }
    }
    let mut start_cycle = n;
    for v in 0..n {
        if on_cycle[v] {
            start_cycle = v;
        }
    }
    assert_ne!(start_cycle, n);
    let mut cycle = vec![start_cycle];
    let mut seen = vec![false; n];
    {
        let mut v = start_cycle;
        loop {
            seen[v] = true;
            let mut next = v;
            for &to in g[v].iter() {
                if seen[to] || !on_cycle[to] {
                    continue;
                }
                next = to;
            }
            if next == v {
                break;
            }
            cycle.push(next);
            v = next;
        }
    }
    let mut hashes = vec![];
    let mut calc_tree = RecursiveFunction2::new(|f, v: usize, p: usize| -> Mod {
        let mut child = vec![Mod::new(7877881)];
        for &to in g[v].iter() {
            if to == p || on_cycle[to] {
                continue;
            }
            child.push(f.call(to, v));
        }
        child.sort();
        let mut res = Mod::ZERO;
        for &to in child.iter() {
            res = res * Mod::new(239017) + to;
        }
        res * res * res
    });
    for &c in cycle.iter() {
        hashes.push(calc_tree.call(c, c));
    }
    let b1 = find_best(hashes.clone());
    hashes.reverse();
    let b2 = find_best(hashes);
    if b1 < b2 {
        b1
    } else {
        b2
    }
}

fn find_best(mut hashes: Vec<Mod>) -> Vec<Mod> {
    let sz = hashes.len();
    for i in 0..sz - 1 {
        hashes.push(hashes[i]);
    }
    let powers = Mod::gen_powers(Mod::new(239), hashes.len() + 2);
    let mut pref_hashes = vec![Mod::ZERO; hashes.len() + 1];
    for i in 0..hashes.len() {
        pref_hashes[i + 1] = pref_hashes[i] * powers[1] + hashes[i];
    }
    let mut best_start = 0;
    let mut best_hash = Mod::ZERO;
    for start in 0..sz {
        let cur_hash = pref_hashes[start + sz] - pref_hashes[start] * powers[sz];
        if cur_hash > best_hash {
            best_hash = cur_hash;
            best_start = start;
        }
    }
    let mut res = vec![];
    for &h in hashes[best_start..best_start + sz].iter() {
        res.push(h);
    }
    res
}

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.usize();
    let h1 = calc_hashes(n, input);
    let h2 = calc_hashes(n, input);
    if h1 == h2 {
        out_line!("Yes")
    } else {
        out_line!("No")
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
    tester::run_tests();
    // tester::run_single_test("1");
    // tester::run_stress(stress);
    // tester::run_locally();
    // tester::run_with_last_downloaded_file();
}
//END MAIN
