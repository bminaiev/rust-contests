//{"name":"C. Деревянная ложка","group":"Codeforces - VK 2022 Finals","url":"https://codeforces.com/gym/425375/problem/C","interactive":false,"timeLimit":4000,"tests":[{"input":"1\n","output":"0\n2\n"},{"input":"2\n","output":"0\n0\n8\n16\n"},{"input":"3\n","output":"0\n0\n0\n1536\n4224\n7680\n11520\n15360\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"CDerevyannayaLozhka"}}}

use algo_lib::collections::array_2d::Array2D;
use algo_lib::io::output::output;
use algo_lib::io::task_io_settings::TaskIoType;
use algo_lib::io::task_runner::run_task;
use algo_lib::io::{input::Input, task_io_settings::TaskIoSettings};
use algo_lib::math::modulo::Mod_998_244_353;
#[allow(unused)]
use algo_lib::{dbg, out, out_line};

type Mod = Mod_998_244_353;

fn calc(max_pw: usize) -> Vec<Mod> {
    let n = 1 << max_pw;
    // dp[pw][pos] - (our tree size is 2**pw. Current element is pos) -> prob
    let mut dp = Array2D::new(Mod::ZERO, max_pw + 1, n + 1);
    dp[max_pw][0] = Mod::ONE;
    let mut res = vec![Mod::ZERO; n];
    let mut inv = vec![Mod::ZERO; n + 1];
    for i in 1..=n {
        inv[i] = Mod::ONE / Mod::new(i);
    }
    for pw in (0..=max_pw).rev() {
        for pos in 0..n {
            let cur_dp = dp[pw][pos];
            if cur_dp == Mod::ZERO {
                continue;
            }
            let outside_of_tree = n - (1 << pw);
            let more_to_put = n - pos;
            let more_outside = outside_of_tree - pos;
            let prob_cur_element_is_outside = Mod::new(more_outside) * inv[more_to_put];
            dp[pw][pos + 1] += prob_cur_element_is_outside * cur_dp;
            let prob_cur_element_is_min = Mod::ONE - prob_cur_element_is_outside;
            if pw == 0 {
                res[pos] = prob_cur_element_is_min * cur_dp;
            } else {
                dp[pw - 1][pos + 1] += prob_cur_element_is_min * cur_dp;
            }
        }
    }
    let mut fact = Mod::ONE;
    for i in 1..=n {
        fact *= Mod::new(i);
    }
    for x in res.iter_mut() {
        *x *= fact;
    }
    res
}

fn stress() {
    calc(20);
}

fn solve(input: &mut Input, _test_case: usize) {
    let max_pw = input.usize();
    let res = calc(max_pw);
    for &x in res.iter() {
        out_line!(x);
    }
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
    // tester::run_single_test("3");
    // tester::run_stress(stress);
    // tester::run_locally();
    // tester::run_with_last_downloaded_file();
}
//END MAIN
