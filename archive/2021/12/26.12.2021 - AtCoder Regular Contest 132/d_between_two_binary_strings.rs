//{"name":"D - Between Two Binary Strings","group":"AtCoder - AtCoder Regular Contest 132","url":"https://atcoder.jp/contests/arc132/tasks/arc132_d","interactive":false,"timeLimit":2000,"tests":[{"input":"2 3\n10110\n01101\n","output":"2\n"},{"input":"4 2\n000011\n110000\n","output":"4\n"},{"input":"12 26\n01110111101110111101001101111010110110\n10011110111011011001111011111101001110\n","output":"22\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"DBetweenTwoBinaryStrings"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::misc::min_max::UpdateMinMax;
use algo_lib::{dbg, out, out_line};
use std::cmp::{max, min};

fn solve(input: &mut Input) {
    let zeros = input.usize();
    let ones = input.usize();
    let n = zeros + ones;
    if ones == 0 || zeros == 0 {
        out_line!(n - 1);
        return;
    }
    let mut read = || {
        let s = input.string_as_vec();
        let mut res = vec![];
        for (pos, &c) in s.iter().enumerate() {
            if c == b'1' {
                res.push(pos);
            }
        }
        res
    };
    let first = read();
    let second = read();
    let to: Vec<_> = first
        .iter()
        .zip(second.iter())
        .map(|(x, y)| max(*x, *y))
        .collect();
    let from: Vec<_> = first
        .iter()
        .zip(second.iter())
        .map(|(x, y)| min(*x, *y))
        .collect();
    let mut smallest_changes = n - 1;
    for &start_from_one in [false, true].iter() {
        if start_from_one && from[0] > 0 {
            continue;
        }
        let mut changes = if start_from_one { 0 } else { 1 };
        let mut last_one_pos = if start_from_one { 0 } else { to[0] };
        for i in 1..from.len() {
            if from[i] <= last_one_pos + 1 {
                last_one_pos += 1;
            } else {
                changes += 2;
                last_one_pos = to[i];
            }
        }
        if last_one_pos != n - 1 {
            changes += 1;
        }
        smallest_changes.update_min(changes);
    }
    let max_equal = n - 1 - smallest_changes;
    out_line!(max_equal);
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
