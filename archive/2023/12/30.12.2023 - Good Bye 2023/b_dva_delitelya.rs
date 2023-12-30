//{"name":"B. Два делителя","group":"Codeforces - Good Bye 2023","url":"https://codeforces.com/contest/1916/problem/B","interactive":false,"timeLimit":1000,"tests":[{"input":"8\n2 3\n1 2\n3 11\n1 5\n5 10\n4 6\n3 9\n250000000 500000000\n","output":"6\n4\n33\n25\n20\n12\n27\n1000000000\n"}],"testType":"multiNumber","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"BDvaDelitelya"}}}

#[allow(unused)]
use algo_lib::dbg;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::math::gcd::lcm;

fn solve(input: &mut Input, out: &mut Output, _test_case: usize) {
    let x = input.i64();
    let y = input.i64();
    let lcm = lcm(x, y);
    if lcm == y {
        out.println(lcm * (y / x));
    } else {
        out.println(lcm);
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
    const PROBLEM_NAME: &str = "b_dva_delitelya";
    use algo_lib::tester::helper::*;

    run_tests(PROBLEM_NAME, run);
    // run_single_test(PROBLEM_NAME, run, "1");
    // run_stress(stress);
    // run_locally(run);
}
//END MAIN
