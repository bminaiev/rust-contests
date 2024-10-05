//{"name":"f","group":"Manual","url":"","interactive":false,"timeLimit":2000,"tests":[{"input":"","output":""},{"input":"","output":""}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"f"}}}

#[allow(unused)]
use algo_lib::dbg;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;

fn solve(input: &mut Input, out: &mut Output, _test_case: usize) {
    let n = input.usize();
    let mut s: Vec<Vec<u8>> = vec![];
    let mut cnt = vec![];
    for i in 0..n {
        s.push(input.string());
        cnt.push(input.i32());
    }
    let mut res = vec![];
    for i in 0..n {
        let mut rank = 1;
        for j in 0..n {
            if j != i && cnt[j] > cnt[i] {
                rank += 1;
            }
        }
        if rank > s[i].len() {
            rank = s[i].len();
        }
        res.extend(s[i][..s[i].len() - rank].to_vec());
    }
    if res.len() != 0 {
        res[0] = res[0].to_ascii_uppercase();
    }
    let res_str = String::from_utf8(res).unwrap();
    out.print(format!("Stage: {res_str}"));
}

pub(crate) fn run(mut input: Input, mut output: Output) -> bool {
    solve(&mut input, &mut output, 1);
    output.flush();
    true
}

//START MAIN

fn main() {
    const PROBLEM_NAME: &str = "f";
    use algo_lib::tester::helper::*;

    run_tests(PROBLEM_NAME, run);
    // run_single_test(PROBLEM_NAME, run, "1");
    // run_stress(stress);
    // run_locally(run);
}
//END MAIN
