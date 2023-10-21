//{"name":"D. Лексихроматография","group":"Codeforces - Codeforces Round 902 (Div. 1, based on COMPFEST 15 - Final Round)","url":"https://codeforces.com/contest/1876/problem/D","interactive":false,"timeLimit":2000,"tests":[{"input":"8\n1 3 1 2 3 2 3 3\n","output":"3\n"},{"input":"1\n265\n","output":"1\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"DLeksikhromatografiya"}}}

use std::collections::BTreeMap;

use algo_lib::graph::dsu::Dsu;
use algo_lib::io::output::output;
use algo_lib::io::task_io_settings::TaskIoType;
use algo_lib::io::task_runner::run_task;
use algo_lib::io::{input::Input, task_io_settings::TaskIoSettings};
use algo_lib::math::modulo::Mod_998_244_353;
use algo_lib::misc::rand::Random;
use algo_lib::misc::vec_apply_delta::ApplyDelta;
#[allow(unused)]
use algo_lib::{dbg, out, out_line};

type Mod = Mod_998_244_353;

#[target_feature(enable = "avx2")]
unsafe fn solve_fast(vals: &[usize]) -> Mod {
    let mx = vals.iter().max().unwrap() + 1;
    // let mut positions = vec![vec![]; mx];
    // for i in 0..vals.len() {
    //     positions[vals[i]].push(i);
    // }
    // for v in positions.iter_mut() {
    //     v.push(vals.len());
    // }
    let mut next = vec![2; mx];
    let mut a = vec![vec![]; 2];
    let mut ways_total = Mod::ONE;
    let mut ways_eq = Mod::ONE;
    // let mut uneven = vec![false; vals.len()];
    let mut dsu = Dsu::new(mx);
    for &value in vals.iter() {
        if next[value] == 2 {
            ways_total *= Mod::TWO;
            if a[0].len() > a[1].len() {
                let prev = *a[0].last().unwrap();
                // dbg!(prev, value);
                dsu.unite(prev, value);
            }
            // let mut use_idx = if a[0].len() >= a[1].len() { 0 } else { 1 };
            // if a[0].len() == a[1].len() {
            //     use_idx = 2;
            //     for ch in positions[value].chunks_exact(2) {
            //         if uneven[ch[0]..ch[1]].iter().any(|&x| x) {
            //             use_idx = 0;
            //             break;
            //         }
            //         // if let Some(entry) = map.range(ch[0]..ch[1]).next() {
            //         //     use_idx = *entry.1;
            //         //     break;
            //         // }
            //     }
            //     dbg!(value, use_idx);
            //     if use_idx == 2 {
            //         ways_eq *= Mod::TWO;
            //         use_idx = 0;
            //     }
            // }
            let use_idx = 0;
            a[use_idx].push(value);
            next[value] = use_idx ^ 1;
            // let mut cur_idx = use_idx;
            // let mut itt = 0;
            // for c in positions[value].chunks_exact(2) {
            //     for x in c[0]..c[1] {
            //         uneven[x] = true;
            //     }
            // }
            // for &p in positions[value].iter() {
            //     if p != vals.len() && itt % 2 == 0 {
            //         map.insert(p, cur_idx);
            //     }
            //     itt += 1;
            //     cur_idx ^= 1;
            // }
        } else {
            if next[value] == 0 && a[0].len() > a[1].len() {
                let last = *a[0].last().unwrap();
                dsu.unite(last, value);
            }
            a[next[value]].push(value);
            let pos = a[next[value]].len() - 1;
            if pos < a[0].len() && pos < a[1].len() {
                if a[0][pos] != a[1][pos] {
                    ways_eq = Mod::ZERO;
                }
            }
            next[value] ^= 1;
        }
    }
    let mut exist = vec![false; mx];
    for &v in vals.iter() {
        exist[v] = true;
    }
    for v in 0..mx {
        if exist[v] && dsu.get(v) == v {
            ways_eq *= Mod::TWO;
        }
    }
    // dbg!(a[0]);
    // dbg!(a[1]);
    // dbg!(ways_eq);
    if a[0].len() != a[1].len() {
        ways_eq = Mod::ZERO;
    }
    let res = (ways_total - ways_eq) / Mod::TWO;
    res
}

fn solve_slow(vals: &[usize]) -> Mod {
    let mut res = Mod::ZERO;
    let mx = vals.iter().max().unwrap() + 1;
    for mask in 0..(1 << vals.len()) {
        let mut a = vec![];
        let mut b = vec![];
        for i in 0..vals.len() {
            if (mask & (1 << i)) != 0 {
                a.push(vals[i]);
            } else {
                b.push(vals[i]);
            }
        }
        if a >= b {
            continue;
        }
        let mut ok = true;
        for fr in 0..vals.len() {
            for to in fr + 1..=vals.len() {
                let mut cnt = vec![0i32; mx];
                for i in fr..to {
                    if (mask & (1 << i)) != 0 {
                        cnt[vals[i]] += 1;
                    } else {
                        cnt[vals[i]] -= 1;
                    }
                }
                for x in cnt.iter() {
                    if x.abs() >= 2 {
                        ok = false;
                    }
                }
            }
        }
        if ok {
            // dbg!(a, b);
            res += Mod::ONE;
        }
    }
    res
}

fn stress() {
    for it in 1831.. {
        dbg!(it);
        let mut rnd = Random::new(it);
        let n = rnd.gen(1..15);
        let a = rnd.gen_vec(n, 0..5);
        let fast = unsafe { solve_fast(&a) };
        let slow = solve_slow(&a);
        if fast != slow {
            dbg!(n, a, fast, slow);
            panic!();
        }
    }
}

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.usize();
    let vals = input.vec::<usize>(n).sub_from_all(1);
    let res = unsafe { solve_fast(&vals) };
    out_line!(res);
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
