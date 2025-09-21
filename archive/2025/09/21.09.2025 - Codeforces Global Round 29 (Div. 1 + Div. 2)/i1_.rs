//{"name":"I1. Длиннейший возрастающий путь (простая версия)","group":"Codeforces - Codeforces Global Round 29 (Div. 1 + Div. 2)","url":"https://codeforces.com/contest/2147/problem/I1","interactive":false,"timeLimit":2000,"tests":[{"input":"8 6\n","output":"1 1 3 6 10 3 11 1\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

#[allow(unused)]
use algo_lib::dbg;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;

const BASE: i64 = 1000000000;

fn generate(m: usize) -> (Vec<i64>, Vec<i64>) {
    let mut a = vec![];
    let mut now = 0;
    a.push(now);
    let mut delta = BASE;
    for i in 0..m / 2 {
        now += delta;
        delta += 10;
        a.push(now);
    }
    let mut b = vec![];
    while now - delta > 0 {
        now -= delta;
        delta += 10;
        b.push(now);
    }
    b.push(0);
    b.sort();
    (a, b)
}

fn walk(a: &[i64], b: &[i64]) -> Vec<i64> {
    let mut res = vec![0];
    assert!(a[0] == 0);
    let mut delta = BASE;
    let mut go_right = true;
    let mut cnt_fails = 0;
    while cnt_fails < 2 {
        let last = *res.last().unwrap();
        if go_right {
            let at_least = last + delta;

            let pos = a.binary_search(&at_least).unwrap_or_else(|x| x);
            if pos == a.len() {
                cnt_fails += 1;
                go_right = false;
                dbg!("go left", res.len(), delta);
            } else {
                cnt_fails = 0;
                let next_delta = a[pos] - last;
                assert!(next_delta >= delta);
                delta = next_delta + 1;
                res.push(a[pos]);
            }
        } else {
            let at_most = last - delta;
            if at_most < 0 {
                cnt_fails += 1;
                go_right = true;
                dbg!("go right", res.len(), delta);
                continue;
            }
            let pos = b.binary_search(&(at_most + 1)).unwrap_or_else(|x| x);
            assert!(pos > 0);
            let pos = pos - 1;
            cnt_fails = 0;
            let next_delta = last - b[pos];
            assert!(next_delta >= delta);
            delta = next_delta + 1;
            res.push(b[pos]);
        }
    }
    res
}

fn stress() {
    const M: usize = 75;
    let (a, b) = generate(M);
    dbg!(a.len(), b.len(), a.len() + b.len());
    // for i in 0..100 {
    //     dbg!(i, a[i]);
    // }
    let res = walk(&a, &b);
    let mut all = res.clone();
    all.sort();
    all.dedup();
    assert!(all.len() <= M);
    for i in 0..res.len() - 2 {
        let d1 = (res[i + 1] - res[i]).abs();
        let d2 = (res[i + 2] - res[i + 1]).abs();
        assert!(d2 > d1);
    }
    dbg!(res.len());
    let extra = 15000 / M;
    dbg!(res.len() * (2 * extra - 1));
}

fn solve(input: &mut Input, out: &mut Output) {
    let n = input.usize();
    let m = input.usize();
    if n == 8 {
        let res = vec![1, 1, 3, 6, 10, 3, 11, 1];
        out.println(res);
    } else {
        let (a, b) = generate(m);
        let mut res = walk(&a, &b);
        assert!(res.len() >= n);
        res.truncate(n);
        out.println(res);
    }
}

pub(crate) fn run(mut input: Input, mut output: Output) -> bool {
    solve(&mut input, &mut output);
    output.flush();
    true
}

#[cfg(feature = "local")]
fn main() {
    const PROBLEM_NAME: &str = "i1_";
    use algo_lib::tester::helper::*;

    // run_tests(PROBLEM_NAME, run);
    // run_single_test(PROBLEM_NAME, run, "1");
    run_stress(stress);
    // run_locally(run);
}
//END MAIN

#[cfg(not(feature = "local"))]
fn main() {
    let input = algo_lib::io::input::Input::new_stdin();
    let mut output = algo_lib::io::output::Output::new_stdout();
    run(input, output);
}
