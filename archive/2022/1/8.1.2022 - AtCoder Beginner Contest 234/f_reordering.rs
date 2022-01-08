//{"name":"F - Reordering","group":"AtCoder - AtCoder Beginner Contest 234","url":"https://atcoder.jp/contests/abc234/tasks/abc234_f","interactive":false,"timeLimit":2000,"tests":[{"input":"aab\n","output":"8\n"},{"input":"aaa\n","output":"3\n"},{"input":"abcdefghijklmnopqrstuvwxyz\n","output":"149621752\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"FReordering"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::math::combinations::{Combinations, CombinationsFact};
use algo_lib::math::modulo::Mod_998_244_353;
use algo_lib::{dbg, out, out_line};

type Mod = Mod_998_244_353;

fn solve(input: &mut Input) {
    let s = input.string_as_vec();
    let mut cnt = vec![0; 26];
    for &c in s.iter() {
        cnt[(c - b'a') as usize] += 1;
    }
    let mut res = Mod::ONE;
    let mut dp = vec![Mod::ONE];
    let cnk = CombinationsFact::new(s.len() + 1);
    for &x in cnt.iter() {
        let mut ndp = vec![Mod::ZERO; dp.len() + x];
        for old in 0..dp.len() {
            for new in 0..=x {
                ndp[old + new] += dp[old] * cnk.c(old + new, new);
            }
        }
        dp = ndp;
    }
    let mut res = Mod::ZERO;
    for &x in dp.iter() {
        res += x;
    }
    out_line!(res - Mod::ONE);
}

pub(crate) fn run(mut input: Input) -> bool {
    solve(&mut input);
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
