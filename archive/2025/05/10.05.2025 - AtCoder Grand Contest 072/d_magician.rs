//{"name":"D - Magician","group":"AtCoder - AtCoder Grand Contest 072","url":"https://atcoder.jp/contests/agc072/tasks/agc072_d","interactive":false,"timeLimit":4000,"tests":[],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"DMagician"}}}

use algo_lib::collections::array_2d::Array2D;
#[allow(unused)]
use algo_lib::dbg;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::rand::Random;

fn solve(input: &mut Input, out: &mut Output, _test_case: usize) {}

fn stress3() {
    let n = 3;
    let mut cards = vec![0; 1 << (n * 2)];
    let mut rnd = Random::new(123);
    loop {
        for i in 0..cards.len() {
            cards[i] = rnd.gen(0..n);
        }
        let mut pow3 = vec![1; 2 * n + 1];
        for i in 1..pow3.len() {
            pow3[i] = pow3[i - 1] * 3;
        }
        let mut dp = Array2D::new(false, n, pow3[n * 2]);
        for mask in (0usize..pow3[n * 2]).rev() {
            let mut state = vec![0; 2 * n];
            let mut cur = mask;
            for i in 0..2 * n {
                state[i] = cur % 3;
                cur /= 3;
            }

            let mut cnt = 0;
            for i in 0..2 * n {
                if state[i] == 1 {
                    cnt += 1;
                }
            }
            if cnt == n {
                let mut binary_mask = 0;
                for i in 0..2 * n {
                    if state[i] == 1 {
                        binary_mask += 1 << i;
                    }
                }
                let value = cards[binary_mask];
                dp[value][mask] = true;
            }
            if cnt < n {
                for value in 0..n {
                    let mut ok = true;
                    for i in 0..2 * n {
                        for j in i + 1..2 * n {
                            if state[i] == 0 && state[j] == 0 {
                                let nstate1 = mask + pow3[i] + 2 * pow3[j];
                                let nstate2 = mask + pow3[j] + 2 * pow3[i];
                                if !dp[value][nstate1] && !dp[value][nstate2] {
                                    ok = false;
                                    break;
                                }
                            }
                        }
                    }
                    if ok {
                        dp[value][mask] = true;
                    }
                }
            }
        }
        let mut cnt_good = 0;
        for value in 0..n {
            if dp[value][0] {
                cnt_good += 1;
            }
        }
        dbg!(cnt_good);
        if cnt_good == n {
            for mask in 0usize..(1 << (2 * n)) {
                if mask.count_ones() as usize == n {
                    dbg!(mask, cards[mask]);
                }
            }
            break;
        }
    }
}

fn stress() {
    let n = 5;
    let mut cards = vec![0; 1 << (n * 2)];
    let mut rnd = Random::new(123);
    for i in 0..cards.len() {
        cards[i] = rnd.gen(0..n);
    }
    for game in 1.. {
        dbg!(game);
        let mut state = vec![2; n * 2];
        let value = rnd.gen(0..n);
        for i in 0..n {
            let x = {
                let mut z = n * 2;
                while z == n * 2 || state[z] != 2 {
                    z = rnd.gen(0..n * 2);
                }
                z
            };
            let y = {
                let mut z = n * 2;
                while z == n * 2 || state[z] != 2 || z == x {
                    z = rnd.gen(0..n * 2);
                }
                z
            };
            state[x] = 0;
            state[y] = 1;
            let mut cnt0 = 0;
            for mask in 0..(1 << (2 * n)) {
                let mut good_mask = true;
                for j in 0..2 * n {
                    if state[j] != 2 {
                        if (mask >> j) & 1 != state[j] {
                            good_mask = false;
                            break;
                        }
                    }
                }
                if good_mask {
                    if cards[mask] == value {
                        cnt0 += 1;
                    }
                }
            }
            let mut cnt1 = 0;
            state[x] = 1;
            state[y] = 0;
            for mask in 0..(1 << (2 * n)) {
                let mut good_mask = true;
                for j in 0..2 * n {
                    if state[j] != 2 {
                        if (mask >> j) & 1 != state[j] {
                            good_mask = false;
                            break;
                        }
                    }
                }
                if good_mask {
                    if cards[mask] == value {
                        cnt1 += 1;
                    }
                }
            }
            if cnt0 == 0 && cnt1 == 0 {
                if i == n - 1 {
                    dbg!("NOT OK", i, n);
                    let mut mask = 0;
                    for j in 0..(2 * n) {
                        if state[j] == 1 {
                            mask += 1 << j;
                        }
                    }
                    cards[mask] = value;
                }
            }
            if cnt0 > cnt1 {
                state[x] = 0;
                state[y] = 1;
            }
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
    const PROBLEM_NAME: &str = "d_magician";
    use algo_lib::tester::helper::*;

    // run_tests(PROBLEM_NAME, run);
    // run_single_test(PROBLEM_NAME, run, "1");
    run_stress(stress);
    // run_locally(run);
}
//END MAIN
