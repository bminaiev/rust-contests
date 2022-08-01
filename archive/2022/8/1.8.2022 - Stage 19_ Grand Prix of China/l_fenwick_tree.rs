//{"name":"L. Fenwick Tree","group":"Yandex - Stage 19: Grand Prix of China","url":"https://official.contest.yandex.com/opencupXXII/contest/39025/problems/L/","interactive":false,"timeLimit":1000,"tests":[{"input":"3\n5\n10110\n5\n00000\n5\n11111\n","output":"3\n0\n3\n"}],"testType":"multiNumber","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"LFenwickTree"}}}

use algo_lib::io::output::output;
use algo_lib::io::task_io_settings::TaskIoType;
use algo_lib::io::task_runner::run_task;
use algo_lib::io::{input::Input, task_io_settings::TaskIoSettings};
#[allow(unused)]
use algo_lib::{dbg, out, out_line};

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.usize();
    let s = input.string();
    let mut cnt_not_zero = vec![0; n + 1];
    let mut res = 0;
    for v in 1..=n {
        if s[v - 1] == b'1' {
            let pos = v as i64;
            let next = pos + (pos & (-pos));
            let parent = next as usize;
            if parent < cnt_not_zero.len() {
                cnt_not_zero[parent] += 1;
            }
        }
        if s[v - 1] == b'0' && cnt_not_zero[v] == 1 {
            res += 1;
        } else if s[v - 1] == b'1' && cnt_not_zero[v] == 0 {
            res += 1;
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
    // tester::run_stress(stress);
}
//END MAIN
