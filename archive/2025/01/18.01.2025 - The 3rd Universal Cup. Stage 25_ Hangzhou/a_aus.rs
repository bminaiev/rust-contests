//{"name":"A. AUS","group":"Universal Cup - The 3rd Universal Cup. Stage 25: Hangzhou","url":"https://contest.ucup.ac/contest/1893/problem/9726","interactive":false,"timeLimit":1000,"tests":[{"input":"4\nabab\ncdcd\nabce\nabab\ncdcd\nabcd\nabab\ncdcd\nabc\nx\nyz\ndef\n","output":"YES\nNO\nYES\nNO\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"AAUS"}}}

#[allow(unused)]
use algo_lib::dbg;
use algo_lib::graph::dsu::Dsu;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;

fn solve(input: &mut Input, out: &mut Output, _test_case: usize) {
    let tc = input.usize();
    for _ in 0..tc {
        let s1 = input.string();
        let s2 = input.string();
        let s3 = input.string();
        let mut ok = s1.len() == s2.len();
        if ok {
            let mut dsu = Dsu::new(26);
            for i in 0..s1.len() {
                let p1 = s1[i] as usize - 'a' as usize;
                let p2 = s2[i] as usize - 'a' as usize;
                dsu.unite(p1, p2);
            }
            if s1.len() == s3.len() {
                ok = false;
                for i in 0..s1.len() {
                    let p1 = s1[i] as usize - 'a' as usize;
                    let p2 = s3[i] as usize - 'a' as usize;
                    if dsu.get(p1) != dsu.get(p2) {
                        ok = true;
                    }
                }
            }
        }
        if ok {
            out.println("YES");
        } else {
            out.println("NO");
        }
    }
}

pub(crate) fn run(mut input: Input, mut output: Output) -> bool {
    solve(&mut input, &mut output, 1);
    output.flush();
    true
}

//START MAIN

fn main() {
    const PROBLEM_NAME: &str = "a_aus";
    use algo_lib::tester::helper::*;

    run_tests(PROBLEM_NAME, run);
    // run_single_test(PROBLEM_NAME, run, "1");
    // run_stress(stress);
    // run_locally(run);
}
//END MAIN
