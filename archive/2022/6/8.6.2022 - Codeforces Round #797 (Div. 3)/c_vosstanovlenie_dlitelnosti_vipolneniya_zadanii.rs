//{"name":"C. Восстановление длительности выполнения заданий","group":"Codeforces - Codeforces Round #797 (Div. 3)","url":"https://codeforces.com/contest/1690/problem/C","interactive":false,"timeLimit":2000,"tests":[{"input":"4\n3\n0 3 7\n2 10 11\n2\n10 15\n11 16\n9\n12 16 90 195 1456 1569 3001 5237 19275\n13 199 200 260 9100 10000 10914 91066 5735533\n1\n0\n1000000000\n","output":"2 7 1\n1 1\n1 183 1 60 7644 900 914 80152 5644467\n1000000000\n"}],"testType":"multiNumber","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"CVosstanovlenieDlitelnostiVipolneniyaZadanii"}}}

use std::cmp::max;

use algo_lib::io::output::output;
use algo_lib::io::task_io_settings::TaskIoType;
use algo_lib::io::task_runner::run_task;
use algo_lib::io::{input::Input, task_io_settings::TaskIoSettings};
#[allow(unused)]
use algo_lib::{dbg, out, out_line};

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.usize();
    let start = input.vec::<i64>(n);
    let finish = input.vec::<i64>(n);
    let mut first_ok = 0;
    for i in 0..n {
        let real_start = max(first_ok, start[i]);
        out!(finish[i] - real_start, "");
        first_ok = finish[i];
    }
    out_line!();
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
}
//END MAIN
