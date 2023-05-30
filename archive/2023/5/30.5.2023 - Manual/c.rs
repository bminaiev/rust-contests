//{"name":"c","group":"Manual","url":"","interactive":false,"timeLimit":2000,"tests":[{"input":"","output":""},{"input":"","output":""}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"c"}}}

use std::cmp::min;

use algo_lib::io::output::output;
use algo_lib::io::task_io_settings::TaskIoType;
use algo_lib::io::task_runner::run_task;
use algo_lib::io::{input::Input, task_io_settings::TaskIoSettings};
use algo_lib::math::modulo::Mod_998_244_353;
#[allow(unused)]
use algo_lib::{dbg, out, out_line};

type Mod = Mod_998_244_353;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct Object {
    pos: i32,
    value: i32,
}

fn solve_simple(mut pos: &mut [Object], mut neg: &mut [Object]) -> Mod {
    assert!(pos.len() > 0);
    if neg.is_empty() {
        return Mod::new(pos[0].pos);
    }
    let cnt_pos = pos[0].value;
    let cnt_neg = neg[0].value;
    let mut res = Mod::ZERO;
    let cnt_same = min(cnt_pos, cnt_neg);
    res += Mod::new(cnt_same) * (Mod::new(pos[0].pos) + Mod::new(neg[0].pos)) * Mod::TWO;
    pos[0].value -= cnt_same;
    neg[0].value -= cnt_same;
    if pos[0].value == 0 && pos.len() == 1 {
        res -= Mod::new(neg[0].pos);
        return res;
    }
    if pos[0].value == 0 {
        pos = &mut pos[1..];
    }
    if neg[0].value == 0 {
        neg = &mut neg[1..];
    }
    res + solve_simple(pos, neg)
}

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.usize();
    let mut positive = vec![];
    let mut negative = vec![];
    for _ in 0..n {
        let pos = input.i32();
        let value = input.i32();
        if pos > 0 {
            positive.push(Object { pos, value });
        } else {
            negative.push(Object { pos: -pos, value });
        }
    }
    positive.sort();
    negative.sort();
    let res = solve_simple(&mut positive, &mut negative);
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
