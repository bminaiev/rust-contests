//{"name":"H. Build an Array","group":"Codeforces - Testing Constructor Cup","url":"https://codeforces.com/gym/503340/problem/H","interactive":false,"timeLimit":2000,"tests":[{"input":"2 1\n","output":"3\n"},{"input":"1 2\n","output":"1\n"},{"input":"3 5\n","output":"146\n"},{"input":"6 3\n","output":"876\n"},{"input":"100 100\n","output":"875463899\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"HBuildAnArray"}}}

use algo_lib::collections::array_2d::Array2D;
#[allow(unused)]
use algo_lib::dbg;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::math::combinations::{Combinations, CombinationsFact};
use algo_lib::math::modulo::Mod_998_244_353;

type Mod = Mod_998_244_353;

fn solve(input: &mut Input, out: &mut Output, _test_case: usize) {
    let n = input.usize();
    let k = input.usize();
    // [current number of segments][more available segments]
    let mut dp = Array2D::new(Mod::ZERO, k + 1, k + 1);
    dp[0][k] = Mod::ONE;
    for _ in 0..n {
        let mut ndp = Array2D::new(Mod::ZERO, k + 1, k + 1);
        for cur in 0..=k {
            for avail in 0..=k {
                for next in 0..=k {
                    let required = if next > cur { next - cur } else { 0 };
                    if required > avail {
                        continue;
                    }
                    ndp[next][avail - required] += dp[cur][avail];
                }
            }
        }
        dp = ndp;
    }
    let mut res = Mod::ZERO;
    for cur in 0..=k {
        for avail in 0..=k {
            res += dp[cur][avail];
        }
    }
    let cnk = CombinationsFact::<Mod>::new(n + k + 1);
    for used in 0..k {
        res -= cnk.c(n + used - 1, used);
    }
    out.println(res);
}

pub(crate) fn run(mut input: Input, mut output: Output) -> bool {
    solve(&mut input, &mut output, 1);
    output.flush();
    true
}

//START MAIN

fn main() {
    const PROBLEM_NAME: &str = "h_build_an_array";
    use algo_lib::tester::helper::*;

    run_tests(PROBLEM_NAME, run);
    // run_single_test(PROBLEM_NAME, run, "1");
    // run_stress(stress);
    // run_locally(run);
}
//END MAIN
