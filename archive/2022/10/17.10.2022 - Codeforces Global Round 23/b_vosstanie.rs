//{"name":"B. Восстание","group":"Codeforces - Codeforces Global Round 23","url":"https://codeforces.com/contest/1746/problem/B","interactive":false,"timeLimit":1000,"tests":[{"input":"4\n8\n0 0 1 1 1 1 1 1\n5\n1 0 0 1 1\n2\n1 0\n11\n1 1 0 0 1 0 0 1 1 1 0\n","output":"0\n1\n1\n3\n"}],"testType":"multiNumber","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"BVosstanie"}}}

use std::cmp::max;

use algo_lib::io::output::output;
use algo_lib::io::task_io_settings::TaskIoType;
use algo_lib::io::task_runner::run_task;
use algo_lib::io::{input::Input, task_io_settings::TaskIoSettings};
use algo_lib::misc::min_max::UpdateMinMax;
#[allow(unused)]
use algo_lib::{dbg, out, out_line};

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.usize();
    let a = input.vec::<usize>(n);
    let mut suf = vec![0; n];
    suf[n - 1] = a[n - 1];
    for i in (0..n - 1).rev() {
        suf[i] = suf[i + 1] + a[i];
    }
    let mut res = n;
    let mut add = 0;
    for pref in 0..n {
        let cost = max(add, (n - pref - suf[pref]));
        res.update_min(cost);
        if a[pref] == 1 {
            add += 1;
        }
    }
    res.update_min(add);
    out_line!(res);
}

pub(crate) fn run(mut input: Input) -> bool {
    let t = input.read();
    for i in 0usize..t {
        solve(&mut input, i + 1);
    }
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
    // tester::run_single_test("1");
    // tester::run_stress(stress);
    // tester::run_locally();
    // tester::run_with_last_downloaded_file();
}
//END MAIN
