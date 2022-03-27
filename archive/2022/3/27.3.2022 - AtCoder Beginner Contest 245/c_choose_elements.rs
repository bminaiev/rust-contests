//{"name":"C - Choose Elements","group":"AtCoder - AtCoder Beginner Contest 245","url":"https://atcoder.jp/contests/abc245/tasks/abc245_c","interactive":false,"timeLimit":2000,"tests":[{"input":"5 4\n9 8 3 7 2\n1 6 2 9 5\n","output":"Yes\n"},{"input":"4 90\n1 1 1 100\n1 2 3 100\n","output":"No\n"},{"input":"4 1000000000\n1 1 1000000000 1000000000\n1 1000000000 1 1000000000\n","output":"Yes\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"CChooseElements"}}}

use algo_lib::io::output::output;
use algo_lib::io::task_io_settings::TaskIoType;
use algo_lib::io::task_runner::run_task;
use algo_lib::io::{input::Input, task_io_settings::TaskIoSettings};
use algo_lib::misc::gen_vector::gen_vec;
#[allow(unused)]
use algo_lib::{dbg, out, out_line};

fn solve(input: &mut Input) {
    let n = input.usize();
    let k = input.i64();
    let a = gen_vec(2, |_| input.vec::<i64>(n));
    let mut dp = vec![vec![false; n]; 2];
    dp[0][0] = true;
    dp[1][0] = true;
    for i in 1..n {
        for j1 in 0..2 {
            for j2 in 0..2 {
                if (a[j1][i - 1] - a[j2][i]).abs() <= k {
                    if dp[j1][i - 1] {
                        dp[j2][i] = true;
                    }
                }
            }
        }
    }
    if dp[0][n - 1] || dp[1][n - 1] {
        out_line!("Yes");
    } else {
        out_line!("No");
    }
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
