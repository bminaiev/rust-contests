//{"name":"F. Final Picture","group":"Yandex - SNWS-2022, Round 4","url":"https://contest.yandex.ru/snws2022/contest/23960/problems/F/","interactive":false,"timeLimit":2000,"tests":[{"input":"9 3\n","output":"92\n"},{"input":"20 22\n","output":"84450197\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"FFinalPicture"}}}

use algo_lib::collections::array_2d::Array2D;
use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::math::modulo::Mod_998_244_353;
use algo_lib::{dbg, out, out_line};

type Mod = Mod_998_244_353;

fn solve(input: &mut Input, _test_case: usize) {
    let h = input.usize();
    let need = input.usize();
    let mut dp = vec![Array2D::new(Mod::ZERO, h + 1, need + 1); 2];
    dp[0][0][0] = Mod::ONE;
    for max_h in 1..=h {
        for sum in 0..=need {
            let mut pref_sum = Mod::ZERO;
            for next_h in (0..=max_h).rev() {
                if next_h != 0 {
                    pref_sum += dp[0][next_h - 1][sum];
                }
                if sum + next_h > need {
                    continue;
                }
                dp[1][next_h][sum + next_h] += pref_sum;
            }
        }
        dp.swap(0, 1);
        for i in 0..=h {
            for j in 0..=need {
                dp[1][i][j] = Mod::ZERO;
            }
        }
    }
    let mut res = Mod::ZERO;
    for last_h in 0..=h {
        res += dp[0][last_h][need];
    }
    out_line!(res);
}

pub(crate) fn run(mut input: Input) -> bool {
    solve(&mut input, 1);
    output().flush();
    input.skip_whitespace();
    input.peek().is_none()
}

//START MAIN
mod tester;

fn main() {
    tester::run_tests();
    // tester::run_single_test("1");
}
//END MAIN
