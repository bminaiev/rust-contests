//{"name":"d","group":"Manual","url":"","interactive":false,"timeLimit":2000,"tests":[{"input":"","output":""},{"input":"","output":""}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"d"}}}

use algo_lib::io::output::output;
use algo_lib::io::task_io_settings::TaskIoType;
use algo_lib::io::task_runner::run_task;
use algo_lib::io::{input::Input, task_io_settings::TaskIoSettings};
use algo_lib::math::modulo::Mod_998_244_353;
use algo_lib::misc::min_max::UpdateMinMax;
#[allow(unused)]
use algo_lib::{dbg, out, out_line};

type Mod = Mod_998_244_353;

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.usize();
    let w = input.vec::<i64>(n);
    let mut dp = vec![i64::MAX; 1 << n];
    dp[1] = 0;
    for mask in 0..(1 << n) {
        if dp[mask] == i64::MAX {
            continue;
        }
        for add in 1..n {
            let mut nmask = mask | (mask << add) | ((mask << add) >> n);
            nmask &= (1 << n) - 1;
            let ncost = dp[mask] + w[n - add];
            dp[nmask].update_min(ncost);
        }
    }
    for bit in 0..n {
        for mask in (0..(1 << n)).rev() {
            if mask & (1 << bit) != 0 {
                let nmask = mask ^ (1 << bit);
                let cost = dp[mask];
                dp[nmask].update_min(cost);
            }
        }
    }
    let mut res = Mod::ZERO;
    for mask in 1..(1 << n) {
        let dp_val = Mod::new(dp[mask] % 998_244_353);
        let mul = Mod::new(mask);
        res += dp_val * mul;
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
