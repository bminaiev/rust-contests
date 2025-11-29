//{"name":"D. Игра с миллиардом игроков","group":"Codeforces - Codeforces Round 1066 (Div. 1 + Div. 2)","url":"https://codeforces.com/contest/2157/problem/D","interactive":false,"timeLimit":2000,"tests":[{"input":"4\n1 1 5\n3\n2 100 100\n50 200\n5 1 10\n5 7 3 9 1\n5 6 10\n9 3 1 7 5\n","output":"0\n150\n12\n13\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

#[allow(unused)]
use algo_lib::dbg;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;

fn solve(input: &mut Input, out: &mut Output) {
    let tc = input.usize();
    for _ in 0..tc {
        let n = input.usize();
        let l = input.i64();
        let r: i64 = input.i64();
        let a = input.vec::<i64>(n);
        let mut inside = vec![];
        let mut used_left = vec![];
        let mut used_right = vec![];
        let mut cnt_left = 0;
        let mut cnt_right = 0;
        for x in a {
            if x <= l {
                used_left.push(x);
                cnt_left += 1;
            } else if x >= r {
                used_right.push(x);
                cnt_right += 1;
            } else {
                inside.push(x);
            }
        }
        inside.sort();
        let mut more_left = 0;
        let mut more_right = 0;
        if cnt_left > cnt_right {
            let more = (cnt_left - cnt_right).min(inside.len());
            more_right += more;
        }
        if cnt_right > cnt_left {
            let more = (cnt_right - cnt_left).min(inside.len());
            more_left += more;
        }
        while more_left + more_right + 2 <= inside.len() {
            more_left += 1;
            more_right += 1;
        }
        for i in 0..more_left {
            used_left.push(inside[i]);
        }
        for i in 0..more_right {
            used_right.push(inside[inside.len() - 1 - i]);
        }
        let mut res = i64::MAX;
        for x in [l, r] {
            let mut cur_res = 0;
            for &pred in used_left.iter() {
                cur_res += x - pred;
            }
            for &pred in used_right.iter() {
                cur_res += pred - x;
            }
            res = res.min(cur_res);
        }
        out.println(res);
    }
}

pub(crate) fn run(mut input: Input, mut output: Output) -> bool {
    solve(&mut input, &mut output);
    output.flush();
    true
}

#[cfg(feature = "local")]
fn main() {
    const PROBLEM_NAME: &str = "d_";
    use algo_lib::tester::helper::*;

    run_tests(PROBLEM_NAME, run);
    // run_single_test(PROBLEM_NAME, run, "1");
    // run_stress(stress);
    // run_locally(run);
}
//END MAIN

#[cfg(not(feature = "local"))]
fn main() {
    let input = algo_lib::io::input::Input::new_stdin();
    let mut output = algo_lib::io::output::Output::new_stdout();
    run(input, output);
}
