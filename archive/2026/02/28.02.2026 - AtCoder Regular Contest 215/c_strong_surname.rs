//{"name":"C - Strong Surname","group":"AtCoder - AtCoder Regular Contest 215","url":"https://atcoder.jp/contests/arc215/tasks/arc215_c","interactive":false,"timeLimit":2000,"tests":[{"input":"3\n4\n3 4 4\n2 2 2\n4 3 3\n1 1 1\n3\n2 1 1\n1 2 1\n1 1 2\n3\n2 2 2\n2 2 2\n1 1 1\n","output":"2\n3\n2\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

#[allow(unused)]
use algo_lib::dbg;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;

fn solve(input: &mut Input, out: &mut Output) {
    let tc = input.usize();
    for _ in 0..tc {
        let n = input.usize();
        let mut a = vec![];
        for _ in 0..n {
            a.push(input.vec::<usize>(3));
        }
        let mut orders = vec![];
        for id in 0..3 {
            let mut order: Vec<usize> = (0..n).collect();
            order.sort_by_key(|i| a[*i][id]);
            order.reverse();
            orders.push(order);
        }
        let mut iters = [0; 3];
        let mut mins = [0; 3];
        for i in 0..n {
            for j in 0..3 {
                mins[j] = mins[j].max(a[i][j]);
            }
        }
        let mut can_win = vec![false; n];
        loop {
            let mut changed = false;
            for i in 0..3 {
                while iters[i] < n && a[orders[i][iters[i]]][i] >= mins[i] {
                    let who = orders[i][iters[i]];
                    can_win[who] = true;
                    for j in 0..3 {
                        mins[j] = mins[j].min(a[who][j]);
                    }
                    iters[i] += 1;
                    changed = true;
                }
            }
            if !changed {
                break;
            }
        }
        let mut res = 0;
        for i in 0..n {
            if can_win[i] {
                res += 1;
            }
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
    const PROBLEM_NAME: &str = "c_strong_surname";
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
