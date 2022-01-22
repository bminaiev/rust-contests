//{"name":"B - Dividing Subsequence","group":"AtCoder - AtCoder Regular Contest 133","url":"https://atcoder.jp/contests/arc133/tasks/arc133_b","interactive":false,"timeLimit":5000,"tests":[{"input":"4\n3 1 4 2\n4 2 1 3\n","output":"2\n"},{"input":"5\n1 2 3 4 5\n5 4 3 2 1\n","output":"3\n"},{"input":"10\n4 3 1 10 9 2 8 6 5 7\n9 6 5 4 2 3 8 10 1 7\n","output":"6\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"BDividingSubsequence"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::misc::binary_search::binary_search_first_true;
use algo_lib::misc::longest_increasing_subsequence::longest_increasing_subsequence;
use algo_lib::misc::min_max::UpdateMinMax;
use algo_lib::{dbg, out, out_line};

fn solve(input: &mut Input) {
    let n = input.usize();
    let a = input.read_vec::<usize>(n);
    let b = input.read_vec::<usize>(n);
    let mut pos_in_b = vec![0; n + 1];
    for (pos, &val) in b.iter().enumerate() {
        pos_in_b[val] = pos;
    }
    let mut all_positions = vec![];
    for &x in a.iter() {
        let mut all_pos = vec![];
        for val in (x..=n).step_by(x) {
            all_pos.push(pos_in_b[val] + 1);
        }
        all_pos.sort();
        all_pos.reverse();
        all_positions.append(&mut all_pos);
    }
    let max_len = longest_increasing_subsequence(&all_positions);
    out_line!(max_len);
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
