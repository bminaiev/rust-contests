//{"name":"F - Many Xor Optimization Problems","group":"AtCoder - AtCoder Regular Contest 139","url":"https://atcoder.jp/contests/arc139/tasks/arc139_f","interactive":false,"timeLimit":8000,"tests":[{"input":"2 1\n","output":"3\n"},{"input":"3 4\n","output":"52290\n"},{"input":"1234 5678\n","output":"495502261\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"FManyXorOptimizationProblems"}}}

use std::cmp::{max, min};

use algo_lib::collections::array_2d::Array2D;
use algo_lib::io::output::output;
use algo_lib::io::task_io_settings::TaskIoType;
use algo_lib::io::task_runner::run_task;
use algo_lib::io::{input::Input, task_io_settings::TaskIoSettings};
use algo_lib::math::modulo::Mod_998_244_353;
#[allow(unused)]
use algo_lib::{dbg, out, out_line};

type Mod = Mod_998_244_353;

fn solve(input: &mut Input) {
    let n = input.usize();
    let bits = input.usize();
    let mut dp = Array2D::new(Mod::ZERO, n + 1, bits + 1);
    dp[0][0] = Mod::ONE;
    let pow2 = Mod::gen_powers(Mod::TWO, max(bits, n) + 1);
    let mut res = Mod::ZERO;
    for bit in 0..bits {
        for used in 0..=min(bit, n) {
            let cur = dp[used][bit];
            let left = n - used;
            let pr_no_one = Mod::ONE / pow2[left];
            dp[used][bit + 1] += cur * pr_no_one;
            if used != 0 {
                res += cur * pr_no_one * pow2[bits - bit - 1] / Mod::TWO;
            }
            if pr_no_one != Mod::ONE {
                res += cur * (Mod::ONE - pr_no_one) * pow2[bits - bit - 1];
                dp[used + 1][bit + 1] += cur * (Mod::ONE - pr_no_one);
            }
        }
    }
    for _bit in 0..bits {
        res *= pow2[n];
    }
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
}
//END MAIN
