//{"name":"C. Раскрась середину","group":"Codeforces - Codeforces Round #768 (Div. 1)","url":"https://codeforces.com/contest/1630/problem/C","interactive":false,"timeLimit":2000,"tests":[{"input":"7\n1 2 1 2 7 4 7\n","output":"2\n"},{"input":"13\n1 2 3 2 1 3 3 4 5 5 5 4 7\n","output":"7\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"CRaskrasSeredinu"}}}

use algo_lib::collections::last_exn::LastExn;
use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::misc::min_max::UpdateMinMax;
use algo_lib::misc::vec_apply_delta::ApplyDelta;
use algo_lib::seg_trees::lazy_seg_tree_max::{MaxValNode, SegTreeMax};
use algo_lib::{dbg, out, out_line};
use std::ops::Range;

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.usize();
    let a = input.read_vec::<usize>(n).sub_from_all(1);
    let mut first = vec![None; n];
    let mut dp = vec![0; n + 1];
    let mut seg_tree = SegTreeMax::new_f(
        n,
        &|pos| MaxValNode {
            pos,
            max_val: i32::MIN,
        },
        (),
    );
    for (pos, &elem) in a.iter().enumerate() {
        dp[pos + 1] = dp[pos];
        if let Some(prev) = first[elem] {
            let can = dp[prev] + (pos - prev - 1);
            dp[pos + 1].update_max(can);
            if prev + 1 < pos {
                let can = seg_tree.get(prev + 1, pos).max_val + (pos as i32) - 1;
                dp[pos + 1].update_max(can as usize);
            }
        } else {
            first[elem] = Some(pos);
        }
        seg_tree.modify(pos, pos + 1, dp[pos + 1] as i32 - (pos as i32));
    }
    out_line!(dp[n]);
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
}
//END MAIN
