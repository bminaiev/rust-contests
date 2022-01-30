//{"name":"F - |LIS| = 3","group":"AtCoder - AtCoder Beginner Contest 237","url":"https://atcoder.jp/contests/abc237/tasks/abc237_f","interactive":false,"timeLimit":2000,"tests":[{"input":"4 5\n","output":"135\n"},{"input":"3 4\n","output":"4\n"},{"input":"111 3\n","output":"144980434\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"FLIS3"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::math::modulo::Mod_998_244_353;
use algo_lib::misc::binary_search::binary_search_first_true;
use algo_lib::misc::min_max::UpdateMinMax;
use algo_lib::{dbg, out, out_line};
use std::collections::HashMap;

type Mod = Mod_998_244_353;

fn solve(input: &mut Input) {
    let n = input.usize();
    let m = input.usize();

    let mut dp = HashMap::new();
    dp.insert(vec![], Mod::ONE);
    for _ in 0..n {
        let mut ndp = HashMap::new();
        for (key, val) in dp.iter() {
            for next_val in 0..m {
                let first_pos = binary_search_first_true(0..key.len(), |pos| key[pos] >= next_val);
                let mut next_key = key.clone();
                if first_pos == next_key.len() {
                    next_key.push(next_val);
                } else {
                    next_key[first_pos].update_min(next_val);
                }
                if next_key.len() <= 3 {
                    *ndp.entry(next_key).or_default() += *val;
                }
            }
        }
        dp = ndp;
    }
    let mut res = Mod::ZERO;
    for (key, val) in dp.iter() {
        if key.len() == 3 {
            res += *val;
        }
    }
    out_line!(res);
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
    // tester::run_single_test("1");
}
//END MAIN
