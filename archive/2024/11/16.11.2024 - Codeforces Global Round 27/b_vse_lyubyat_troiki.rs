//{"name":"B. Все любят тройки","group":"Codeforces - Codeforces Global Round 27","url":"https://codeforces.com/contest/2035/problem/B","interactive":false,"timeLimit":1000,"tests":[{"input":"6\n1\n2\n3\n4\n5\n7\n","output":"-1\n66\n-1\n3366\n36366\n3336366\n"}],"testType":"multiNumber","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"BVseLyubyatTroiki"}}}

#[allow(unused)]
use algo_lib::dbg;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;

fn solve(input: &mut Input, out: &mut Output, _test_case: usize) {
    let n = input.usize();
    if n == 1 || n == 3 {
        out.println(-1);
        return;
    }
    if n % 2 == 0 {
        for _i in 0..n - 2 {
            out.print('3');
        }
        out.println("66");
    } else {
        for _i in 0..n - 4 {
            out.print('3');
        }
        out.println("6366");
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
    const PROBLEM_NAME: &str = "b_vse_lyubyat_troiki";
    use algo_lib::tester::helper::*;

    run_tests(PROBLEM_NAME, run);
    // run_single_test(PROBLEM_NAME, run, "1");
    // run_stress(stress);
    // run_locally(run);
}
//END MAIN
