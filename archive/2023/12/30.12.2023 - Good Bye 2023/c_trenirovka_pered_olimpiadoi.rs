//{"name":"C. Тренировка перед олимпиадой","group":"Codeforces - Good Bye 2023","url":"https://codeforces.com/contest/1916/problem/C","interactive":false,"timeLimit":1000,"tests":[{"input":"4\n1\n31\n6\n6 3 7 2 5 4\n3\n3 10 11\n5\n7 13 11 19 1\n","output":"31\n6 8 16 18 22 26\n3 12 24\n7 20 30 48 50\n"}],"testType":"multiNumber","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"CTrenirovkaPeredOlimpiadoi"}}}

#[allow(unused)]
use algo_lib::dbg;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;

fn solve(input: &mut Input, out: &mut Output, _test_case: usize) {
    let n = input.usize();
    let a = input.vec::<i64>(n);
    let mut res = vec![];
    let mut sum = 0;
    let mut cnt_odd = 0;

    for (it, &x) in a.iter().enumerate() {
        sum += x;
        if x % 2 == 1 {
            cnt_odd += 1;
        }
        let mut r = sum - cnt_odd;
        r += (cnt_odd + 1) / 3 * 2;
        if it == 0 {
            r = sum;
        }
        res.push(r);
    }
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
    const PROBLEM_NAME: &str = "c_trenirovka_pered_olimpiadoi";
    use algo_lib::tester::helper::*;

    run_tests(PROBLEM_NAME, run);
    // run_single_test(PROBLEM_NAME, run, "1");
    // run_stress(stress);
    // run_locally(run);
}
//END MAIN
