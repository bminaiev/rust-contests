use std::{todo, unreachable};

use algo_lib::collections::array_2d::Array2D;
use algo_lib::collections::permutation::Permutation;
#[allow(unused)]
use algo_lib::dbg;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;

fn solve(input: &mut Input, out: &mut Output) {
    let tc = input.usize();
    for _ in 0..tc {
        let n = input.usize();
        let x = input.usize();
        if let Some(res) = go_one2(n, x) {
            let full = restore_full(&res, x);
            for i in 0..n {
                out.println(full[i].to_vec());
            }
        } else {
            out.println("-1");
        }
    }
}

fn is_good(a: &Array2D<usize>, x: usize) -> bool {
    let n = a.rows();
    for i in 1..n {
        for j in 1..n {
            let cur = a[i - 1][j - 1] ^ a[i][j - 1] ^ a[i - 1][j] ^ x;
            if cur != a[i][j] {
                return false;
            }
        }
    }
    for i in 0..n {
        let mut seen = vec![false; n];
        for j in 0..n {
            if a[i][j] >= n {
                return false;
            }
            if seen[a[i][j]] {
                return false;
            }
            seen[a[i][j]] = true;
        }
        let mut seen = vec![false; n];
        for j in 0..n {
            if a[j][i] >= n {
                return false;
            }
            if seen[a[j][i]] {
                return false;
            }
            seen[a[j][i]] = true;
        }
    }
    true
}

fn go_one(n: usize, x: usize) -> Vec<Vec<usize>> {
    let mut p1 = Permutation::new(n);
    let mut res = vec![];
    loop {
        // let mut p2 = Permutation::new(n);
        // loop {
        let mut a = Array2D::new(0, n, n);
        for i in 0..n {
            a[0][i] = p1[i];
            a[i][0] = p1[i];
        }
        // if p1[0] == p2[0] {
        for i in 1..n {
            for j in 1..n {
                let cur = a[i - 1][j - 1] ^ a[i][j - 1] ^ a[i - 1][j] ^ x;
                a[i][j] = cur;
            }
        }
        if is_good(&a, x) {
            res.push(p1.to_vec());
            return res;
        }
        // }
        //     if !p2.next() {
        //         break;
        //     }
        // }
        if !p1.next() {
            break;
        }
    }
    res
}

fn restore_full(a: &[usize], x: usize) -> Array2D<usize> {
    let n = a.len();
    let mut res = Array2D::new(0, n, n);
    for i in 0..n {
        res[0][i] = a[i];
        res[i][0] = a[i];
    }
    for i in 1..n {
        for j in 1..n {
            let cur = res[i - 1][j - 1] ^ res[i][j - 1] ^ res[i - 1][j] ^ x;
            res[i][j] = cur;
        }
    }
    assert!(is_good(&res, x));
    res
}

fn go_one2(n: usize, x: usize) -> Option<Vec<usize>> {
    let mut p = vec![0; n];
    for i in 0..n {
        p[i] = i;
    }
    if !n.is_power_of_two() {
        return None;
    }
    if x == n {
        return None;
    }
    if x == 1 {
        if n == 2 {
            return None;
        }
        let mut res = vec![0; n];
        for i in 0..n {
            res[i] = i;
        }
        for i in (0..n).step_by(4) {
            res.swap(i + 1, i + 2);
        }
        return Some(res);
    }
    if x % 2 == 0 {
        let mut res = vec![0; n];
        for i in 0..n {
            res[i] = i;
        }
        return Some(res);
    }
    for mask_pw in 0.. {
        let mask_len = 1 << mask_pw;
        let mut p2 = p.clone();
        let mut mask_full = vec![];
        for i in 0..n / 2 {
            if (i % (mask_len * 2)) < mask_len {
                mask_full.push(1);
                let pos = i * 2;
                p2.swap(pos, pos + 1);
            } else {
                mask_full.push(0);
            }
        }
        let mut a = Array2D::new(0, n, n);
        for i in 0..n {
            a[0][i] = p2[i];
            a[i][0] = p2[i];
        }
        for i in 1..n {
            for j in 1..n {
                let cur = a[i - 1][j - 1] ^ a[i][j - 1] ^ a[i - 1][j] ^ x;
                a[i][j] = cur;
            }
        }
        if is_good(&a, x) {
            // dbg!(x, mask_full);
            return Some(p2);
        }
    }
    None
}

fn stress() {
    for n in 1..2555 {
        dbg!(n);
        for x in 0..=n {
            let res = go_one2(n, x);
            if let Some(res) = res {
                // dbg!(n, x, res);
                let a = restore_full(&res, x);
            }
        }
    }
    // let res = go_one(n, x);
    // dbg!(res.len());
    // for a in res {
    //     dbg!(a);
    // }
}

pub(crate) fn run(mut input: Input, mut output: Output) -> bool {
    solve(&mut input, &mut output);
    output.flush();
    true
}

#[cfg(feature = "local")]
fn main() {
    const PROBLEM_NAME: &str = "xor";
    #[allow(unused)]
    use algo_lib::misc::dragon::run_dragon;
    use algo_lib::tester::helper::*;

    run_tests(PROBLEM_NAME, run);
    // run_single_test(PROBLEM_NAME, run, "1");
    // run_stress(stress);
    // run_locally(run);
    // run_dragon(solve, "W", 1..2);
}
//END MAIN

#[cfg(not(feature = "local"))]
fn main() {
    let input = algo_lib::io::input::Input::new_stdin();
    let mut output = algo_lib::io::output::Output::new_stdout();
    run(input, output);
}
