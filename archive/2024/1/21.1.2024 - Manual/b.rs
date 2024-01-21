//{"name":"b","group":"Manual","url":"","interactive":false,"timeLimit":2000,"tests":[{"input":"","output":""}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"b"}}}

#[allow(unused)]
use algo_lib::dbg;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;

fn solve(input: &mut Input, out: &mut Output, _test_case: usize) {
    let a = gen();
    // let mut sum_len = 0;
    // for i in 0..a.len() {
    //     sum_len += a[i].len();
    // }
    // dbg!(sum_len);
    let prec_sz = a.len() as i64;
    let tc = input.usize();
    for _ in 0..tc {
        let mut n = input.i64() - 1;
        let pos = input.usize();
        if n > prec_sz {
            let shift = (n - prec_sz) / 2;
            n -= shift * 2;
        }
        while n >= prec_sz {
            n -= 2;
        }
        let ans = if pos >= a[n as usize].len() {
            0
        } else {
            a[n as usize][pos]
        };
        out.println(ans);
    }
}

const MX: usize = 2_000_000;

fn next(a: &[u8]) -> Vec<u8> {
    let mut i = 0;
    let mut res = vec![];
    while i < a.len() {
        let mut j = i;
        while j != a.len() && a[j] == a[i] {
            j += 1;
        }
        res.push(a[i]);
        let mut len = j - i;
        while len > 0 {
            res.push((len % 2) as u8);
            len /= 2;
        }
        i = j;
    }
    res.truncate(MX);
    res
}

fn gen() -> Vec<Vec<u8>> {
    let mut a = vec![];
    a.push(vec![1]);
    for lvl in 0.. {
        let last = &a[a.len() - 1];
        let next = next(last);
        // eprintln!("{lvl} -> {next:?}");
        a.push(next);
        let sz = a.len();
        if a.len() > 5 && a[sz - 3] == a[sz - 5] && a[sz - 4] == a[sz - 6] {
            // eprintln!("Done! {sz}");
            break;
        }
    }
    a
}

fn stress() {}

pub(crate) fn run(mut input: Input, mut output: Output) -> bool {
    solve(&mut input, &mut output, 1);
    output.flush();
    true
}

//START MAIN

fn main() {
    const PROBLEM_NAME: &str = "b";
    use algo_lib::tester::helper::*;

    run_tests(PROBLEM_NAME, run);
    // run_single_test(PROBLEM_NAME, run, "1");
    // run_stress(stress);
    // run_locally(run);
}
//END MAIN
