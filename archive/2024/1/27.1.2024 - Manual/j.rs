//{"name":"j","group":"Manual","url":"","interactive":false,"timeLimit":2000,"tests":[{"input":"","output":""}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"j"}}}

use std::collections::HashSet;

use algo_lib::collections::array_2d::Array2D;
#[allow(unused)]
use algo_lib::dbg;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::rand::Random;

const MX: usize = 1900;

fn make_full(mut a: Vec<u64>, max_m: usize) -> Vec<u64> {
    let mut seen = HashSet::new();
    for &x in a.iter() {
        seen.insert(x);
    }
    let mut it = 0;
    while it < a.len() {
        if seen.len() > max_m {
            break;
        }
        for it2 in 0..it {
            let x = a[it];
            let y = a[it2];
            for &z in [x | y, x & y].iter() {
                if !seen.contains(&z) {
                    seen.insert(z);
                    a.push(z);
                }
            }
        }
        it += 1;
    }
    a
}

fn add_one(a: &[u64]) -> Vec<u64> {
    let mut res = vec![0];
    for &x in a.iter() {
        res.push(x * 2 + 1);
    }
    res
}

fn mul_two(a: &[u64]) -> Vec<u64> {
    let mut res = vec![];
    for &x in a.iter() {
        res.push(x * 2);
        res.push(x * 2 + 1);
    }
    res
}

fn check_n(n: usize, saw_sz: &mut [Option<Vec<u64>>]) {
    let max_m = n * (n + 1) / 2;

    let mut tot_saw = 0;
    for i in 0..=max_m {
        if saw_sz[i].is_some() {
            tot_saw += 1;
        }
    }

    for it in 1.. {
        // dbg!(it);
        // if it % 1_000_000 == 0 || it % 19 == 0 {
        //     dbg!(it, tot_saw, max_m - 1);
        // }
        let mut rnd = Random::new(it);
        let mx = 2u64.pow(n as u32) - 1;
        let mut queue = vec![0, mx];
        let mut base = queue.clone();
        loop {
            queue = make_full(queue, MX);
            if queue.len() > MX {
                break;
            }
            let cur_sz = queue.len();
            // {
            //     let base2 = add_one(&base);
            //     let next_queue = make_full(base2, usize::MAX);
            //     assert_eq!(next_queue.len(), cur_sz + 1);
            // }
            if saw_sz[cur_sz].is_none() {
                saw_sz[cur_sz] = Some(base.clone());
                if cur_sz <= max_m {
                    tot_saw += 1;
                }
                // dbg!(cur_sz, tot_saw, max_m - 1, base.len());
                if tot_saw == max_m - 1 {
                    // eprintln!("FOUND  ALL!");
                    return;
                }
            }
            if queue.len() == (mx + 1) as usize {
                break;
            }
            loop {
                let x = rnd.gen_u64() % mx;
                if !queue.contains(&x) {
                    queue.push(x);
                    base.push(x);
                    break;
                }
            }
        }
    }
}

fn calc() -> Array2D<Option<Vec<u64>>> {
    let mut seen = Array2D::new(None, 61, MX);
    for n in 2..=60 {
        // eprintln!("N = {n}");
        check_n(n, &mut seen[n]);
        if n < 60 {
            for sz in 0..seen[n].len() {
                if let Some(base) = &seen[n][sz].clone() {
                    {
                        let nsz = sz + 1;
                        if nsz < seen[n + 1].len() && seen[n + 1][nsz].is_none() {
                            let nbase = add_one(base);
                            seen[n + 1][nsz] = Some(nbase);
                        }
                    }
                    {
                        let nsz = sz * 2;
                        if nsz < seen[n + 1].len() && seen[n + 1][nsz].is_none() {
                            let nbase = mul_two(base);
                            seen[n + 1][nsz] = Some(nbase);
                        }
                    }
                }
            }
        }
    }
    seen
}

fn stress() {

    // for n in 2..=60 {
    //     let max_m = n * (n + 1) / 2;
    //     for sz in 2..=max_m {
    //         eprintln!("Checking {n} {sz}");
    //         let base = seen[n][sz].clone().unwrap();
    //         let full = make_full(base, usize::MAX);
    //         assert_eq!(full.len(), sz);
    //     }
    // }
}

fn solve(input: &mut Input, out: &mut Output, _test_case: usize) {
    let res = calc();
    let tc = input.usize();
    for _ in 0..tc {
        let n = input.usize();
        let m = input.usize();
        let base = res[n][m].clone().unwrap();
        let mut full = make_full(base, usize::MAX);
        full.sort();
        out.println(full);
    }
}

pub(crate) fn run(mut input: Input, mut output: Output) -> bool {
    solve(&mut input, &mut output, 1);
    output.flush();
    true
}

//START MAIN

fn main() {
    const PROBLEM_NAME: &str = "j";
    use algo_lib::tester::helper::*;

    run_tests(PROBLEM_NAME, run);
    // run_single_test(PROBLEM_NAME, run, "1");
    // run_stress(stress);
    // run_locally(run);
}
//END MAIN
