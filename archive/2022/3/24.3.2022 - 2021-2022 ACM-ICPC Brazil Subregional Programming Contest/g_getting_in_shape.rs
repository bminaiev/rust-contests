//{"name":"G. Getting in Shape","group":"Codeforces - 2021-2022 ACM-ICPC Brazil Subregional Programming Contest","url":"https://codeforces.com/gym/103388/problem/G","interactive":false,"timeLimit":500,"tests":[{"input":"2\n","output":"AB\n"},{"input":"4\n","output":"ABAB\n"},{"input":"7\n","output":"IMPOSSIBLE\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"GGettingInShape"}}}

use algo_lib::collections::last_exn::LastExn;
use algo_lib::io::output::output;
use algo_lib::io::task_io_settings::TaskIoType;
use algo_lib::io::task_runner::run_task;
use algo_lib::io::{input::Input, task_io_settings::TaskIoSettings};
use algo_lib::misc::rec_function::{Callable2, RecursiveFunction2};
#[allow(unused)]
use algo_lib::{dbg, out, out_line};

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.i64();
    let mut fibs = vec![1, 1i64];
    while *fibs.last_exn() <= n {
        let sz = fibs.len();
        let next = fibs[sz - 1] + fibs[sz - 2];
        fibs.push(next);
    }
    fibs.reverse();
    dbg!(fibs.len());
    let mut res = vec![];
    let mut go = RecursiveFunction2::new(|f, n: i64, it: usize| -> bool {
        if n == 1 {
            return true;
        }
        if it == fibs.len() {
            return false;
        }
        if n % fibs[it] == 0 && fibs[it] != 1 {
            res.push(fibs.len() - it - 1);
            if f.call(n / fibs[it], it) {
                return true;
            }
            res.pop();
        }
        f.call(n, it + 1)
    });
    if go.call(n, 0) {
        for &cnt in res.iter() {
            assert!(cnt > 1);
            for _ in 0..cnt - 1 {
                out!("A");
            }
            out!("B");
        }
        out_line!();
    } else {
        out_line!("IMPOSSIBLE");
    }
}

pub(crate) fn run(mut input: Input) -> bool {
    solve(&mut input, 1);
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
    // tester::run_tests();
    // tester::run_single_test("1");
    tester::run_locally();
}
//END MAIN
