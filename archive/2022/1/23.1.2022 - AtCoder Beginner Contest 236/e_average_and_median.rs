//{"name":"E - Average and Median","group":"AtCoder - AtCoder Beginner Contest 236","url":"https://atcoder.jp/contests/abc236/tasks/abc236_e","interactive":false,"timeLimit":2000,"tests":[{"input":"6\n2 1 2 1 1 10\n","output":"4\n2\n"},{"input":"7\n3 1 4 1 5 9 2\n","output":"5.250000000\n4\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"EAverageAndMedian"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::misc::binary_search::binary_search_last_true;
use algo_lib::misc::binary_search_float::float_binary_search_first_true;
use algo_lib::misc::num_traits::HasConstants;
use algo_lib::misc::ord_f64::OrdF64;
use algo_lib::{dbg, out, out_line};
use std::cmp::max;

fn max_average(a: &[i64]) -> OrdF64 {
    float_binary_search_first_true(OrdF64::ZERO, OrdF64(1.1e9), 120, |mid| -> bool {
        let mut used_last = OrdF64::ZERO;
        let mut not_used = OrdF64::ZERO;
        for &elem in a.iter() {
            let delta = (OrdF64(elem as f64)) - mid;
            let next_used_last = max(not_used, used_last) + delta;
            let next_not_used = used_last;
            used_last = next_used_last;
            not_used = next_not_used;
        }
        used_last < OrdF64::ZERO || not_used < OrdF64::ZERO
    })
}

fn max_median(a: &[i64]) -> i64 {
    binary_search_last_true(0..1_000_000_010, |mid| -> bool {
        let mut used_last = 0;
        let mut not_used = 0;
        for &elem in a.iter() {
            let delta = if elem >= mid { 1 } else { -1 };
            let next_used_last = max(not_used, used_last) + delta;
            let next_not_used = used_last;
            used_last = next_used_last;
            not_used = next_not_used;
        }
        used_last > 0 || not_used > 0
    })
    .unwrap()
}

fn solve(input: &mut Input) {
    let n = input.usize();
    let a: Vec<i64> = input.read_vec(n);
    out_line!(max_average(&a));
    out_line!(max_median(&a));
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
