#[allow(unused)]
use algo_lib::dbg;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;

fn solve_case_fast(n: usize) -> Vec<bool> {
    if n < 10 {
        return solve_case(n);
    }
    let mut res = vec![false; n];
    res[n / 3] = true;
    if n % 6 < 3 {
        res[(n / 3) * 2] = true;
    } else if n % 6 == 3 {
        res[(n / 3) * 2 - 1] = true;
    } else {
        res[(n / 3) * 2 + 1] = true;
    }
    if n % 6 == 2 || n % 6 == 3 {
        res[n - 1] = true;
    }
    res
}

fn solve_case(n: usize) -> Vec<bool> {
    let mut best = (i64::MAX, vec![]);
    for mask in 0i32..1 << n {
        if mask.count_ones() > 3 {
            continue;
        }
        let mut a = vec![0; n];
        for i in 0..n {
            if (mask >> i) & 1 == 1 {
                a[i] = 1;
            }
        }
        let mut score = 0;
        for i in 0..n {
            let mut val = 0;
            for j in i..n {
                val = (val * 2 + a[j]) % 3;
                if val == 0 {
                    score += 1;
                }
            }
        }
        best = best.min((score, a));
    }
    best.1.into_iter().map(|x| x == 1).collect()
}

fn solve(input: &mut Input, out: &mut Output) {
    let tc = input.usize();
    for _ in 0..tc {
        let n = input.usize();
        let res = solve_case_fast(n);
        let s = res
            .into_iter()
            .map(|x| if x { '1' } else { '0' })
            .collect::<String>();
        out.println(s);
    }
}

fn stress() {
    for n in 10..30 {
        let ss = solve_case(n);
        let s2 = solve_case_fast(n);
        let s = ss
            .iter()
            .cloned()
            .map(|x| if x { '1' } else { '0' })
            .collect::<String>();
        let mut ones = vec![];
        for i in 0..n {
            if s.chars().nth(i).unwrap() == '1' {
                ones.push(i);
            }
        }
        dbg!(n, s, ones);
        if ss != s2 {
            let mut ones2 = vec![];
            for i in 0..n {
                if s2[i] {
                    ones2.push(i);
                }
            }
            dbg!(ones, ones2);
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
    const PROBLEM_NAME: &str = "d_";
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
