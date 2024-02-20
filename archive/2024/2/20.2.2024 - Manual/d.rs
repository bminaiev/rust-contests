//{"name":"d","group":"Manual","url":"","interactive":false,"timeLimit":2000,"tests":[{"input":"","output":""}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"d"}}}

use std::time::Instant;

#[allow(unused)]
use algo_lib::dbg;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::rand::Random;

fn solve_case<const MAGIC: usize>(a: &[i32]) -> i64 {
    let mut cnt = vec![0; MAGIC * 2];
    let mut res = 0;
    for med in 0..a.len() {
        for z in cnt.iter_mut() {
            *z = 0;
        }
        let value = a[med];
        let mut cur = MAGIC as i32;
        cnt[cur as usize] = 1;
        for pos in (0..med).rev() {
            let x = unsafe { *a.get_unchecked(pos) };
            cur += if x < value { -1 } else { 1 };
            if pos & 31 == 0 {
                if cur < 32 || cur > MAGIC as i32 * 2 - 32 {
                    break;
                }
            }
            unsafe {
                *cnt.get_unchecked_mut(cur as usize) += 1;
            }
            // cnt[cur] += 1;
        }
        for i in 0..cnt.len() - 1 {
            cnt[i] += cnt[i + 1];
        }
        cur = MAGIC as i32;
        let mut ways = cnt[cur as usize];
        for pos in med + 1..a.len() {
            let x = unsafe { *a.get_unchecked(pos) };
            cur += if x < value { 1 } else { -1 };
            if pos & 31 == 0 {
                if cur < 32 || cur > MAGIC as i32 * 2 - 32 {
                    break;
                }
            }
            unsafe {
                ways += cnt.get_unchecked(cur as usize);
            }
            {
                // let need_cur = magic * 2 - cur;
                // ways += cnt[cur];
            }
        }
        res += ways * a[med] as i64;
    }
    res
}

fn solve(input: &mut Input, out: &mut Output, _test_case: usize) {
    let n = input.usize();
    let a = input.vec(n);
    let res = solve_case::<1000>(&a);
    out.println(res);
    // let n = 300_000;
    // for it in 0..1 {
    //     dbg!(it);
    //     let mut rnd = Random::new(it + 787);
    //     let perm = rnd.gen_permutation(n);
    //     let perm: Vec<i32> = perm.iter().map(|&x| x as i32).collect();
    //     const M1: usize = 1000;
    //     // const M2: usize = 1300;
    //     let mut start = Instant::now();
    //     let solve1 = solve_case::<M1>(&perm);
    //     dbg!(start.elapsed());
    //     out.println(solve1);
    // }
}

fn stress() {
    // let n = 300_000;
    // for it in 100002.. {
    //     dbg!(it);
    //     let mut rnd = Random::new(it);
    //     let perm = rnd.gen_permutation(n);
    //     let perm: Vec<i32> = perm.iter().map(|&x| x as i32).collect();
    //     const M1: usize = 1000;
    //     const M2: usize = 1300;
    //     let mut start = Instant::now();
    //     let solve1 = solve_case::<M1>(&perm);
    //     dbg!(start.elapsed());
    //     let solve2 = solve_case::<M2>(&perm);
    //     assert_eq!(solve1, solve2);
    // }
    // let mut s = 0;
    // for x in 1..=500 {
    //     for y in 1..=500 {
    //         s += x.min(y);
    //     }
    // }
    // dbg!(s);
    const N: usize = 500;
    let mut dp = vec![1; N + 1];
    for i in 1..=N {
        dp[i] = dp[i / 2] * 4 + i * i * i;
    }
    dbg!(dp[N]);
}

pub(crate) fn run(mut input: Input, mut output: Output) -> bool {
    solve(&mut input, &mut output, 1);
    output.flush();
    true
}

//START MAIN

fn main() {
    const PROBLEM_NAME: &str = "d";
    use algo_lib::tester::helper::*;

    // run_tests(PROBLEM_NAME, run);
    // run_single_test(PROBLEM_NAME, run, "1");
    run_stress(stress);
    // run_locally(run);
}
//END MAIN
