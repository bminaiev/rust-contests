//{"name":"F. Случайный код","group":"Codeforces - Educational Codeforces Round 121 (рейтинговый для Div. 2)","url":"https://codeforces.com/contest/1626/problem/F","interactive":false,"timeLimit":3000,"tests":[{"input":"3 10 3 5 13 88\n","output":"382842030\n"},{"input":"2 15363 270880 34698 17 2357023\n","output":"319392398\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"FSluchainiiKod"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::math::gcd::{gcd, lcm};
use algo_lib::math::modulo::Mod_998_244_353;
use algo_lib::misc::num_traits::ConvI32;
use algo_lib::{dbg, out, out_line};

type Mod = Mod_998_244_353;

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.usize();
    let a0 = input.i64();
    let x = input.i64();
    let y = input.i64();
    let k = input.usize();
    let m = input.i64();

    let mut max = 1;
    for x in 2..=k {
        max = lcm(max, x);
    }

    let pr_choose = Mod::ONE / Mod::new(n as i32);
    let pr_not_choose = Mod::ONE - pr_choose;

    let mut dp = vec![Mod::ZERO; max];
    for i in (1..=k).rev() {
        for start in (0..max).step_by(i) {
            for shift in (0..i).rev() {
                let j = start + shift;
                dp[j] = pr_choose * (dp[start] + Mod::new(j as i32)) + pr_not_choose * dp[j];
            }
        }
    }

    let f = |value: i64| -> Mod {
        let value = value.to_i32() as usize;
        let more = value / dp.len() * dp.len();
        dp[value - more] + pr_choose * Mod::new(k as i32) * Mod::new(more as i32)
    };
    let mut res = f(a0);
    let mut prev = a0;
    for _ in 1..n {
        prev = (prev * x + y) % m;
        res += f(prev);
    }
    res *= Mod::new(n as i32).pown(k);
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
}
//END MAIN
