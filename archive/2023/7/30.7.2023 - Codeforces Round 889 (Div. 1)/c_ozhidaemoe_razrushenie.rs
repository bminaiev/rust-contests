//{"name":"C. Ожидаемое разрушение","group":"Codeforces - Codeforces Round 889 (Div. 1)","url":"https://codeforces.com/contest/1854/problem/C","interactive":false,"timeLimit":1000,"tests":[{"input":"2 3\n1 3\n","output":"750000009\n"},{"input":"5 10\n1 2 3 4 5\n","output":"300277731\n"},{"input":"5 10\n2 3 6 8 9\n","output":"695648216\n"},{"input":"1 100\n1\n","output":"100\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"COzhidaemoeRazrushenie"}}}

use algo_lib::collections::array_2d::Array2D;
use algo_lib::io::output::output;
use algo_lib::io::task_io_settings::TaskIoType;
use algo_lib::io::task_runner::run_task;
use algo_lib::io::{input::Input, task_io_settings::TaskIoSettings};
use algo_lib::math::modulo::Mod7;
use algo_lib::misc::vec_apply_delta::ApplyDelta;
#[allow(unused)]
use algo_lib::{dbg, out, out_line};

type Mod = Mod7;

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.usize();
    let m = input.usize();
    let mut dp = Array2D::new(Mod::ZERO, m + 1, m + 1);
    for first in (0..=m).rev() {
        for second in (first + 1..=m).rev() {
            if second == m {
                dp[first][second] = Mod::ONE + dp[first + 1][second];
            } else {
                dp[first][second] =
                    (dp[first + 1][second] + dp[first][second + 1] + Mod::ONE) / Mod::TWO;
            }
        }
    }
    let mut s = input.vec::<usize>(n).sub_from_all(1);
    s.push(m);
    let mut res = Mod::ZERO;
    for w in s.windows(2) {
        res += dp[w[0]][w[1]];
    }
    out_line!(res);
}

pub(crate) fn run(mut input: Input) -> bool {
    solve(&mut input, 1);
    output().flush();
    true
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
    // tester::run_locally();
    // tester::run_with_last_downloaded_file();
}
//END MAIN
