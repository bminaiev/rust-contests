//{"name":"K. Kim's Quest","group":"Codeforces - NEF 23","url":"https://codeforces.com/gym/490499/problem/K","interactive":false,"timeLimit":3000,"tests":[{"input":"3\n1 2 3\n","output":"1\n"},{"input":"5\n2 8 2 6 4\n","output":"16\n"},{"input":"5\n5 7 1 3 5\n","output":"0\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"KKimsQuest"}}}

use std::cmp::min;

use algo_lib::collections::array_2d::Array2D;
use algo_lib::io::output::output;
use algo_lib::io::task_io_settings::TaskIoType;
use algo_lib::io::task_runner::run_task;
use algo_lib::io::{input::Input, task_io_settings::TaskIoSettings};
use algo_lib::math::modulo::Mod_998_244_353;
#[allow(unused)]
use algo_lib::{dbg, out, out_line};

type Mod = Mod_998_244_353;

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.usize();
    let a = input.vec::<usize>(n);
    let mut dp = Array2D::new(Mod::ZERO, 4, 8);
    let mut res = Mod::ZERO;
    dp[0][0] = Mod::ONE;
    for &x in a.iter() {
        let x = (x % 2) * 4;
        let mut ndp = Array2D::new(Mod::ZERO, 4, 8);
        for len in 0..=3 {
            for mask in 0..8usize {
                for use_it in 0..2 {
                    let mut nmask = mask;
                    if use_it == 1 {
                        nmask /= 2;
                        nmask += x;
                    }
                    let nlen = min(len + use_it, 3);
                    if nlen == 3 && nmask.count_ones() % 2 == 1 {
                        continue;
                    }
                    if nlen == 3 && use_it == 1 {
                        res += dp[len][mask];
                    }
                    ndp[nlen][nmask] += dp[len][mask];
                }
            }
        }
        dp = ndp;
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
    // tester::run_single_test("2");
    // tester::run_stress(stress);
    // tester::run_locally();
    // tester::run_with_last_downloaded_file();
}
//END MAIN
