//{"name":"C. Communications","group":"Yandex - SNWS-2022, Round 3","url":"https://contest.yandex.ru/snws2022/contest/23959/problems/C/","interactive":false,"timeLimit":2000,"tests":[{"input":"391997\n1234500\n","output":"3\n1\n"}],"testType":"multiEof","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"CCommunications"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::math::modulo::Mod9;
use algo_lib::misc::binary_search::binary_search_last_true;
use algo_lib::misc::digits::digit_from_char;
use algo_lib::misc::min_max::UpdateMinMax;
use algo_lib::strings::hash_string_context::HashContext;
use algo_lib::{dbg, out, out_line};
use std::ops::Range;

type Mod = Mod9;

fn solve_digits(digits: &[i32]) -> usize {
    let mut res = 0;
    let ctx = HashContext::new(digits.len(), Mod::new(10));
    let string = ctx.make_string(digits);
    let n = digits.len();
    for first in 1..n {
        let shorten_range = |range: Range<usize>, len: usize| {
            if range.len() >= len {
                range.end - len..range.end
            } else {
                range
            }
        };
        let max_len = binary_search_last_true(0..n + 1, |len| -> bool {
            let r1 = shorten_range(0..first, len);
            let r2 = shorten_range(first..n, len);
            let hash = string.calc_hash(r1) + string.calc_hash(r2);
            hash == Mod::ZERO || hash == ctx.powers[len]
        })
        .unwrap();
        res.update_max(max_len);
    }
    res
}

fn solve(input: &mut Input, _test_case: usize) {
    let digits: Vec<_> = input
        .string_as_vec()
        .into_iter()
        .map(digit_from_char)
        .collect();
    out_line!(solve_digits(&digits));
}

pub(crate) fn run(mut input: Input) -> bool {
    let mut i = 1usize;
    while input.peek().is_some() {
        solve(&mut input, i);
        i += 1;
    }
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
