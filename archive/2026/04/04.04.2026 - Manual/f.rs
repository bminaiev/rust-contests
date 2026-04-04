//{"name":"f","group":"Manual","url":"","interactive":false,"timeLimit":2000,"tests":[{"input":"","output":""}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::permutation::Permutation;
#[allow(unused)]
use algo_lib::dbg;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::longest_increasing_subsequence::longest_increasing_subsequence;
use algo_lib::misc::rand::Random;

fn solve_case(incr: &[i32], decr: &[i32]) -> Option<Vec<usize>> {
    if incr[0] != 1 || decr[0] != 1 {
        return None;
    }
    let mut sizes = vec![1];
    const MULT: i64 = 1_000_000;
    let mut a = vec![0];
    let mut iter = 0;
    for i in 1..incr.len() {
        let d_incr = incr[i] - incr[i - 1];
        let d_decr = decr[i] - decr[i - 1];
        if d_incr != 0 && d_incr != 1 {
            return None;
        }
        if d_decr != 0 && d_decr != 1 {
            return None;
        }
        if d_incr == 1 && d_decr == 1 {
            return None;
        }
        if d_incr == 1 {
            sizes.push(1);
            let value = (sizes.len() - 1) as i64 * MULT;
            a.push(value);
        } else if d_decr == 1 {
            let value = -sizes[0];
            a.push(value);
            sizes[0] += 1;
            iter = 0;
        } else {
            assert!(d_incr == 0 && d_decr == 0);
            while iter < sizes.len() && sizes[iter] == decr[i] as i64 {
                iter += 1;
            }
            if iter == sizes.len() {
                assert!((incr[i] as i64 * decr[i] as i64) < i as i64 + 1);
                return None;
            }
            let value = (iter as i64) * MULT - sizes[iter] as i64;
            a.push(value);
            sizes[iter] += 1;
        }
    }
    let mut all_values = a.clone();
    all_values.sort();
    all_values.dedup();
    assert_eq!(all_values.len(), a.len());
    let mut perm = vec![0; a.len()];
    for i in 0..a.len() {
        let pos = all_values.binary_search(&a[i]).unwrap();
        perm[i] = pos + 1;
    }
    Some(perm)
}

fn check(a: &[usize], incr: &[i32], decr: &[i32]) -> bool {
    for i in 0..a.len() {
        let a_small = a[..i + 1].to_vec();
        let max_incr = longest_increasing_subsequence(&a_small);
        let neg_a_small: Vec<usize> = a_small.iter().map(|x| a.len() - *x).collect();
        let max_decr = longest_increasing_subsequence(&neg_a_small);
        if max_incr != incr[i] as usize || max_decr != decr[i] as usize {
            return false;
        }
    }
    true
}

fn stupid(incr: &[i32], decr: &[i32]) -> Option<Vec<usize>> {
    let n = incr.len();
    let mut p = Permutation::new(n);
    loop {
        if check(&p.ids, incr, decr) {
            return Some(p.ids.clone());
        }
        if !p.next() {
            break;
        }
    }
    None
}

fn stress() {
    for it in 1.. {
        dbg!(it);
        let mut rnd = Random::new(it);
        let n = rnd.gen_range(1..10);
        let mut incr = vec![1];
        let mut decr = vec![1];
        for i in 1..n {
            let d_incr = rnd.gen_range(-1..2);
            let d_decr = rnd.gen_range(-1..2);
            incr.push(*incr.last().unwrap() + d_incr);
            decr.push(*decr.last().unwrap() + d_decr);
        }
        if let Some(perm) = solve_case(&incr, &decr) {
            dbg!("OK?");
            let r = check(&perm, &incr, &decr);
            assert!(r);
            let stupid_res = stupid(&incr, &decr);
            assert!(check(&stupid_res.unwrap(), &incr, &decr));
        } else {
            let stupid_res = stupid(&incr, &decr);
            if let Some(stupid_res) = stupid_res {
                assert!(check(&stupid_res, &incr, &decr));
                dbg!(stupid_res);
                unreachable!();
            }
            // dbg!("No solution");
        }
    }
}

fn solve(input: &mut Input, out: &mut Output) {
    let n = input.usize();
    let incr = input.vec::<i32>(n);
    let decr = input.vec::<i32>(n);
    if let Some(perm) = solve_case(&incr, &decr) {
        out.println(perm);
    } else {
        out.println(-1);
    }
}

pub(crate) fn run(mut input: Input, mut output: Output) -> bool {
    solve(&mut input, &mut output);
    output.flush();
    true
}

#[cfg(feature = "local")]
fn main() {
    const PROBLEM_NAME: &str = "f";
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
