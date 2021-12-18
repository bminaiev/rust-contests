//{"name":"D. Без сдачи","group":"Codeforces - Educational Codeforces Round 119 (рейтинговый для Div. 2)","url":"https://codeforces.com/contest/1620/problem/D","interactive":false,"timeLimit":2000,"tests":[{"input":"4\n1\n1337\n3\n10 8 10\n5\n1 2 3 4 5\n3\n7 77 777\n","output":"446\n4\n3\n260\n"}],"testType":"multiNumber","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"DBezSdachi"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::misc::min_max::UpdateMinMax;
use algo_lib::{dbg, out, out_line};

fn need_threes(ones: i64, twos: i64, need: i64) -> i64 {
    let mut res = i64::MAX / 2;
    for use_ones in 0..=ones {
        for use_twos in 0..=twos {
            let more = need - use_ones - use_twos * 2;
            if more >= 0 && more % 3 == 0 {
                res.update_min(more / 3);
            }
        }
    }
    res
}

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.usize();
    let a: Vec<i64> = input.read_vec(n);
    let mut res = i64::MAX;
    const M: i64 = 6;
    for cnt_ones in 0..M {
        for cnt_twos in 0..M {
            let mut required_threes = 0;
            for val in a.iter() {
                required_threes.update_max(need_threes(cnt_ones, cnt_twos, *val));
            }
            res.update_min(required_threes + cnt_ones + cnt_twos);
        }
    }
    out_line!(res);
}

pub(crate) fn run(mut input: Input) -> bool {
    let t = input.read();
    for i in 0usize..t {
        solve(&mut input, i + 1);
    }
    output().flush();
    input.skip_whitespace();
    !input.peek().is_some()
}

//START MAIN
mod tester;

fn main() {
    tester::run_tests();
}
//END MAIN
