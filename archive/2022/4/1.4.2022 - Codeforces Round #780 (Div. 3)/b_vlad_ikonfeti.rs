//{"name":"B. Влад и конфеты","group":"Codeforces - Codeforces Round #780 (Div. 3)","url":"https://codeforces.com/contest/1660/problem/B","interactive":false,"timeLimit":1000,"tests":[{"input":"6\n2\n2 3\n1\n2\n5\n1 6 2 4 3\n4\n2 2 2 1\n3\n1 1000000000 999999999\n1\n1\n","output":"YES\nNO\nNO\nYES\nYES\nYES\n"}],"testType":"multiNumber","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"BVladIKonfeti"}}}

use algo_lib::io::output::output;
use algo_lib::io::task_io_settings::TaskIoType;
use algo_lib::io::task_runner::run_task;
use algo_lib::io::{input::Input, task_io_settings::TaskIoSettings};
#[allow(unused)]
use algo_lib::{dbg, out, out_line};

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.usize();
    let mut a = input.vec::<i64>(n);
    a.sort();
    let ok = if n == 1 {
        a[0] == 1
    } else {
        a[n - 2] + 1 >= a[n - 1]
    };
    if ok {
        out_line!("YES");
    } else {
        out_line!("NO");
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
}
//END MAIN
