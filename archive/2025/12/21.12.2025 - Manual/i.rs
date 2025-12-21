//{"name":"i","group":"Manual","url":"","interactive":false,"timeLimit":2000,"tests":[{"input":"","output":""},{"input":"","output":""},{"input":"","output":""},{"input":"","output":""}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::array_2d::Array2D;
#[allow(unused)]
use algo_lib::dbg;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;

fn solve(input: &mut Input, out: &mut Output) {
    let n = input.usize();
    let max_money = input.usize();
    let mut sat_stay = vec![];
    let mut cost_stay = vec![];
    let mut sat_go = vec![];
    let mut cost_go = vec![];
    let mut taxi_cost = vec![];
    let mut prob_a = vec![];
    for _ in 0..n {
        sat_stay.push(input.f64());
        cost_stay.push(input.usize());
        sat_go.push(input.f64());
        cost_go.push(input.usize());
        taxi_cost.push(input.usize());
        prob_a.push(input.i32() as f64 / 100.0);
    }
    const NEG_INF: f64 = -1e18;
    let mut dp = Array2D::new(0.0, 4, max_money + 1);
    for day in (0..n).rev() {
        let mut ndp = Array2D::new(NEG_INF, 4, max_money + 1);
        for mask in 0..4 {
            for money in 0..=max_money {
                if money >= cost_stay[day] {
                    // stay
                    let nsat = prob_a[day] * dp[mask | 1][money - cost_stay[day]]
                        + (1.0 - prob_a[day]) * dp[mask | 2][money - cost_stay[day]]
                        + sat_stay[day];
                    if nsat > ndp[mask][money] {
                        ndp[mask][money] = nsat;
                    }
                }
                let max_taxi_cost = if mask == 3 { taxi_cost[day] } else { 0 };
                if money >= cost_go[day] + max_taxi_cost {
                    // go
                    let nsat = if mask == 3 {
                        dp[0][money - cost_go[day] - max_taxi_cost] + sat_go[day]
                    } else {
                        prob_a[day] * dp[mask | 1][money - cost_go[day]]
                            + (1.0 - prob_a[day]) * dp[mask | 2][money - cost_go[day]]
                            + sat_go[day]
                    };
                    if nsat > ndp[mask][money] {
                        ndp[mask][money] = nsat;
                    }
                }
            }
        }
        dp = ndp;
    }
    let res = dp[0][max_money];
    if res < 0.0 {
        out.println(-1);
    } else {
        out.println(res);
    }
}

pub(crate) fn run(mut input: Input, mut output: Output) -> bool {
    solve(&mut input, &mut output);
    output.flush();
    true
}

#[cfg(feature = "local")]
fn main() {
    const PROBLEM_NAME: &str = "i";
    use algo_lib::tester::helper::*;

    run_tests(PROBLEM_NAME, run);
    // run_single_test(PROBLEM_NAME, run, "1");
    // run_stress(stress);
    // run_locally(run);
}
//END MAIN

#[cfg(not(feature = "local"))]
fn main() {
    let input = algo_lib::io::input::Input::new_stdin();
    let mut output = algo_lib::io::output::Output::new_stdout();
    run(input, output);
}
