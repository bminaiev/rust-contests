//{"name":"D. Новогодний концерт","group":"Codeforces - Codeforces Round #769 (Div. 2)","url":"https://codeforces.com/contest/1632/problem/D","interactive":false,"timeLimit":3000,"tests":[{"input":"1\n1\n","output":"1\n"},{"input":"3\n1 4 2\n","output":"1 1 2\n"},{"input":"7\n2 12 4 8 18 3 6\n","output":"0 1 1 1 2 2 2\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"DNovogodniiKontsert"}}}

use algo_lib::collections::array_2d::Array2D;
use algo_lib::collections::sparse_table_gcd::SparseTableGCD;
use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::math::gcd::gcd;
use algo_lib::misc::binary_search::binary_search_first_true;
use algo_lib::{dbg, out, out_line};

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.usize();
    let a = input.read_vec::<i64>(n);
    let gcd_table = SparseTableGCD::new(&a);
    let mut segs = vec![];
    for left in 0..n {
        let len = binary_search_first_true(1..n - left, |len| -> bool {
            gcd_table.query(left..left + len) <= len as i64
        });
        if gcd_table.query(left..left + len) == len as i64 {
            segs.push(left..left + len);
        }
    }
    segs.sort_by_key(|s| s.end);
    let mut need_remove = vec![false; n];
    let mut first_not_covered = 0;
    for seg in segs.iter() {
        if seg.start < first_not_covered {
            continue;
        }
        need_remove[seg.end - 1] = true;
        first_not_covered = seg.end;
    }
    let mut res = 0;
    for pos in 0..n {
        if need_remove[pos] {
            res += 1;
        }
        out!(res, "");
    }
    out_line!();
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
    // tester::run_single_test("1");
    // tester::run_locally();
}
//END MAIN
