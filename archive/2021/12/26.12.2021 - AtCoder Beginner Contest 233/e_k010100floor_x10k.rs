//{"name":"E - Σ[k=0..10^100]floor(X／10^k)","group":"AtCoder - AtCoder Beginner Contest 233","url":"https://atcoder.jp/contests/abc233/tasks/abc233_e","interactive":false,"timeLimit":2000,"tests":[{"input":"1225\n","output":"1360\n"},{"input":"99999\n","output":"111105\n"},{"input":"314159265358979323846264338327950288419716939937510\n","output":"349065850398865915384738153697722542688574377708317\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"EK010100floorX10k"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::misc::pref_sum::PrefSum;
use algo_lib::{dbg, out, out_line};

fn solve(input: &mut Input) {
    let s = input.string_as_vec();
    let digits: Vec<i64> = s.iter().map(|&c| (c - b'0') as i64).collect();
    let digits_pref_sum = digits.pref_sum();
    let mut res = vec![];
    let mut carry = 0i64;
    for pos in (0..digits.len()).rev() {
        let sum = digits_pref_sum[pos + 1] + carry;
        res.push(sum % 10);
        carry = sum / 10;
    }
    while carry != 0 {
        res.push(carry % 10);
        carry /= 10;
    }
    res.reverse();
    for &x in res.iter() {
        out!(x);
    }
    out_line!();
}

pub(crate) fn run(mut input: Input) -> bool {
    solve(&mut input);
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
