//{"name":"H. Horizon Scanning","group":"Universal Cup - The 3rd Universal Cup. Stage 20: Kunming","url":"https://contest.ucup.ac/contest/1871/problem/9869","interactive":false,"timeLimit":1000,"tests":[{"input":"5\n1 1\n0 1\n8 2\n1 0\n1 1\n0 1\n-1 1\n-1 0\n-1 -1\n0 -1\n1 -1\n4 2\n-1 1\n0 1\n0 2\n1 1\n4 2\n-1000000000 0\n-998244353 1\n998244353 1\n1000000000 0\n3 1\n0 1\n0 2\n0 -1\n","output":"6.2831853072\n1.5707963268\n5.4977871438\n3.1415926546\n3.1415926536\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"HHorizonScanning"}}}

#[allow(unused)]
use algo_lib::dbg;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;

fn solve(input: &mut Input, out: &mut Output, _test_case: usize) {
    let tc = input.usize();
    for _ in 0..tc {
        let n = input.usize();
        let k = input.usize();
        let mut angles = vec![];
        for _ in 0..n {
            let x = input.f64();
            let y = input.f64();
            let angle = y.atan2(x);
            angles.push(angle);
        }
        angles.sort_by(|&a, &b| a.partial_cmp(&b).unwrap());
        let mut extra_angles = vec![];
        for &x in angles.iter() {
            extra_angles.push(x + 2.0 * std::f64::consts::PI);
        }
        angles.extend(extra_angles);
        let mut res = 0.0;
        for i in 0..n {
            let delta = angles[i + k] - angles[i];
            if delta > res {
                res = delta;
            }
        }
        out.println(res);
    }
}

pub(crate) fn run(mut input: Input, mut output: Output) -> bool {
    solve(&mut input, &mut output, 1);
    output.flush();
    true
}

//START MAIN

fn main() {
    const PROBLEM_NAME: &str = "h_horizon_scanning";
    use algo_lib::tester::helper::*;

    run_tests(PROBLEM_NAME, run);
    // run_single_test(PROBLEM_NAME, run, "1");
    // run_stress(stress);
    // run_locally(run);
}
//END MAIN
