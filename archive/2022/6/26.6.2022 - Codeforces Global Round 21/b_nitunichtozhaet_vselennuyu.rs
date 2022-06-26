//{"name":"B. НИТ уничтожает вселенную","group":"Codeforces - Codeforces Global Round 21","url":"https://codeforces.com/contest/1696/problem/B","interactive":false,"timeLimit":2000,"tests":[{"input":"4\n4\n0 0 0 0\n5\n0 1 2 3 4\n7\n0 2 3 0 1 2 0\n1\n1000000000\n","output":"0\n1\n2\n1\n"}],"testType":"multiNumber","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"BNITUnichtozhaetVselennuyu"}}}

use algo_lib::collections::last_exn::LastExn;
use algo_lib::io::output::output;
use algo_lib::io::task_io_settings::TaskIoType;
use algo_lib::io::task_runner::run_task;
use algo_lib::io::{input::Input, task_io_settings::TaskIoSettings};
use algo_lib::misc::min_max::UpdateMinMax;
#[allow(unused)]
use algo_lib::{dbg, out, out_line};

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.usize();
    let a = input.vec::<i32>(n);
    if *a.iter().max().unwrap() == 0 {
        out_line!(0);
    } else {
        let mut first = n;
        let mut last = 0;
        let mut cnt = 0;
        for i in 0..n {
            if a[i] != 0 {
                first.update_min(i);
                last.update_max(i);
                cnt += 1;
            }
        }
        if last - first + 1 == cnt {
            out_line!(1);
        } else {
            out_line!(2);
        }
    }
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
}
//END MAIN
