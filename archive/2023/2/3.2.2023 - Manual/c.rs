//{"name":"c","group":"Manual","url":"","interactive":false,"timeLimit":2000,"tests":[{"input":"","output":""}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"c"}}}

use algo_lib::io::output::output;
use algo_lib::io::task_io_settings::TaskIoType;
use algo_lib::io::task_runner::run_task;
use algo_lib::io::{input::Input, task_io_settings::TaskIoSettings};
use algo_lib::misc::min_max::UpdateMinMax;
#[allow(unused)]
use algo_lib::{dbg, out, out_line};

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.usize();
    let d = input.i64();
    let a = input.vec::<i64>(n);
    let mut res = 0;
    let mut to_check_l = a.clone();
    for &x in a.iter() {
        to_check_l.push(x - d);
    }
    for &l in to_check_l.iter() {
        let r = l + d;
        let clamp = |x: i64| {
            if x < l {
                l
            } else if x > r {
                r
            } else {
                x
            }
        };
        let mut cur_res = 0;
        let mut prev = clamp(a[0]);
        for i in 1..n {
            let next = clamp(a[i]);
            cur_res += (next - prev).abs();
            prev = next;
        }
        res.update_max(cur_res);
    }
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
