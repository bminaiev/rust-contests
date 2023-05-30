//{"name":"b","group":"Manual","url":"","interactive":false,"timeLimit":2000,"tests":[{"input":"","output":""},{"input":"","output":""}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"b"}}}

use algo_lib::collections::array_2d::Array2D;
use algo_lib::io::output::output;
use algo_lib::io::task_io_settings::TaskIoType;
use algo_lib::io::task_runner::run_task;
use algo_lib::io::{input::Input, task_io_settings::TaskIoSettings};
use algo_lib::math::combinations::{Combinations, CombinationsFact};
use algo_lib::math::modulo::Mod_998_244_353;
#[allow(unused)]
use algo_lib::{dbg, out, out_line};

type Mod = Mod_998_244_353;

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.usize();
    let mut dp = Array2D::new(Mod::ZERO, n + 1, n + 1);
    dp[1][1] = Mod::ONE;
    let pow2 = Mod::gen_powers(Mod::TWO, n + 1);
    let cnk = CombinationsFact::<Mod>::new(n + 1);
    for len in 1..n {
        for free in 0..=len {
            for use_not_free in 0..2 {
                let not_free_ways = pow2[len - free] - Mod::ONE;
                for used_free in 0..=free {
                    if used_free == 0 && use_not_free == 0 {
                        continue;
                    }
                    let ways = cnk.c(free, used_free);
                    let ways = if use_not_free == 0 {
                        ways
                    } else {
                        ways * not_free_ways
                    };
                    let ways = ways * dp[len][free];
                    dp[len + 1][free - used_free + 1] += ways;
                }
            }
        }
    }
    let res = dp[n][1];
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
