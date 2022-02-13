//{"name":"C. Андрей и камни","group":"Codeforces - Codeforces Global Round 19","url":"https://codeforces.com/contest/1637/problem/C","interactive":false,"timeLimit":1000,"tests":[{"input":"4\n5\n1 2 2 3 6\n3\n1 3 1\n3\n1 2 1\n4\n3 1 1 2\n","output":"4\n-1\n1\n-1\n"}],"testType":"multiNumber","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"CAndreiIKamni"}}}

use algo_lib::io::output::output;
use algo_lib::io::task_io_settings::TaskIoType;
use algo_lib::io::task_runner::run_task;
use algo_lib::io::{input::Input, task_io_settings::TaskIoSettings};
#[allow(unused)]
use algo_lib::{dbg, out, out_line};

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.usize();
    let a = input.read_vec::<i64>(n);
    if a[1..n - 1].iter().all(|x| *x == 1) {
        out_line!(-1);
        return;
    }
    if n == 3 && a[1] % 2 == 1 {
        out_line!(-1);
        return;
    }
    let mut res = 0;
    for x in 1..n - 1 {
        res += (a[x] + 1) / 2;
    }
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
}
//END MAIN
