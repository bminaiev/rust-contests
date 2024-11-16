//{"name":"j","group":"Manual","url":"","interactive":false,"timeLimit":2000,"tests":[{"input":"","output":""}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"j"}}}

use std::collections::BTreeSet;

#[allow(unused)]
use algo_lib::dbg;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;

#[derive(Debug, Clone, Copy)]
struct Charger {
    pos: i64,
    id: usize,
}

fn solve(input: &mut Input, out: &mut Output, _test_case: usize) {
    let tc = input.usize();
    for _ in 0..tc {
        let n = input.usize();
        let m = input.usize();
        let a: Vec<i64> = input.vec(n);
        let mut chargers = vec![];
        let mut by_id = vec![vec![]; n];
        for _i in 0..m {
            let pos = input.i64();
            let id = input.usize() - 1;
            chargers.push(Charger { pos, id });
            by_id[id].push(pos);
        }
        for i in 0..n {
            by_id[i].push(i64::MAX);
            by_id[i].reverse();
        }
        let mut to_use = BTreeSet::new();
        for i in 0..n {
            to_use.insert((*by_id[i].last().unwrap(), i));
        }
        let mut cur_alive = a.clone();
        let mut cur_pos = 0;
        let mut it = 0;
        loop {
            if to_use.is_empty() {
                break;
            }
            if it == m {
                for i in 0..n {
                    cur_pos += cur_alive[i];
                }
                break;
            }
            let next_pos = chargers[it].pos;
            while cur_pos < next_pos {
                if to_use.is_empty() {
                    break;
                }
                let (zz, id) = *to_use.iter().next().unwrap();
                let can_use = cur_alive[id];
                let use_here = can_use.min(next_pos - cur_pos);
                cur_pos += use_here;
                cur_alive[id] -= use_here;
                if cur_alive[id] == 0 {
                    to_use.remove(&(zz, id));
                }
            }
            if cur_pos == next_pos {
                let id = chargers[it].id;
                cur_alive[id] = a[id];
                by_id[id].pop();
                to_use.remove(&(next_pos, id));
                let next_pos = by_id[id].last().unwrap();
                to_use.insert((*next_pos, id));
            } else {
                break;
            }
            it += 1;
        }
        out.println(cur_pos);
    }
}

pub(crate) fn run(mut input: Input, mut output: Output) -> bool {
    solve(&mut input, &mut output, 1);
    output.flush();
    true
}

//START MAIN

fn main() {
    const PROBLEM_NAME: &str = "j";
    use algo_lib::tester::helper::*;

    run_tests(PROBLEM_NAME, run);
    // run_single_test(PROBLEM_NAME, run, "1");
    // run_stress(stress);
    // run_locally(run);
}
//END MAIN
