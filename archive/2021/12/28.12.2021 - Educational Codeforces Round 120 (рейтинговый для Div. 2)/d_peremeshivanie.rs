//{"name":"D. Перемешивание","group":"Codeforces - Educational Codeforces Round 120 (рейтинговый для Div. 2)","url":"http://codeforces.com/contest/1622/problem/D","interactive":false,"timeLimit":2000,"tests":[{"input":"7 2\n1100110\n","output":"16\n"},{"input":"5 0\n10010\n","output":"1\n"},{"input":"8 1\n10001000\n","output":"10\n"},{"input":"10 8\n0010011000\n","output":"1\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"DPeremeshivanie"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::math::combinations::{Combinations, CombinationsFact};
use algo_lib::math::modulo::Mod_998_244_353;
use algo_lib::misc::binary_search::binary_search_last_true;
use algo_lib::misc::pref_sum::PrefSum;
use algo_lib::{dbg, out, out_line};

type Mod = Mod_998_244_353;

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.usize();
    let k = input.i32();
    let a: Vec<_> = input
        .string_as_vec()
        .iter()
        .map(|c| (c - b'0') as i32)
        .collect();
    let a_pref = a.pref_sum();
    if k == 0 || *a_pref.last().unwrap() < k {
        out_line!(1i32);
        return;
    }
    let mut res = Mod::ONE;
    let cnk = CombinationsFact::new(n + 1);
    for first_change in 0..n {
        let last_ok_pos = binary_search_last_true(first_change..n, |pos| {
            a_pref[pos + 1] - a_pref[first_change] <= k
        })
        .unwrap();
        let have_ones = (a_pref[last_ok_pos + 1] - a_pref[first_change])
            - (if a[first_change] == 0 { 1 } else { 0 });
        let len = last_ok_pos - first_change;
        res += cnk.c(len, have_ones as usize);
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
    tester::run_single_test("1");
}
//END MAIN
