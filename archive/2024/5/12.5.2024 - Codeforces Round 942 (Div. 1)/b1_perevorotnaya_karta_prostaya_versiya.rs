//{"name":"B1. Переворотная карта (простая версия)","group":"Codeforces - Codeforces Round 942 (Div. 1)","url":"https://codeforces.com/contest/1967/problem/B1","interactive":false,"timeLimit":2000,"tests":[{"input":"6\n1 1\n2 3\n3 5\n10 8\n100 1233\n1000000 1145141\n","output":"1\n3\n4\n14\n153\n1643498\n"}],"testType":"multiNumber","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"B1PerevorotnayaKartaProstayaVersiya"}}}

#[allow(unused)]
use algo_lib::dbg;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::math::gcd::gcd;

fn solve_case(n: i64, m: i64) -> i64 {
    let mut res = n;
    for b in 2..=m {
        for mul in ((b - 1)..).step_by(b as usize) {
            if mul * b <= n {
                res += 1;
            } else {
                break;
            }
        }
    }
    res
}

fn stress() {
    const MX: i32 = 50;
    for a in 1..MX {
        let mut ok = vec![];
        for b in 1..MX * 10 {
            // if (a + b) % (b * gcd(a, b)) == 0 {
            // ok.push(a);
            // }
            if ((b * gcd(a, b)) % (a + b)) == 0 {
                ok.push(b);
            }
        }
        dbg!(a, ok);
    }
}

fn solve(input: &mut Input, out: &mut Output, _test_case: usize) {
    let n = input.i64();
    let m = input.i64();
    let res = solve_case(n, m);
    out.println(res);
}

pub(crate) fn run(mut input: Input, mut output: Output) -> bool {
    let t = input.read();
    for i in 0usize..t {
        solve(&mut input, &mut output, i + 1);
    }
    output.flush();
    true
}

//START MAIN

fn main() {
    const PROBLEM_NAME: &str = "b1_perevorotnaya_karta_prostaya_versiya";
    use algo_lib::tester::helper::*;

    // run_tests(PROBLEM_NAME, run);
    // run_single_test(PROBLEM_NAME, run, "1");
    run_stress(stress);
    // run_locally(run);
}
//END MAIN
