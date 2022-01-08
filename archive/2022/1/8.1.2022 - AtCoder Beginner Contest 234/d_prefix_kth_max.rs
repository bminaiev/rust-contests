//{"name":"D - Prefix K-th Max","group":"AtCoder - AtCoder Beginner Contest 234","url":"https://atcoder.jp/contests/abc234/tasks/abc234_d","interactive":false,"timeLimit":2000,"tests":[{"input":"3 2\n1 2 3\n","output":"1\n2\n"},{"input":"11 5\n3 7 2 5 11 6 1 9 8 10 4\n","output":"2\n3\n3\n5\n6\n7\n7\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"DPrefixKThMax"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::misc::binary_search::binary_search_first_true;
use algo_lib::seg_trees::fenwick::Fenwick;
use algo_lib::{dbg, out, out_line};

fn solve(input: &mut Input) {
    let n = input.usize();
    let k = input.usize();
    let mut fenw = Fenwick::new_pow2(n);
    for i in 0..n {
        let val = input.usize() - 1;
        fenw.add(val, 1);
        if i >= k - 1 {
            let res = binary_search_first_true(0..n, |mid| fenw.get_suffix_sum(mid) < k as i64);
            out_line!(res);
        }
    }
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
