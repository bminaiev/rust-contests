#[allow(unused)]
use algo_lib::dbg;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::rand::Random;

fn solve(input: &mut Input, out: &mut Output) {
    let tc = input.usize();
    let mut rnd = Random::new(123);
    for _ in 0..tc {
        let a = input.usize();
        let b = input.usize();
        let c = input.usize();
        const MX: usize = 1024;
        let mut found = false;
        let mut all = vec![];
        for i in a + 1..MX {
            all.push((0, i));
        }
        for i in b + 1..MX {
            all.push((1, i));
        }
        for _ in 0..20 {
            let mut aa = vec![vec![false; MX]; 2];
            for i in 0..a {
                aa[0][i] = true;
            }
            for i in 0..b {
                aa[1][i] = true;
            }
            rnd.shuffle(&mut all);
            for &(id, val) in all.iter() {
                let val2 = val ^ c;
                if aa[1 - id][val2] {
                    continue;
                }
                aa[id][val] = true;
            }
            let mut cc = vec![false; MX];
            for i in 0..MX {
                for j in 0..MX {
                    cc[i ^ j] |= (aa[0][i] & aa[1][j]);
                }
            }
            let mut mex = 0;
            while mex != MX && cc[mex] {
                mex += 1;
            }
            if mex == c {
                out.println("Yes");
                for i in 0..2 {
                    let mut cur = vec![];
                    for j in 0..MX {
                        if aa[i][j] {
                            cur.push(j);
                        }
                    }
                    let sz = cur.len();
                    cur.insert(0, sz);
                    out.println(cur);
                }
                found = true;
                break;
            }
        }
        if !found {
            out.println("No");
        }
    }
}

pub(crate) fn run(mut input: Input, mut output: Output) -> bool {
    solve(&mut input, &mut output);
    output.flush();
    true
}

#[cfg(feature = "local")]
fn main() {
    const PROBLEM_NAME: &str = "b_three_mex";
    #[allow(unused)]
    use algo_lib::misc::dragon::run_dragon;
    use algo_lib::tester::helper::*;

    run_tests(PROBLEM_NAME, run);
    // run_single_test(PROBLEM_NAME, run, "1");
    // run_stress(stress);
    // run_locally(run);
    // run_dragon(solve, "W", 1..2);
}
//END MAIN

#[cfg(not(feature = "local"))]
fn main() {
    let input = algo_lib::io::input::Input::new_stdin();
    let mut output = algo_lib::io::output::Output::new_stdout();
    run(input, output);
}
