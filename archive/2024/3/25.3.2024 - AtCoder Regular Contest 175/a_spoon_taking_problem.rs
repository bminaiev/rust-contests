//{"name":"A - Spoon Taking Problem","group":"AtCoder - AtCoder Regular Contest 175","url":"https://atcoder.jp/contests/arc175/tasks/arc175_a","interactive":false,"timeLimit":2000,"tests":[{"input":"3\n1 2 3\nL??\n","output":"2\n"},{"input":"3\n1 3 2\nR?L\n","output":"0\n"},{"input":"12\n6 2 9 3 1 4 11 5 12 10 7 8\n????????????\n","output":"160\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"ASpoonTakingProblem"}}}

#[allow(unused)]
use algo_lib::dbg;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::math::modulo::Mod_998_244_353;
use algo_lib::misc::vec_apply_delta::ApplyDelta;

type Mod = Mod_998_244_353;

fn solve(input: &mut Input, out: &mut Output, _test_case: usize) {
    let n = input.usize();
    let p = input.vec::<usize>(n).sub_from_all(1);
    let s = input.string();
    let mut res = Mod::ZERO;
    for &left in [false, true].iter() {
        let mut seen = vec![false; n];
        let mut ways = Mod::ONE;
        for &cur in p.iter() {
            let seen_left = seen[(cur + n - 1) % n];
            let seen_right = seen[(cur + 1) % n];
            if left {
                if seen_right {
                    if s[cur] == b'?' {
                        ways *= Mod::TWO;
                    }
                } else {
                    if s[cur] == b'R' {
                        ways = Mod::ZERO;
                    }
                }
            } else {
                if seen_left {
                    if s[cur] == b'?' {
                        ways *= Mod::TWO;
                    }
                } else {
                    if s[cur] == b'L' {
                        ways = Mod::ZERO;
                    }
                }
            }
            seen[cur] = true;
            // dbg!(left, cur, ways);
        }
        res += ways;
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
    const PROBLEM_NAME: &str = "a_spoon_taking_problem";
    use algo_lib::tester::helper::*;

    run_tests(PROBLEM_NAME, run);
    // run_single_test(PROBLEM_NAME, run, "1");
    // run_stress(stress);
    // run_locally(run);
}
//END MAIN
