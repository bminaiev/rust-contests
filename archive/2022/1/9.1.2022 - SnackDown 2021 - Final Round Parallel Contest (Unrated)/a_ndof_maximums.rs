//{"name":"AND of Maximums","group":"CodeChef - SnackDown 2021 - Final Round Parallel Contest (Unrated)","url":"https://www.codechef.com/SNCKFP21/problems/ANDOFMAXES","interactive":false,"timeLimit":3000,"tests":[{"input":"3\n5 1\n100 10 1 1000 10000\n6 2\n7 8 9 10 11 12\n5 3\n26447356 268435455 56544987 1000000000 296823278\n","output":"10000\n8\n52087296\n"}],"testType":"multiNumber","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"ANDOfMaximums"}}}

use algo_lib::collections::sparse_table::SparseTableMax;
use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::misc::rec_function::{Callable3, RecursiveFunction3};
use algo_lib::{dbg, out, out_line};
use std::cmp::max;
use std::ops::Range;
use algo_lib::misc::min_max::UpdateMinMax;

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.usize();
    let k = input.usize();
    let a = input.read_vec::<i32>(n);
    let table = SparseTableMax::new(&a);
    let mut res = 0;
    for bit in (0..30).rev() {
        let check_res = res | (1 << bit);
        let mut max_segs = RecursiveFunction3::new(
            |max_segs, range: Range<usize>, can_join_left: bool, can_join_right: bool| -> usize {
                if range.is_empty() {
                    return 0;
                }
                let max_pos = table.find_max_pos(range.clone());
                let max_val = a[max_pos];
                if (max_val & check_res) != check_res {
                    if range.start == 0 && range.end == a.len() {
                        return 0;
                    }
                    let mut res = 0usize;
                    if can_join_left {
                        res.update_max(max_segs.call(
                            max_pos + 1..range.end,
                            can_join_left,
                            can_join_right,
                        ));
                    }
                    if can_join_right {
                        res.update_max(max_segs.call(
                            range.start..max_pos,
                            can_join_left,
                            can_join_right,
                        ));
                    }
                    res
                } else {
                    let mut res = 1;
                    res += max_segs.call(range.start..max_pos, can_join_left, true);
                    res += max_segs.call(max_pos + 1..range.end, true, can_join_right);
                    res
                }
            },
        );
        if max_segs.call(0..n, false, false) >= k {
            res = check_res;
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
    input.peek().is_none()
}

//START MAIN
mod tester;

fn main() {
    tester::run_tests();
}
//END MAIN
