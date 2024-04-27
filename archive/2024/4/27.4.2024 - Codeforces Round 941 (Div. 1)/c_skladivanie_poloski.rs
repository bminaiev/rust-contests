//{"name":"C. Складывание полоски","group":"Codeforces - Codeforces Round 941 (Div. 1)","url":"https://codeforces.com/contest/1965/problem/C","interactive":false,"timeLimit":2000,"tests":[{"input":"6\n6\n101101\n1\n0\n12\n110110110011\n5\n01110\n4\n1111\n2\n01\n","output":"3\n1\n3\n3\n1\n2\n"}],"testType":"multiNumber","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"CSkladivaniePoloski"}}}

#[allow(unused)]
use algo_lib::dbg;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;

fn solve(input: &mut Input, out: &mut Output, _test_case: usize) {
    let _n = input.usize();
    let s = input.string();
    let mut dedup = vec![];
    for &c in s.iter() {
        let sz = dedup.len();
        if sz >= 2 && dedup[sz - 1] == dedup[sz - 2] && dedup[sz - 1] == c {
            dedup.pop();
        } else {
            dedup.push(c);
        }
    }
    let mut res = 1;
    let mut same = vec![0];
    for i in 0..(dedup.len() - 1) {
        if dedup[i] == dedup[i + 1] {
            same.push(i + 1);
        }
    }
    same.push(dedup.len());
    for w in same.windows(2) {
        res = res.max(w[1] - w[0]);
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
    const PROBLEM_NAME: &str = "c_skladivanie_poloski";
    use algo_lib::tester::helper::*;

    // run_tests(PROBLEM_NAME, run);
    run_single_test(PROBLEM_NAME, run, "2");
    // run_stress(stress);
    // run_locally(run);
}
//END MAIN
