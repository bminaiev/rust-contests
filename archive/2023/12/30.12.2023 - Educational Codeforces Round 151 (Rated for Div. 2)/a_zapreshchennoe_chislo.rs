//{"name":"A. Запрещенное число","group":"Codeforces - Educational Codeforces Round 151 (Rated for Div. 2)","url":"https://codeforces.com/contest/1845/problem/A","interactive":false,"timeLimit":2000,"tests":[{"input":"5\n10 3 2\n5 2 1\n4 2 1\n7 7 3\n6 1 1\n","output":"YES\n6\n3 1 1 1 1 3\nNO\nYES\n2\n2 2\nYES\n1\n7\nNO\n"}],"testType":"multiNumber","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"AZapreshchennoeChislo"}}}

#[allow(unused)]
use algo_lib::dbg;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;

fn solve(input: &mut Input, out: &mut Output, _test_case: usize) {
    let mut n = input.i32();
    let k = input.i32();
    let x = input.i32();
    let ok = if x != 1 {
        vec![1; n as usize]
    } else if k == 1 {
        vec![]
    } else {
        let mut res = vec![];
        if n % 2 == 1 && n >= 3 && k >= 3 {
            res.push(3);
            n -= 3;
        }
        while n != 0 && n > 1 && n % 2 == 0 {
            res.push(2);
            n -= 2;
        }
        res
    };
    if ok.is_empty() {
        out.println("NO");
    } else {
        out.println("YES");
        out.println(ok.len());
        out.println(ok);
    }
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
    const PROBLEM_NAME: &str = "a_zapreshchennoe_chislo";
    use algo_lib::tester::helper::*;

    run_tests(PROBLEM_NAME, run);
    // run_single_test(PROBLEM_NAME, run, "1");
    // run_stress(stress);
    // run_locally(run);
}
//END MAIN
