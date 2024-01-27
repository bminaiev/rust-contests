//{"name":"m","group":"Manual","url":"","interactive":false,"timeLimit":2000,"tests":[{"input":"","output":""},{"input":"","output":""},{"input":"","output":""}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"m"}}}

#[allow(unused)]
use algo_lib::dbg;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;

fn pow(x: i128, pw: i64, md: i128) -> i128 {
    if pw == 0 {
        return 1;
    }
    let mut res = pow(x, pw / 2, md);
    res = res * res % md;
    if pw % 2 == 1 {
        res = res * x % md;
    }
    res
}

const OK_DIFF: i64 = 2_00000_00010;

fn close(x: i64, y: i64, md: i64) -> bool {
    if x + OK_DIFF >= md {
        y >= x || y <= x + OK_DIFF - md
    } else {
        y >= x && y <= x + OK_DIFF
    }
}

fn solve(input: &mut Input, out: &mut Output, _test_case: usize) {
    let p1 = 1000000000000000003i64;
    let p2 = 1010000000000000017i64;
    let n = input.usize();
    let mut cur = [0, 0];
    let mut a = vec![];
    a.push(cur);
    for _ in 0..n {
        let p = input.i128();
        let q = input.i128();
        let val_m1 = pow(q, p1 - 2, p1 as i128);
        let val_m2 = pow(q, p2 - 2, p2 as i128);
        let val_m1 = (val_m1 * p % (p1 as i128)) as i64;
        let val_m2 = (val_m2 * p % (p2 as i128)) as i64;
        cur[0] = (cur[0] + val_m1) % p1;
        cur[1] = (cur[1] + val_m2) % p2;
        a.push(cur);
    }
    a.sort();

    for i in 0..a.len() - 1 {
        if !close(a[i][0], a[i + 1][0], p1) {
            a.rotate_left(i + 1);
            break;
        }
    }

    let mut res = 0;
    let mut i = 0;
    while i < a.len() {
        let mut j = i + 1;
        let mut second = vec![];
        while j < a.len() && close(a[j - 1][0], a[j][0], p1) {
            j += 1;
        }
        for k in i..j {
            second.push(a[k][1]);
        }
        res += solve_simple(second, p2);
        i = j;
    }
    out.println(res);
}

fn solve_simple(mut a: Vec<i64>, p2: i64) -> i64 {
    a.sort();
    for i in 0..a.len() - 1 {
        if !close(a[i], a[i + 1], p2) {
            a.rotate_left(i + 1);
            break;
        }
    }
    let mut res = 0;
    let mut i = 0;
    while i != a.len() {
        let mut j = i + 1;
        while j != a.len() && close(a[j - 1], a[j], p2) {
            j += 1;
        }
        let len = (j - i) as i64;
        res += len * (len - 1) / 2;
        i = j;
    }
    res
}

pub(crate) fn run(mut input: Input, mut output: Output) -> bool {
    solve(&mut input, &mut output, 1);
    output.flush();
    true
}

//START MAIN

fn main() {
    const PROBLEM_NAME: &str = "m";
    use algo_lib::tester::helper::*;

    run_tests(PROBLEM_NAME, run);
    // run_single_test(PROBLEM_NAME, run, "1");
    // run_stress(stress);
    // run_locally(run);
}
//END MAIN
