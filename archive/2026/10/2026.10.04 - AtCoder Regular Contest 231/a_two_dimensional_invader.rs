use algo_lib::collections::array_2d::Array2D;
#[allow(unused)]
use algo_lib::dbg;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::pref_sum::PrefSum;

fn d(x: usize, y: usize) -> i64 {
    let x = x as i64;
    let y = y as i64;
    ((x - y) * (x - y))
}

fn solve(input: &mut Input, out: &mut Output) {
    let tc = input.usize();
    for _ in 0..tc {
        let n = input.usize();
        let mut xx = vec![];
        let mut yy = vec![];
        let mut zz = vec![];
        for _ in 0..n {
            xx.push(input.usize());
            yy.push(input.usize());
            zz.push(input.i64());
        }
        let zz_pref = zz.pref_sum();
        assert_eq!(size_of_val(&zz_pref[0]), 8);
        const MAX_C: usize = 501;
        // dp[prev_x][next_y]
        let mut dp = Array2D::new(i64::MAX / 2, MAX_C, MAX_C);
        for next_y in 0..MAX_C {
            let cost = (next_y * next_y) as i64;
            dp[0][next_y] = cost;
        }
        let mut res = zz_pref[n];
        for i in 0..n {
            let x = xx[i];
            let y = yy[i];
            let mut res_here = i64::MAX;
            for prev_x in 0..MAX_C {
                let cost = zz_pref[i] + d(x, prev_x) + dp[prev_x][y];
                res_here = res_here.min(cost);
            }
            for next_y in 0..MAX_C {
                let ncost = res_here - zz_pref[i + 1] + d(next_y, y);
                dp[x][next_y] = dp[x][next_y].min(ncost);
            }
            res = res.min(res_here + (zz_pref[n] - zz_pref[i + 1]));
        }
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
    const PROBLEM_NAME: &str = "a_two_dimensional_invader";
    #[allow(unused)]
    use algo_lib::misc::dragon::run_dragon;
    use algo_lib::tester::helper::*;

    run_tests(PROBLEM_NAME, run);
    // run_single_test(PROBLEM_NAME, run, "1");
    // run_stress(stress);
    // run_locally(run);
    // run_dragon(solve, "W", 1..2);
}
//END MAIN

#[cfg(not(feature = "local"))]
fn main() {
    let input = algo_lib::io::input::Input::new_stdin();
    let mut output = algo_lib::io::output::Output::new_stdout();
    run(input, output);
}
