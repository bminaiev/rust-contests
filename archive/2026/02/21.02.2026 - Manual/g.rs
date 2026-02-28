//{"name":"g","group":"Manual","url":"","interactive":false,"timeLimit":2000,"tests":[{"input":"","output":""}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

#[allow(unused)]
use algo_lib::dbg;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;

fn solve(input: &mut Input, out: &mut Output) {
    let tc = input.usize();
    for _ in 0..tc {
        let max_val = input.usize();
        let x = input.usize();
        assert!(x % 2 == 1);
        let n = (x + 1) / 2;
        assert!(n * n <= max_val);
        let divs = gen_divs(n);
        let non_divs = gen_non_divs(n);
        for &x in divs.iter() {
            assert!(x <= n * n);
            assert!(x > 0);
        }
        for &x in non_divs.iter() {
            assert!(x <= n * n);
            assert!(x > 0);
        }
        let mut div_it = 0;
        let mut non_div_it = 0;
        for _ in 0..x {
            let op = input.string()[0];
            if op == b'-' {
                assert!(div_it < divs.len());
                out.println(divs[div_it] as i32 * -1);
                out.flush();
                div_it += 1;
            } else {
                assert_eq!(op, b'+');
                assert!(non_div_it < non_divs.len());
                out.println(non_divs[non_div_it]);
                out.flush();
                non_div_it += 1;
            }
        }
    }
}

fn add(can: &[bool], x: usize) -> Vec<bool> {
    let mut ncan = vec![false; can.len() + x];
    for i in 0..can.len() {
        if can[i] {
            ncan[i] = true;
            ncan[i + x] = true;
        }
    }
    ncan
}

fn gen_divs(n: usize) -> Vec<usize> {
    let mut res = vec![];
    for i in 0..n {
        res.push((i + 1) * n);
    }
    for i in (0..n).rev() {
        res.push((i + 1) * n - 1);
    }
    res.pop();
    res
}

fn gen_non_divs(n: usize) -> Vec<usize> {
    let mut res = vec![];
    for i in 0..n {
        res.push(i * n + 1);
    }
    for i in (0..n).rev() {
        res.push(i * n + 2);
    }
    res.pop();
    res
}

fn calc(a: &[usize]) -> Vec<bool> {
    let mut can = vec![true];
    for &x in a {
        can = add(&can, x);
    }
    can
}

fn fail(a: &[bool], b: &[bool]) -> Option<usize> {
    for i in 1..a.len().min(b.len()) {
        if a[i] && b[i] {
            return Some(i);
        }
    }
    None
}

fn stress123() {
    for n in 1..50 {
        dbg!(n);
        let divs = gen_divs(n);
        let non_divs = gen_non_divs(n);
        dbg!(divs, non_divs);
        for &x in divs.iter() {
            assert!(x <= n * n);
            assert!(x > 0);
        }
        for &x in non_divs.iter() {
            assert!(x <= n * n);
            assert!(x > 0);
        }
        dbg!(divs.len(), non_divs.len());
        for cnt_non_divs in 0..n * 2 {
            dbg!(cnt_non_divs);
            let cnt_divs = n * 2 - 1 - cnt_non_divs;
            let can_divs = calc(&divs[..cnt_divs]);
            let can_non_divs = calc(&non_divs[..cnt_non_divs]);

            if let Some(x) = fail(&can_divs, &can_non_divs) {
                dbg!(x);
                panic!("fail");
            } else {
                // dbg!("no fail");
            }
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
    const PROBLEM_NAME: &str = "g";
    use algo_lib::tester::helper::*;

    // run_tests(PROBLEM_NAME, run);
    // run_single_test(PROBLEM_NAME, run, "1");
    // run_stress(stress);
    run_locally(run);
}
//END MAIN

#[cfg(not(feature = "local"))]
fn main() {
    let input = algo_lib::io::input::Input::new_stdin();
    let mut output = algo_lib::io::output::Output::new_stdout();
    run(input, output);
}
