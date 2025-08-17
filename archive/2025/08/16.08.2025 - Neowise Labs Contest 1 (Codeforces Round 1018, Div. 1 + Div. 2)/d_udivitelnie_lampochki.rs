//{"name":"D. Удивительные лампочки","group":"Codeforces - Neowise Labs Contest 1 (Codeforces Round 1018, Div. 1 + Div. 2)","url":"https://codeforces.com/contest/2096/problem/D","interactive":false,"timeLimit":2000,"tests":[{"input":"4\n1\n2 3\n3\n-2 -1\n-1 -2\n-1 -3\n7\n7 26\n6 27\n6 28\n7 27\n8 26\n8 27\n7 28\n11\n70 9\n69 8\n69 0\n73 5\n70 -1\n70 5\n71 7\n70 4\n73 4\n71 3\n72 3\n","output":"2 3\n-2 -2\n7 27\n72 7\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"DUdivitelnieLampochki"}}}

#[allow(unused)]
use algo_lib::dbg;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;

fn solve(input: &mut Input, out: &mut Output, _test_case: usize) {
    let tc = input.usize();
    for _ in 0..tc {
        let n = input.usize();
        let mut x = 0;
        let mut x_p_y = 0;
        for _ in 0..n {
            let cx = input.i64();
            let cy = input.i64();
            x ^= cx;
            x_p_y ^= cx + cy;
        }
        let y = x_p_y - x;
        out.println(vec![x, y]);
    }
}

pub(crate) fn run(mut input: Input, mut output: Output) -> bool {
    solve(&mut input, &mut output, 1);
    output.flush();
    true
}

//START MAIN

fn main() {
    const PROBLEM_NAME: &str = "d_udivitelnie_lampochki";
    use algo_lib::tester::helper::*;

    run_tests(PROBLEM_NAME, run);
    // run_single_test(PROBLEM_NAME, run, "1");
    // run_stress(stress);
    // run_locally(run);
}
//END MAIN
