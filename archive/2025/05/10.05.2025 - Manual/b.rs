//{"name":"b","group":"Manual","url":"","interactive":false,"timeLimit":2000,"tests":[{"input":"","output":""}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"b"}}}

#[allow(unused)]
use algo_lib::dbg;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::rand::Random;

const N: usize = 12;
const M: usize = 6;

fn convert_state(s: &[usize; M], max_state: &[usize; M]) -> usize {
    let mut res = 0;
    for i in 1..M {
        res *= max_state[i] + 1;
        res += s[i];
    }
    res
}

fn unconvert_state(mut state: usize, max_state: &[usize; M]) -> [usize; M] {
    let mut res = [0; M];
    for i in (1..M).rev() {
        res[i] = state % (max_state[i] + 1);
        state /= max_state[i] + 1;
    }
    res
}

fn solve_case(b: &[usize], precalc: &[Vec<[usize; M]>]) -> usize {
    let mut a = b.to_vec();
    let mut res = 0;
    for i in 1..6 {
        let use_cnt = a[i].min(a[N - i]);
        res += use_cnt;
        a[i] -= use_cnt;
        a[N - i] -= use_cnt;
    }
    res += a[6] / 2;
    a[6] %= 2;
    let mut start_state = [0; M];
    for i in 1..6 {
        start_state[i] = a[i];
    }
    // dbg!(start_state, res);
    let mut n_states = 1;
    for i in 1..6 {
        n_states *= a[i] + 1;
    }

    let start_state_id = convert_state(&start_state, &start_state);
    assert_eq!(start_state_id, n_states - 1);

    const INF: usize = usize::MAX / 2;
    let mut dp = vec![INF; n_states];
    dp[start_state_id] = 0;

    for biggest in 6..=N {
        let left = N - biggest;
        for _ in 0..a[biggest] {
            res += 1;
            // dbg!("checking", biggest);
            for state_id in 0..dp.len() {
                let cost = dp[state_id];
                if cost == INF {
                    continue;
                }
                let state = unconvert_state(state_id, &start_state);
                for sub in precalc[left].iter() {
                    let mut ok = true;
                    for i in 1..6 {
                        if sub[i] > state[i] {
                            ok = false;
                            break;
                        }
                    }
                    if !ok {
                        continue;
                    }
                    let mut new_state = [0; M];
                    for i in 1..6 {
                        new_state[i] = state[i] - sub[i];
                    }
                    // dbg!("CAN DO", new_state);
                    let new_state_id = convert_state(&new_state, &start_state);
                    assert!(new_state_id < state_id);
                    dp[new_state_id] = dp[new_state_id].min(cost);
                    // dbg!(new_state_id, dp[new_state_id]);
                }
            }
        }
    }
    // dbg!("final?");
    for state_id in (0..dp.len()).rev() {
        let cost = dp[state_id];
        if cost == INF {
            continue;
        }
        let state = unconvert_state(state_id, &start_state);
        // dbg!(state_id, state, cost, res);
        for sub in precalc[12].iter() {
            let mut ok = true;
            for i in 1..6 {
                if sub[i] > state[i] {
                    ok = false;
                    break;
                }
            }
            if !ok {
                continue;
            }
            let mut new_state = [0; M];
            for i in 1..6 {
                new_state[i] = state[i] - sub[i];
            }
            // dbg!(new_state);
            let new_state_id = convert_state(&new_state, &start_state);
            dp[new_state_id] = dp[new_state_id].min(cost + 1);
        }
    }
    res + dp[0]
}

fn gen_precalc() -> Vec<Vec<[usize; M]>> {
    let mut res = vec![vec![]; N + 1];
    for cnt1 in 0..=12 {
        for cnt2 in 0..=6 {
            for cnt3 in 0..=4 {
                for cnt4 in 0..=3 {
                    for cnt5 in 0..=2 {
                        let mut a = [0; M];
                        a[1] = cnt1;
                        a[2] = cnt2;
                        a[3] = cnt3;
                        a[4] = cnt4;
                        a[5] = cnt5;
                        let sum = cnt1 + cnt2 * 2 + cnt3 * 3 + cnt4 * 4 + cnt5 * 5;
                        if sum != 0 {
                            for it in sum..=N {
                                res[it].push(a);
                            }
                        }
                    }
                }
            }
        }
    }
    res
}

fn solve(input: &mut Input, out: &mut Output, _test_case: usize) {
    let prec = gen_precalc();
    let tc = input.usize();
    for t in 0..tc {
        // dbg!("!!", t);
        let n = input.usize();
        let mut cnt = [0; N + 1];
        for _ in 0..n {
            let x = input.usize();
            cnt[x] += 1;
        }
        out.println(solve_case(&cnt, &prec));
    }
}

fn stress() {
    let prec = gen_precalc();
    for tc in 0..100 {
        let mut sum = 0;
        for it in 1..100 {
            // dbg!(it);
            let mut rnd = Random::new(it + tc * 123123);
            let n = 1000;
            let mut cnt = [0; N + 1];
            for _ in 0..n {
                let x = rnd.gen(1..N + 1);
                cnt[x] += 1;
            }
            sum += solve_case(&cnt, &prec);
        }
        dbg!(sum);
    }
}

pub(crate) fn run(mut input: Input, mut output: Output) -> bool {
    solve(&mut input, &mut output, 1);
    output.flush();
    true
}

//START MAIN

fn main() {
    const PROBLEM_NAME: &str = "b";
    use algo_lib::tester::helper::*;

    run_tests(PROBLEM_NAME, run);
    // run_single_test(PROBLEM_NAME, run, "2");
    // run_stress(stress);
    // run_locally(run);
}
//END MAIN
