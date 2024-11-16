//{"name":"h","group":"Manual","url":"","interactive":false,"timeLimit":2000,"tests":[{"input":"","output":""},{"input":"","output":""},{"input":"","output":""}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"h"}}}

#[allow(unused)]
use algo_lib::dbg;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::math::modulo::Mod_998_244_353;

#[derive(Clone, Copy)]
struct Switch {
    t: i64,
    a: i64,
    b: i64,
}

type Mod = Mod_998_244_353;

fn solve(input: &mut Input, out: &mut Output, _test_case: usize) {
    let n = input.usize();
    let mut a = vec![];
    for _ in 0..n {
        let t = input.i64();
        let a1 = input.i64();
        let b = input.i64();
        a.push(Switch { t, a: a1, b });
    }
    a.sort_by(|s1, s2| {
        let v1 = s1.t * (s2.b - s2.a) * s1.b;
        let v2 = s2.t * (s1.b - s1.a) * s2.b;
        v1.cmp(&v2)
    });
    let mut prob_win = Mod::ONE;
    let mut every_time = Mod::ZERO;
    for s in a.iter() {
        every_time += prob_win * Mod::new(s.t);
        prob_win *= Mod::new(s.a);
        prob_win /= Mod::new(s.b);
    }
    let res = every_time / prob_win;
    out.println(res);
}

pub(crate) fn run(mut input: Input, mut output: Output) -> bool {
    solve(&mut input, &mut output, 1);
    output.flush();
    true
}

//START MAIN

fn main() {
    const PROBLEM_NAME: &str = "h";
    use algo_lib::tester::helper::*;

    run_tests(PROBLEM_NAME, run);
    // run_single_test(PROBLEM_NAME, run, "1");
    // run_stress(stress);
    // run_locally(run);
}
//END MAIN
