//{"name":"A - Zombie","group":"AtCoder - AtCoder Regular Contest 215","url":"https://atcoder.jp/contests/arc215/tasks/arc215_a","interactive":false,"timeLimit":2000,"tests":[{"input":"3\n2 2 20\n4 18\n8 9 14\n0 2 4 6 8 10 12 14\n3 3 140\n120 70 20\n","output":"18\n40\n160\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

#[allow(unused)]
use algo_lib::dbg;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;

fn solve(input: &mut Input, out: &mut Output) {
    let tc = input.usize();
    for _ in 0..tc {
        let n = input.usize();
        let n_bait = input.usize();
        let len = input.i64();
        let mut a = input.vec::<i64>(n);
        a.sort();
        let mut dists = vec![];
        for i in 0..(a.len() - 1) {
            dists.push(a[i + 1] - a[i]);
        }
        dists.sort();
        dists.reverse();
        let mut res = 0;
        let mut already_time = 0;

        for cnt_removed in 0..=dists.len() {
            if cnt_removed > n_bait {
                break;
            }

            let from_left = a[0] + already_time;
            let from_right = len - a[a.len() - 1] + already_time;

            let mut cur_res = already_time;
            let more = n_bait - cnt_removed;
            if more > 0 {
                cur_res += from_left.max(from_right);

                cur_res += (more - 1) as i64 * (from_left + from_right);
            }

            res = res.max(cur_res);

            if cnt_removed != dists.len() {
                already_time += dists[cnt_removed] / 2;
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
    const PROBLEM_NAME: &str = "a_zombie";
    use algo_lib::tester::helper::*;

    run_tests(PROBLEM_NAME, run);
    // run_single_test(PROBLEM_NAME, run, "2");
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
