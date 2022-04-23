//{"name":"H. Максимальный AND","group":"Codeforces - Codeforces Round #784 (Div. 4)","url":"https://codeforces.com/contest/1669/problem/H","interactive":false,"timeLimit":2000,"tests":[{"input":"4\n3 2\n2 1 1\n7 0\n4 6 6 28 6 6 12\n1 30\n0\n4 4\n3 1 3 1\n","output":"2\n4\n2147483646\n1073741825\n"}],"testType":"multiNumber","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"HMaksimalniiAND"}}}

use algo_lib::io::output::output;
use algo_lib::io::task_io_settings::TaskIoType;
use algo_lib::io::task_runner::run_task;
use algo_lib::io::{input::Input, task_io_settings::TaskIoSettings};
#[allow(unused)]
use algo_lib::{dbg, out, out_line};

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.usize();
    let mut k = input.i64();
    dbg!(n, k);
    let a = input.vec::<i32>(n);
    let mut res = 0;
    for bit in (0..=30).rev() {
        let mut need = 0;
        for i in 0..n {
            if (a[i] & (1 << bit)) == 0 {
                need += 1;
            }
        }
        if need <= k {
            k -= need;
            res |= 1 << bit;
        }
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
