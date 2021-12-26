//{"name":"C - Product","group":"AtCoder - AtCoder Beginner Contest 233","url":"https://atcoder.jp/contests/abc233/tasks/abc233_c","interactive":false,"timeLimit":2000,"tests":[{"input":"2 40\n3 1 8 4\n2 10 5\n","output":"2\n"},{"input":"3 200\n3 10 10 10\n3 10 10 10\n5 2 2 2 2 2\n","output":"45\n"},{"input":"3 1000000000000000000\n2 1000000000 1000000000\n2 1000000000 1000000000\n2 1000000000 1000000000\n","output":"0\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"CProduct"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::{dbg, out, out_line};

fn dfs(a: &[Vec<i64>], required: i64) -> u32 {
    if a.is_empty() {
        if required == 1 {
            1
        } else {
            0
        }
    } else {
        a[0].iter()
            .map(|v| {
                if required % v == 0 {
                    dfs(&a[1..], required / v)
                } else {
                    0
                }
            })
            .sum()
    }
}

fn solve(input: &mut Input) {
    let n = input.usize();
    let required = input.i64();
    let a: Vec<Vec<i64>> = (0..n)
        .map(|_| {
            let sz = input.usize();
            input.read_vec(sz)
        })
        .collect();
    out_line!(dfs(&a, required));
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
