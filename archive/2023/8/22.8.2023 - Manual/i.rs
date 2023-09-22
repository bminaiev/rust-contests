//{"name":"i","group":"Manual","url":"","interactive":false,"timeLimit":2000,"tests":[{"input":"","output":""},{"input":"","output":""}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"i"}}}

use algo_lib::collections::bit_set::BitSet;
use algo_lib::io::output::output;
use algo_lib::io::task_io_settings::TaskIoType;
use algo_lib::io::task_runner::run_task;
use algo_lib::io::{input::Input, task_io_settings::TaskIoSettings};
use algo_lib::math::gcd::gcd;
#[allow(unused)]
use algo_lib::{dbg, out, out_line};

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.usize();
    let a = input.vec::<i64>(n);
    let mut bad = vec![BitSet::new(n); n];
    for i in 0..n {
        for j in i + 1..n {
            if gcd(a[i], a[j]) != 1 {
                bad[i].set(j, true);
                bad[j].set(i, true);
            }
        }
    }
    let mut res = 0i64;
    for i in 0..n {
        for j in i + 1..n {
            let mut now_bad = bad[i].clone();
            now_bad |= &bad[j];
            now_bad.set(i, false);
            now_bad.set(j, false);
            let cnt_bad = now_bad.count_ones() as i64;
            let cnt_ok = n as i64 - cnt_bad - 2;
            res += cnt_ok * (cnt_ok - 1) / 2;
        }
    }
    out_line!(res * 4);
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
