//{"name":"B - Sum of Three Terms","group":"AtCoder - AtCoder Regular Contest 135","url":"https://atcoder.jp/contests/arc135/tasks/arc135_b","interactive":false,"timeLimit":2000,"tests":[{"input":"5\n6 9 6 6 5\n","output":"Yes\n0 4 2 3 1 2 2\n"},{"input":"5\n0 1 2 1 0\n","output":"No\n"},{"input":"1\n10\n","output":"Yes\n0 0 10\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"BSumOfThreeTerms"}}}

use algo_lib::io::output::output;
use algo_lib::io::task_io_settings::TaskIoType;
use algo_lib::io::task_runner::run_task;
use algo_lib::io::{input::Input, task_io_settings::TaskIoSettings};
use algo_lib::misc::min_max::UpdateMinMax;
#[allow(unused)]
use algo_lib::{dbg, out, out_line};

fn solve(input: &mut Input) {
    let n = input.usize();
    let s = input.read_vec::<i64>(n);
    let mut a = vec![];
    for start in 0..3 {
        let mut min_delta = 0;
        let mut cur_delta = 0;
        for pos in (start..n).step_by(3) {
            if pos + 1 < n {
                let apply = s[pos + 1] - s[pos];
                cur_delta += apply;
                min_delta.update_min(cur_delta);
            }
        }
        a.push(-min_delta);
    }
    let already_sum = a[0] + a[1] + a[2];
    if already_sum > s[0] {
        out_line!("No");
        return;
    }
    out_line!("Yes");
    a[0] += s[0] - already_sum;
    for pos in 1..n {
        let cur_sum = a[pos] + a[pos + 1];
        let need = s[pos] - cur_sum;
        assert!(need >= 0);
        a.push(need);
    }
    out_line!(a);
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
}
//END MAIN
