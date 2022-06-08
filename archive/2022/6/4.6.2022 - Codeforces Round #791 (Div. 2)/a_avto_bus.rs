//{"name":"A. AvtoBus","group":"Codeforces - Codeforces Round #791 (Div. 2)","url":"https://codeforces.com/contest/1679/problem/A","interactive":false,"timeLimit":1000,"tests":[{"input":"4\n4\n7\n24\n998244353998244352\n","output":"1 1\n-1\n4 6\n166374058999707392 249561088499561088\n"}],"testType":"multiNumber","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"AAvtoBus"}}}

use algo_lib::io::output::output;
use algo_lib::io::task_io_settings::TaskIoType;
use algo_lib::io::task_runner::run_task;
use algo_lib::io::{input::Input, task_io_settings::TaskIoSettings};
#[allow(unused)]
use algo_lib::{dbg, out, out_line};

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.i64();
    let mut smallest = std::i64::MAX;
    let mut largest = std::i64::MIN;
    let mut update = |x: i64| {
        smallest = std::cmp::min(smallest, x);
        largest = std::cmp::max(largest, x);
    };
    const M: i64 = 10;
    for cnt4 in 0..M {
        let left = n - cnt4 * 4;
        if left % 6 != 0 || left < 0 {
            continue;
        }
        update(left / 6 + cnt4);
    }
    for cnt6 in 0..M {
        let left = n - cnt6 * 6;
        if left % 4 != 0 || left < 0 {
            continue;
        }
        update(left / 4 + cnt6);
    }
    if smallest == std::i64::MAX {
        out_line!("-1");
    } else {
        out_line!(format!("{} {}", smallest, largest));
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
