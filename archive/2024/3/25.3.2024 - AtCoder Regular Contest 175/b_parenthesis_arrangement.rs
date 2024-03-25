//{"name":"B - Parenthesis Arrangement","group":"AtCoder - AtCoder Regular Contest 175","url":"https://atcoder.jp/contests/arc175/tasks/arc175_b","interactive":false,"timeLimit":2000,"tests":[{"input":"3 3 2\n)))(()\n","output":"5\n"},{"input":"1 175 1000000000\n()\n","output":"0\n"},{"input":"7 2622 26092458\n))()((((()()((\n","output":"52187538\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"BParenthesisArrangement"}}}

#[allow(unused)]
use algo_lib::dbg;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;

fn solve(input: &mut Input, out: &mut Output, _test_case: usize) {
    let _n = input.usize();
    let mut swap_cost = input.i64();
    let change_cost = input.i64();
    let mut s = input.string();
    let mut cnt_open = 0;
    for &c in s.iter() {
        if c == b'(' {
            cnt_open += 1;
        }
    }
    let mut res = 0;
    let cnt_close = s.len() - cnt_open;
    if cnt_open < cnt_close {
        let mut to_change = (cnt_close - cnt_open) / 2;
        res += to_change as i64 * change_cost;
        for i in 0..s.len() {
            if s[i] == b')' {
                if to_change > 0 {
                    s[i] = b'(';
                    to_change -= 1;
                }
            }
        }
    } else {
        let mut to_change = (cnt_open - cnt_close) / 2;
        res += to_change as i64 * change_cost;
        for i in (0..s.len()).rev() {
            if s[i] == b'(' {
                if to_change > 0 {
                    s[i] = b')';
                    to_change -= 1;
                }
            }
        }
    }
    swap_cost = swap_cost.min(change_cost * 2);
    let mut balance = 0;
    for &c in s.iter() {
        if c == b'(' {
            balance += 1;
        } else {
            balance -= 1;
        }
        if balance < 0 {
            res += swap_cost;
            balance += 2;
        }
    }
    out.println(res);
}

pub(crate) fn run(mut input: Input, mut output: Output) -> bool {
    solve(&mut input, &mut output, 1);
    output.flush();
    true
}

//START MAIN

fn main() {
    const PROBLEM_NAME: &str = "b_parenthesis_arrangement";
    use algo_lib::tester::helper::*;

    run_tests(PROBLEM_NAME, run);
    // run_single_test(PROBLEM_NAME, run, "1");
    // run_stress(stress);
    // run_locally(run);
}
//END MAIN
