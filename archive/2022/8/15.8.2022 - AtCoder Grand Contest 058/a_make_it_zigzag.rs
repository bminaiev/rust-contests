//{"name":"A - Make it Zigzag","group":"AtCoder - AtCoder Grand Contest 058","url":"https://atcoder.jp/contests/agc058/tasks/agc058_a","interactive":false,"timeLimit":2000,"tests":[{"input":"2\n4 3 2 1\n","output":"2\n1 3\n"},{"input":"1\n1 2\n","output":"0\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"AMakeItZigzag"}}}

use algo_lib::io::output::output;
use algo_lib::io::task_io_settings::TaskIoType;
use algo_lib::io::task_runner::run_task;
use algo_lib::io::{input::Input, task_io_settings::TaskIoSettings};
#[allow(unused)]
use algo_lib::{dbg, out, out_line};

fn solve(input: &mut Input) {
    let n = input.usize() * 2;
    let mut a = input.vec::<usize>(n);
    let mut res = vec![];
    for pos in 0..a.len() - 1 {
        let smaller = pos % 2 == 0;
        if smaller && a[pos] < a[pos + 1] {
            continue;
        }
        if !smaller && a[pos] > a[pos + 1] {
            continue;
        }
        if pos + 2 < a.len() {
            if (smaller && a[pos + 2] > a[pos]) || (!smaller && a[pos + 2] < a[pos]) {
                res.push(pos + 2);
                a.swap(pos + 1, pos + 2);
                continue;
            }
        }
        res.push(pos + 1);
        a.swap(pos, pos + 1);
    }
    assert!(res.len() <= n / 2);
    out_line!(res.len());
    out_line!(res);
}

pub(crate) fn run(mut input: Input) -> bool {
    solve(&mut input);
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
