//{"name":"D - Non Arithmetic Progression Set","group":"AtCoder - AtCoder Regular Contest 145","url":"https://atcoder.jp/contests/arc145/tasks/arc145_d","interactive":false,"timeLimit":2000,"tests":[{"input":"3 9\n","output":"1 2 6\n"},{"input":"5 -15\n","output":"-15 -5 0 2 3\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"DNonArithmeticProgressionSet"}}}

use algo_lib::collections::bit_set::BitSet;
use algo_lib::collections::last_exn::LastExn;
use algo_lib::io::output::output;
use algo_lib::io::task_io_settings::TaskIoType;
use algo_lib::io::task_runner::run_task;
use algo_lib::io::{input::Input, task_io_settings::TaskIoSettings};
#[allow(unused)]
use algo_lib::{dbg, out, out_line};

fn gen(n: usize) -> Vec<i64> {
    const MAX_SIZE: usize = 10_000_000;
    let mut cant_use = BitSet::new(MAX_SIZE);
    let mut pts = vec![0];
    for it in 1..MAX_SIZE {
        if cant_use.get(it) {
            continue;
        }
        for prev in pts.iter() {
            let next = 2 * it - *prev;
            if next < MAX_SIZE {
                cant_use.set(next, true);
            }
        }
        pts.push(it);
        if pts.len() == n {
            break;
        }
    }
    pts.into_iter().map(|x| x as i64).collect()
}

fn solve(input: &mut Input) {
    let n = input.usize();
    let sum = input.i64();
    let mut res = gen(n);
    let cur_sum = res.iter().sum::<i64>();
    const LAST_ELEM_EXP: i64 = 5_000_000;
    let exp_sum = cur_sum - *res.last_exn() + LAST_ELEM_EXP;
    let shift = (sum - exp_sum) / (n as i64);
    for x in res.iter_mut() {
        *x += shift;
    }
    let cur_sum = res.iter().sum::<i64>();
    let delta = sum - cur_sum;
    res[n - 1] += delta;
    assert_eq!(sum, res.iter().sum());
    for i in 0..n {
        for j in i + 1..n - 1 {
            assert!(res[j] - res[i] != res[n - 1] - res[j]);
        }
    }
    out_line!(res);
}

fn stress() {
    let n = 10000;
    let r = gen(n);
    dbg!(r.len());
    dbg!(*r.last_exn());
}

pub(crate) fn run(mut input: Input) -> bool {
    solve(&mut input);
    output().flush();
    input.skip_whitespace();
    input.peek().is_none()
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
    // tester::run_stress(stress);
    // tester::run_single_test("1");
    // tester::run_stress(stress);
}
//END MAIN
