//{"name":"A. Сделай его возрастающим","group":"Codeforces - Codeforces Round #783 (Div. 1)","url":"https://codeforces.com/contest/1667/problem/0","interactive":false,"timeLimit":2000,"tests":[{"input":"5\n1 2 3 4 5\n","output":"4\n"},{"input":"7\n1 2 1 2 1 2 1\n","output":"10\n"},{"input":"8\n1 8 2 7 3 6 4 5\n","output":"16\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"ASdelaiYegoVozrastayushchim"}}}

use algo_lib::io::output::output;
use algo_lib::io::task_io_settings::TaskIoType;
use algo_lib::io::task_runner::run_task;
use algo_lib::io::{input::Input, task_io_settings::TaskIoSettings};
use algo_lib::misc::min_max::UpdateMinMax;
#[allow(unused)]
use algo_lib::{dbg, out, out_line};

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.usize();
    let a = input.vec::<i64>(n);
    let mut res = std::i64::MAX;
    for stay_pos in 0..n {
        let mut cur_res = 0;
        {
            let mut prev = 0;
            for i in stay_pos + 1..n {
                let next = (prev / a[i] + 1) * a[i];
                assert!(next > prev);
                cur_res += next / a[i];
                prev = next;
            }
        }
        {
            let mut prev = 0;
            for i in (0..stay_pos).rev() {
                let next = (prev / a[i] + 1) * a[i];
                assert!(next > prev);
                cur_res += next / a[i];
                prev = next;
            }
        }
        res.update_min(cur_res);
    }
    out_line!(res);
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
