//{"name":"A. Циклический локальный минимакс","group":"Codeforces - Codeforces Round #794 (Div. 1)","url":"https://codeforces.com/contest/1685/problem/0","interactive":false,"timeLimit":1000,"tests":[{"input":"4\n3\n1 1 2\n4\n1 9 8 4\n4\n2 0 2 2\n6\n1 1 1 11 111 1111\n","output":"NO\nYES\n1 8 4 9\nNO\nYES\n1 11 1 111 1 1111\n"}],"testType":"multiNumber","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"ATsiklicheskiiLokalniiMinimaks"}}}

use algo_lib::io::output::output;
use algo_lib::io::task_io_settings::TaskIoType;
use algo_lib::io::task_runner::run_task;
use algo_lib::io::{input::Input, task_io_settings::TaskIoSettings};
#[allow(unused)]
use algo_lib::{dbg, out, out_line};

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.usize();
    let mut a = input.vec::<i32>(n);
    a.sort();
    if n % 2 == 1 {
        out_line!("NO");
        return;
    }
    let mut b = vec![0; n];
    for i in 0..n / 2 {
        b[i * 2] = a[i];
        b[i * 2 + 1] = a[i + n / 2];
    }
    let mut ok = true;
    for i in 0..n {
        let prev = b[(i + n - 1) % n];
        let next = b[(i + 1) % n];
        let cur = b[i];
        let bigger = cur > prev && cur > next;
        let smaller = cur < prev && cur < next;
        if !(bigger || smaller) {
            ok = false;
        }
    }
    if !ok {
        out_line!("NO");
        return;
    }
    out_line!("YES");
    out_line!(b);
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
