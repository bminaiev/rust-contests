//{"name":"C. Creating Multiples","group":"Codeforces - 2021-2022 ACM-ICPC Brazil Subregional Programming Contest","url":"https://codeforces.com/gym/103388/problem/C","interactive":false,"timeLimit":250,"tests":[{"input":"10 5\n2 3 4 5 6\n","output":"3 0\n"},{"input":"10 3\n1 0 2\n","output":"-1 -1\n"},{"input":"2 5\n1 0 1 1 1\n","output":"4 0\n"},{"input":"17 5\n3 0 0 0 0\n","output":"1 0\n"},{"input":"16 4\n15 0 13 10\n","output":"1 14\n"},{"input":"16 5\n1 15 0 13 10\n","output":"0 0\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"CCreatingMultiples"}}}

use algo_lib::io::output::output;
use algo_lib::io::task_io_settings::TaskIoType;
use algo_lib::io::task_runner::run_task;
use algo_lib::io::{input::Input, task_io_settings::TaskIoSettings};
#[allow(unused)]
use algo_lib::{dbg, out, out_line};

fn solve(input: &mut Input, _test_case: usize) {
    let base = input.i64();
    let n = input.usize();
    let digits = input.vec::<i64>(n);
    let mut mult = vec![1i64; n];
    let m = base + 1;
    for pos in (0..n - 1).rev() {
        mult[pos] = (mult[pos + 1] * base) % m;
    }
    let mut cur = 0;
    for i in 0..n {
        cur += mult[i] * digits[i];
        cur %= m
    }
    if cur == 0 {
        out_line!(0, 0);
        return;
    }
    for pos in 0..n {
        let mul = mult[pos];
        assert!(mul == 1 || mul == base);
        if mul == 1 {
            if digits[pos] >= cur {
                out_line!(pos + 1, digits[pos] - cur);
                return;
            }
        } else {
            if digits[pos] + cur - m >= 0 {
                out_line!(pos + 1, digits[pos] + cur - m);
                return;
            }
        }
    }
    out_line!(-1, -1);
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
    tester::run_tests();
    // tester::run_single_test("1");
}
//END MAIN
