//{"name":"c","group":"Manual","url":"","interactive":false,"timeLimit":2000,"tests":[{"input":"","output":""},{"input":"","output":""},{"input":"","output":""}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::array_2d::Array2D;
use algo_lib::collections::fx_hash_map::FxHashMap;
#[allow(unused)]
use algo_lib::dbg;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::rand::Random;
use algo_lib::misc::rec_function::{Callable, RecursiveFunction};

fn calc_fast(n: usize, max1: usize, max2: usize, max_inv: usize, md: u32) -> u32 {
    let sum = |a: u32, b: u32| -> u32 {
        let s = a + b;
        if s >= md {
            s - md
        } else {
            s
        }
    };

    let mul = |a: u32, b: u32| -> u32 {
        let res = (a as u64 * b as u64) % (md as u64);
        res as u32
    };

    let mut c = Array2D::new(0u32, n + 1, n + 1);
    c[0][0] = 1;
    for i in 1..=n {
        c[i][0] = 1;
        for j in 1..=i {
            c[i][j] = sum(c[i - 1][j - 1], c[i - 1][j]);
        }
    }

    let mut dp = FxHashMap::default();
    let mut ways = vec![0u32; n * n + 1];
    let tmp_ways = ways.clone();
    ways[0] = 1;
    dp.insert((0, 0, 0), ways);
    for c1 in 0..=n {
        for c2 in 0..=n - c1 {
            for c3 in 0..=n - c1 - c2 {
                if let Some(ways) = dp.get(&(c1, c2, c3)) {
                    let ways = ways.clone();
                    for nval in [1, 2, 3] {
                        let nc1 = c1 + if nval == 1 { 1 } else { 0 };
                        let nc2 = c2 + if nval == 2 { 1 } else { 0 };
                        let nc3 = c3 + if nval == 3 { 1 } else { 0 };
                        let key = (nc1, nc2, nc3);
                        let mut extra_inv;
                        if nval == 1 {
                            extra_inv = c2 * 2 + c3;
                        } else if nval == 2 {
                            extra_inv = c3 * 2 + c1;
                        } else {
                            extra_inv = c1 * 2 + c2;
                        }
                        let entry = dp.entry(key).or_insert_with(|| tmp_ways.clone());
                        for my_inv in 0..ways.len() - extra_inv {
                            let new_inv = my_inv + extra_inv;
                            entry[new_inv] = sum(entry[new_inv], ways[my_inv]);
                        }
                    }
                }
            }
        }
    }
    let mut res = 0u32;
    for c1 in 0..=max1 {
        for c2 in 0..=max2 {
            for c3 in 0..=n - c1 - c2 {
                if let Some(ways) = dp.get(&(c1, c2, c3)) {
                    let rc1 = max1 - c1;
                    let rc2 = max2 - c2;
                    if c1 + rc1 + c2 + rc2 + c3 <= n {
                        let rc3 = n - (c1 + rc1 + c2 + rc2 + c3);
                        if let Some(rways) = dp.get(&(rc1, rc2, rc3)) {
                            let sum1 = c1 + c2 + c3;
                            let sum2 = rc1 + rc2 + rc3;
                            let extra_inv = sum1 * sum2;
                            for my_inv in 0..ways.len() {
                                if my_inv + extra_inv <= max_inv {
                                    let more_inv = max_inv - my_inv - extra_inv;
                                    if more_inv < rways.len() {
                                        let mut tmp = mul(ways[my_inv], rways[more_inv]);
                                        tmp = mul(tmp, c[n][sum1]);
                                        // if tmp != 0 {
                                        //     dbg!(c1, c2, c3, rc1, rc2, rc3, my_inv, more_inv, tmp);
                                        // }
                                        res = sum(res, tmp);
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    res
}

fn calc_slow(n: usize, max1: usize, max2: usize, max_inv: usize, md: u32) -> u32 {
    let sum = |a: u32, b: u32| -> u32 {
        let s = a + b;
        if s >= md {
            s - md
        } else {
            s
        }
    };

    let perms = [
        [0, 1, 2],
        [0, 2, 1],
        [1, 0, 2],
        [1, 2, 0],
        [2, 0, 1],
        [2, 1, 0],
    ];
    let mut a: Vec<&[i32; 3]> = vec![];
    RecursiveFunction::new(|f, pos: usize| -> u32 {
        if pos == n {
            let mut cnt1 = 0;
            let mut cnt2 = 0;
            let mut inv = 0;
            for i in 0..n {
                if a[i][0] == 0 {
                    cnt1 += 1;
                } else if a[i][0] == 1 {
                    cnt2 += 1;
                }
            }
            for j in 0..3 {
                for i in 0..n {
                    for k in i + 1..n {
                        if a[i][j] > a[k][j] {
                            inv += 1;
                        }
                    }
                }
            }
            if cnt1 == max1 && cnt2 == max2 && inv == max_inv {
                1
            } else {
                0
            }
        } else {
            let mut r = 0;
            for p in perms.iter() {
                a.push(p);
                r = sum(r, f.call(pos + 1));
                a.pop();
            }
            r
        }
    })
    .call(0)
}

fn solve(input: &mut Input, out: &mut Output) {
    let n = input.usize();
    let max1 = input.usize();
    let max2 = input.usize();
    let max_inv = input.usize();
    let md = input.u32();
    let res = calc_fast(n, max1, max2, max_inv, md);
    out.println(res);
}

fn stress() {
    for it in 13.. {
        dbg!(it);
        const MX: usize = 10;
        let mut rnd = Random::new(it);
        let n = rnd.gen_range(1..MX);
        let max1 = rnd.gen_range(0..n + 1);
        let max2 = rnd.gen_range(0..n + 1 - max1);
        let max_inv = rnd.gen_range(0..2 * n * n + 1);
        let md = 1_000_000_000;
        let res_fast = calc_fast(n, max1, max2, max_inv, md);
        let res_slow = calc_slow(n, max1, max2, max_inv, md);
        if res_fast != res_slow {
            dbg!(n, max1, max2, max_inv, res_fast, res_slow);
            panic!();
        }
    }
}

pub(crate) fn run(mut input: Input, mut output: Output) -> bool {
    solve(&mut input, &mut output);
    output.flush();
    true
}

#[cfg(feature = "local")]
fn main() {
    const PROBLEM_NAME: &str = "c";
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
