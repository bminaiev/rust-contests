//{"name":"f","group":"Manual","url":"","interactive":false,"timeLimit":2000,"tests":[{"input":"","output":""}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use std::vec;

use algo_lib::collections::fx_hash_map::FxHashSet;
#[allow(unused)]
use algo_lib::dbg;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::math::modulo::Mod_998_244_353;

type Mod = Mod_998_244_353;

fn solve(input: &mut Input, out: &mut Output) {
    let tc = input.usize();
    for _ in 0..tc {
        let n = input.usize();
        let k = input.usize();
        let mut used = FxHashSet::default();
        let mut by_row = vec![vec![]; 2];
        let mut a = vec![];
        for _ in 0..k {
            let r = input.usize() - 1;
            let c = input.usize() - 1;
            used.insert((r, c));
            a.push((r, c));
            by_row[r].push(c);
        }
        by_row[0].sort();
        by_row[1].sort();
        let mut res = Mod::ZERO;
        for mask in 0..(1 << 4) {
            let m0 = (mask & 1) != 0;
            let m1 = (mask & 2) != 0;
            let m2 = (mask & 4) != 0;
            let m3 = (mask & 8) != 0;
            if (m0 || m1) && by_row[0].is_empty() {
                continue;
            }
            if (m2 || m3) && by_row[1].is_empty() {
                continue;
            }
            let mut ways = Mod::ONE;
            for &(r, c) in a.iter() {
                let mut can = true;
                if used.contains(&(1 - r, c)) {
                    can = false;
                } else if r == 1 {
                    if m0 && c <= by_row[0][0] {
                        can = false;
                    }
                    if m1 && c >= by_row[0].last().copied().unwrap() {
                        can = false;
                    }
                } else {
                    if m2 && c <= by_row[1][0] {
                        can = false;
                    }
                    if m3 && c >= by_row[1].last().copied().unwrap() {
                        can = false;
                    }
                }
                if can {
                    ways = ways + ways;
                }
            }
            res = res + ways;
        }
        for _ in 0..k {
            res = res + res;
        }
        out.println(res);
    }
}

pub(crate) fn run(mut input: Input, mut output: Output) -> bool {
    solve(&mut input, &mut output);
    output.flush();
    true
}

#[cfg(feature = "local")]
fn main() {
    const PROBLEM_NAME: &str = "f";
    use algo_lib::tester::helper::*;

    run_tests(PROBLEM_NAME, run);
    // run_single_test(PROBLEM_NAME, run, "1");
    // run_stress(stress);
    // run_locally(run);
}
//END MAIN

#[cfg(not(feature = "local"))]
fn main() {
    let input = algo_lib::io::input::Input::new_stdin();
    let mut output = algo_lib::io::output::Output::new_stdout();
    run(input, output);
}
