//{"name":"E1. Задача-шутка (простая версия)","group":"Codeforces - Codeforces Global Round 23","url":"https://codeforces.com/contest/1746/problem/E1","interactive":true,"timeLimit":1000,"tests":[{"input":"6\n\nNO\n\n:(\n\nNO\n\n:)\n","output":"? 5 1 2 5 4 3\n\n! 6\n\n? 4 1 2 3 4\n\n! 5\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"E1ZadachaShutkaProstayaVersiya"}}}

use std::collections::HashMap;

use algo_lib::collections::fx_hash_map::FxHashMap;
use algo_lib::io::output::output;
use algo_lib::io::task_io_settings::TaskIoType;
use algo_lib::io::task_runner::run_task;
use algo_lib::io::{input::Input, task_io_settings::TaskIoSettings};
use algo_lib::misc::min_max::UpdateMinMax;
#[allow(unused)]
use algo_lib::{dbg, out, out_line};

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.usize();

    let mut hm = FxHashMap::default();
    gen(20, &mut hm);
    gen(n, &mut hm);
    let mut vals = vec![vec![]; 3];
    for x in 1..=n {
        vals[0].push(x);
    }
    loop {
        let s = vals[0].len() + vals[1].len() + vals[2].len();
        if s <= 2 {
            let mut all = vec![];
            for v in vals.iter() {
                for x in v.iter() {
                    all.push(*x);
                }
            }
            for &x in all.iter() {
                out_line!("!", x);
                output().flush();
                if input.string_as_string() == ":)" {
                    return;
                }
            }
            assert!(false);
        }
        let r = hm
            .get(&State {
                c: [vals[0].len(), vals[1].len(), vals[2].len()],
            })
            .unwrap()
            .how;
        let mut query = vec![];
        for i in 0..3 {
            for sz in 0..r[i] {
                query.push(vals[i][sz]);
            }
        }
        out_line!("?", query.len(), query);
        output().flush();
        let mut nvals = vec![vec![]; 3];
        if input.string_as_string() == "YES" {
            for i in 0..r[2] {
                nvals[0].push(vals[2][i]);
            }
            for i in 0..r[0] {
                nvals[1].push(vals[0][i]);
            }
            for i in 0..r[1] {
                nvals[1].push(vals[1][i]);
            }
            for i in r[0]..vals[0].len() {
                nvals[2].push(vals[0][i]);
            }
            for i in r[1]..vals[1].len() {
                nvals[2].push(vals[1][i]);
            }
        } else {
            for i in r[2]..vals[2].len() {
                nvals[0].push(vals[2][i]);
            }
            for i in r[0]..vals[0].len() {
                nvals[1].push(vals[0][i]);
            }
            for i in r[1]..vals[1].len() {
                nvals[1].push(vals[1][i]);
            }
            for i in 0..r[0] {
                nvals[2].push(vals[0][i]);
            }
            for i in 0..r[1] {
                nvals[2].push(vals[1][i]);
            }
        }
        vals = nvals;
    }
}

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
struct State {
    c: [usize; 3],
}

#[derive(Clone, Copy)]
struct Res {
    val: usize,
    how: [usize; 3],
}

fn calc(s: State, hm: &mut FxHashMap<State, Res>) -> usize {
    if s.c.iter().sum::<usize>() <= 2 {
        return 0;
    }
    if let Some(r) = hm.get(&s) {
        return r.val;
    }
    let mut res = Res {
        val: 100500,
        how: [0, 0, 0],
    };
    hm.insert(s, res);
    let mut st = [s.c[0] / 2, s.c[1] / 2, s.c[2] / 2];
    // for i in 0..3 {
    //     if st[i] > 0 {
    //         st[i] -= 1;
    //     }
    // }
    for a0 in 0..2 {
        for a1 in 0..2 {
            for a2 in 0..2 {
                let ask = [st[0] + a0, st[1] + a1, st[2] + a2];
                if ask[0] > s.c[0] || ask[1] > s.c[1] || ask[2] > s.c[2] {
                    continue;
                }
                let mut worst = 0;
                {
                    let next = [ask[2], ask[0] + ask[1], s.c[0] + s.c[1] - ask[0] - ask[1]];
                    let r1 = calc(State { c: next }, hm);
                    worst.update_max(r1);
                }
                {
                    let nask = [s.c[0] - ask[0], s.c[1] - ask[1], s.c[2] - ask[2]];
                    let next = [
                        nask[2],
                        nask[0] + nask[1],
                        s.c[0] + s.c[1] - nask[0] - nask[1],
                    ];
                    let r2 = calc(State { c: next }, hm);
                    worst.update_max(r2);
                }
                if worst + 1 < res.val {
                    res.val = worst + 1;
                    res.how = ask;
                }
                // res.update_min(worst + 1);
            }
        }
    }
    // for i in 0..3 {
    //     if s.c[i] > 0 && s.queries > 1 {
    //         let mut c = s.c.clone();
    //         c[i] -= 1;
    //         let next = State {
    //             c,
    //             queries: s.queries - 1,
    //         };
    //         res.update_min(calc(next, hm));
    //     }
    // }

    hm.insert(s, res);
    // hm.remove(&s);
    res.val
}

fn stress() {
    for n in 100_000..100_001 {
        dbg!(n);
        gen(n, &mut FxHashMap::default());
    }
}

fn gen(n: usize, hm: &mut FxHashMap<State, Res>) {
    let mut c = [n, 0, 0];
    let start = State { c };
    let res = calc(start.clone(), hm);
    loop {
        let mut changed = false;
        let mut keys: Vec<_> = hm.keys().cloned().collect();
        keys.sort();
        for k in keys.iter() {
            let cur = *hm.get(k).unwrap();
            hm.remove(k);
            let nval = calc(k.clone(), hm);
            if nval != cur.val {
                changed = true;
            }
        }
        if !changed {
            break;
        }
    }
    let res = calc(start.clone(), hm);
}

pub(crate) fn run(mut input: Input) -> bool {
    solve(&mut input, 1);
    output().flush();
    true
    // input.skip_whitespace();
    // input.peek().is_none()
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
    // tester::run_stress(stress);
    // tester::run_single_test("1");
    tester::run_stress(stress);
    // tester::run_locally();
    // tester::run_with_last_downloaded_file();
}
//END MAIN
