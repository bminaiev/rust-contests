//{"name":"F. Stones 1","group":"Yandex - Stage 17: Grand Prix of Seoul","url":"https://official.contest.yandex.com/opencupXXII/contest/39021/problems/F/","interactive":false,"timeLimit":1000,"tests":[{"input":"4\nWBWB\n6 4 5 3\n","output":"5\n"},{"input":"8\nWBBWBWBB\n6 4 8 2 5 3 1 5\n","output":"13\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"FStones1"}}}

use algo_lib::io::output::output;
use algo_lib::io::task_io_settings::TaskIoType;
use algo_lib::io::task_runner::run_task;
use algo_lib::io::{input::Input, task_io_settings::TaskIoSettings};
#[allow(unused)]
use algo_lib::{dbg, out, out_line};

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.usize();
    let s = input.string();
    let a = input.vec::<i64>(n);
    let mut transformed = vec![];
    let mut i = 0;
    while i != n {
        let mut j = i;
        while j != n && s[j] == s[i] {
            j += 1;
        }
        transformed.push(*a[i..j].iter().max().unwrap());
        i = j;
    }
    if transformed.len() <= 2 {
        out_line!(0);
        return;
    }
    let len = transformed.len();
    let mid = &mut transformed[1..len - 1];
    mid.sort();
    let from = mid.len() / 2;
    out_line!(mid[from..].iter().sum::<i64>());
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
    // tester::run_stress(stress);
}
//END MAIN
