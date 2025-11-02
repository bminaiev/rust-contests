//{"name":"f","group":"Manual","url":"","interactive":false,"timeLimit":2000,"tests":[{"input":"","output":""}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::array_2d::Array2D;
#[allow(unused)]
use algo_lib::dbg;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::rand::Random;
use algo_lib::misc::simulated_annealing::SimulatedAnnealing;

fn emulate(a: &Array2D<usize>) -> usize {
    const DX: [i32; 4] = [-1, 0, 0, 1];
    const DY: [i32; 4] = [0, -1, 1, 0];
    const REV: [usize; 4] = [3, 2, 1, 0];

    let mut a = a.clone();
    let mut x = 0;
    let mut y = 0;
    let mut steps = 0;
    while x != a.len() - 1 || y != a[0].len() - 1 {
        steps += 1;
        let dir = a[x][y];
        a[x][y] = REV[dir];
        let xn = (x as i32 + DX[dir]);
        let yn = (y as i32 + DY[dir]);
        if xn >= 0 && (xn < a.len() as i32) && yn >= 0 && (yn < a[0].len() as i32) {
            x = xn as usize;
            y = yn as usize;
        }
    }
    steps
}

fn stress() {
    const N: usize = 8;
    let mut a = Array2D::new(0, N, N);
    let mut rnd = Random::new(342324);
    for i in 0..N {
        for j in 0..N {
            a[i][j] = rnd.gen_range(0..4);
        }
    }
    let steps = emulate(&a);
    let mut sa = SimulatedAnnealing::new(
        10.0,
        algo_lib::misc::simulated_annealing::SearchFor::MaximumScore,
        100.0,
        1e-4,
        steps as f64,
    );
    while sa.should_continue() {
        let x = rnd.gen_range(0..N);
        let y = rnd.gen_range(0..N);
        let old = a[x][y];
        a[x][y] = rnd.gen_range(0..4);
        let new_steps = emulate(&a);
        dbg!(new_steps);
        if !sa.should_go(new_steps as f64) {
            a[x][y] = old;
        } else {
            dbg!(new_steps);
        }
    }
}

fn solve(input: &mut Input, out: &mut Output) {}

pub(crate) fn run(mut input: Input, mut output: Output) -> bool {
    solve(&mut input, &mut output);
    output.flush();
    true
}

#[cfg(feature = "local")]
fn main() {
    const PROBLEM_NAME: &str = "f";
    use algo_lib::tester::helper::*;

    // run_tests(PROBLEM_NAME, run);
    // run_single_test(PROBLEM_NAME, run, "1");
    run_stress(stress);
    // run_locally(run);
}
//END MAIN

#[cfg(not(feature = "local"))]
fn main() {
    let input = algo_lib::io::input::Input::new_stdin();
    let mut output = algo_lib::io::output::Output::new_stdout();
    run(input, output);
}
