//{"name":"B. Mainak и интересная последовательность","group":"Codeforces - Codeforces Round #819 (Div. 1 + Div. 2) and Grimoire of Code Annual Contest 2022","url":"https://codeforces.com/contest/1726/problem/B","interactive":false,"timeLimit":1000,"tests":[{"input":"4\n1 3\n6 12\n2 1\n3 6\n","output":"Yes\n3\nYes\n1 3 2 2 3 1\nNo\nYes\n2 2 2\n"}],"testType":"multiNumber","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"BMainakIInteresnayaPosledovatelnost"}}}

use algo_lib::io::output::output;
use algo_lib::io::task_io_settings::TaskIoType;
use algo_lib::io::task_runner::run_task;
use algo_lib::io::{input::Input, task_io_settings::TaskIoSettings};
#[allow(unused)]
use algo_lib::{dbg, out, out_line};

fn solve_case(n: usize, sum: i64) -> Option<Vec<i64>> {
    if n as i64 > sum {
        return None;
    }
    let mut res = vec![1; n];
    if n % 2 == 0 {
        if sum % 2 == 0 {
            let more = sum - (n as i64);
            res[n - 1] += more / 2;
            res[n - 2] += more / 2;
        } else {
            return None;
        }
    } else {
        let more = sum - (n as i64);
        res[n - 1] += more;
    }
    Some(res)
}

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.usize();
    let sum = input.i64();
    if let Some(r) = solve_case(n, sum) {
        out_line!("Yes");
        out_line!(r);
    } else {
        out_line!("No");
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
