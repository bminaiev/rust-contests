//{"name":"F - All Included","group":"AtCoder - AtCoder Beginner Contest 419","url":"https://atcoder.jp/contests/abc419/tasks/abc419_f","interactive":false,"timeLimit":2000,"tests":[{"input":"2 4\nab\nc\n","output":"153\n"},{"input":"2 6\nabc\ncde\n","output":"54\n"},{"input":"5 50\nbbfogggj\nzkbach\needirhyc\nffgd\noemmswj\n","output":"689020583\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"FAllIncluded"}}}

use algo_lib::collections::fx_hash_map::FxHashMap;
#[allow(unused)]
use algo_lib::dbg;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::math::modulo::Mod_998_244_353;

type Mod = Mod_998_244_353;

fn solve(input: &mut Input, out: &mut Output, _test_case: usize) {
    let n = input.usize();
    let l = input.usize();
    let mut a = vec![];
    for _ in 0..n {
        let s = input.string();
        a.push(s);
    }
    let mut prec = vec![vec![]; n];
    for i in 0..n {
        for cur in 0..=a[i].len() {
            let mut line = vec![0; 26];
            for next in b'a'..=b'z' {
                let ncur = if cur == a[i].len() {
                    cur
                } else if a[i][cur] == next {
                    cur + 1
                } else {
                    let mut word = a[i][0..cur].to_vec();
                    word.push(next);
                    while !a[i].starts_with(&word) {
                        word.remove(0);
                    }
                    word.len()
                };
                line[(next - b'a') as usize] = ncur;
            }
            prec[i].push(line);
        }
    }
    let start_state = vec![0; n];
    let mut dp = FxHashMap::default();
    dp.insert(start_state, Mod::ONE);
    for _ in 0..l {
        let mut ndp = FxHashMap::default();
        for (state, cnt) in dp {
            for next in 0..26 {
                let mut nstate = vec![0; n];
                for i in 0..n {
                    let cur = state[i];
                    let ncur = prec[i][cur][next];
                    nstate[i] = ncur;
                }
                *ndp.entry(nstate).or_insert(Mod::ZERO) += cnt;
            }
        }
        dp = ndp;
    }
    let mut need_state = vec![0; n];
    for i in 0..n {
        need_state[i] = a[i].len();
    }
    let res = dp.get(&need_state).copied().unwrap_or(Mod::ZERO);
    out.println(res);
}

pub(crate) fn run(mut input: Input, mut output: Output) -> bool {
    solve(&mut input, &mut output, 1);
    output.flush();
    true
}

//START MAIN

fn main() {
    const PROBLEM_NAME: &str = "f_all_included";
    use algo_lib::tester::helper::*;

    run_tests(PROBLEM_NAME, run);
    // run_single_test(PROBLEM_NAME, run, "1");
    // run_stress(stress);
    // run_locally(run);
}
//END MAIN
