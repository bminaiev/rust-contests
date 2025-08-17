//{"name":"A - Rhythm Game","group":"AtCoder - AtCoder Grand Contest 072","url":"https://atcoder.jp/contests/agc072/tasks/agc072_a","interactive":false,"timeLimit":2000,"tests":[{"input":"4\n2\n10\n30 20\n50 10\n2\n9\n30 20\n50 10\n4\n185\n0 40\n0 30\n0 20\n0 10\n5\n1312372641\n141421356 314159265\n237309504 358979323\n880168872 846264338\n4209698078 327950288\n5696718753 419716939\n","output":"Yes\nNo\nYes\nNo\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"ARhythmGame"}}}

#[allow(unused)]
use algo_lib::dbg;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;

#[derive(Clone, Copy, Debug)]
struct Point {
    start_t: i64,
    coord: i64,
}

fn solve(input: &mut Input, out: &mut Output, _test_case: usize) {
    let tc = input.usize();
    for _ in 0..tc {
        let n = input.usize();
        let alive = input.i64();
        let mut a = vec![];
        for i in 0..n {
            let t = input.i64();
            let c = input.i64();
            a.push(Point {
                start_t: t,
                coord: c,
            });
        }
        a.sort_by_key(|a| a.coord + a.start_t);
        let coord = a.iter().map(|a| a.coord).collect::<Vec<_>>();
        let start_t = a.iter().map(|a| a.start_t).collect::<Vec<_>>();
        const MX: i64 = i64::MAX / 2;
        let mut dp = vec![MX; n + 1];
        dp[0] = 0;
        for done in 0..n {
            if dp[done] >= MX {
                continue;
            }
            for n_done in done + 1..=n {
                let will_be_at = (dp[done] + coord[n_done - 1]).max(start_t[n_done - 1]);
                if will_be_at > start_t[n_done - 1] + alive {
                    continue;
                }
                let mut cur_t = will_be_at + coord[n_done - 1];
                let mut ok = true;
                for i in done..n_done - 1 {
                    let will_be_at = (cur_t + coord[i]).max(start_t[i]);
                    if will_be_at > start_t[i] + alive {
                        ok = false;
                        break;
                    }
                    cur_t = will_be_at + coord[i];
                }
                if ok {
                    dp[n_done] = dp[n_done].min(cur_t);
                }
            }
        }
        if dp[n] < MX {
            out.println("Yes");
        } else {
            out.println("No");
        }
    }
}

pub(crate) fn run(mut input: Input, mut output: Output) -> bool {
    solve(&mut input, &mut output, 1);
    output.flush();
    true
}

//START MAIN

fn main() {
    const PROBLEM_NAME: &str = "a_rhythm_game";
    use algo_lib::tester::helper::*;

    run_tests(PROBLEM_NAME, run);
    // run_single_test(PROBLEM_NAME, run, "1");
    // run_stress(stress);
    // run_locally(run);
}
//END MAIN
