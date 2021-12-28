//{"name":"C. Сбалансированные кучки камней","group":"Codeforces - Codeforces Round #763 (Div. 2)","url":"http://codeforces.com/contest/1623/problem/C","interactive":false,"timeLimit":1000,"tests":[{"input":"4\n4\n1 2 10 100\n4\n100 100 100 1\n5\n5 1 1 1 8\n6\n1 2 3 4 5 6\n","output":"7\n1\n1\n3\n"}],"testType":"multiNumber","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"CSbalansirovannieKuchkiKamnei"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::misc::binary_search::{binary_search_first_true, binary_search_last_true};
use algo_lib::{dbg, out, out_line};
use std::cmp::min;

fn can_do(a: &mut [i64], need: i64) -> bool {
    let init_a: Vec<_> = a.to_vec();
    for pos in (2..a.len()).rev() {
        if a[pos] < need {
            return false;
        }
        let more = min(init_a[pos], (a[pos] - need)) / 3;
        a[pos - 1] += more;
        a[pos - 2] += more * 2;
    }
    a[0] >= need && a[1] >= need
}

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.usize();
    let a: Vec<i64> = input.read_vec(n);
    let max_elem = a.iter().max().unwrap();
    let res = binary_search_last_true(0..max_elem + 2, |mid| can_do(&mut a.clone(), mid)).unwrap();
    out_line!(res);
}

pub(crate) fn run(mut input: Input) -> bool {
    let t = input.read();
    for i in 0usize..t {
        solve(&mut input, i + 1);
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
