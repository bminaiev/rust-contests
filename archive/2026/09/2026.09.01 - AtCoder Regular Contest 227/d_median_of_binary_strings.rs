#[allow(unused)]
use algo_lib::dbg;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::rand::Random;

fn solve(input: &mut Input, out: &mut Output) {}

fn find_all(a: &[Vec<usize>]) -> usize {
    let n = a[0].len();
    let mut a = a.to_vec();
    loop {
        let mut ch = false;
        for i in 0..a.len() {
            for j in i + 1..a.len() {
                for k in j + 1..a.len() {
                    let mut b = vec![0; n];
                    for l in 0..n {
                        b[l] = (a[i][l] + a[j][l] + a[k][l]) / 2;
                    }
                    if a.contains(&b) {
                        continue;
                    }
                    a.push(b);
                    ch = true;
                }
            }
        }
        if !ch {
            break;
        }
    }
    a.len()
}

fn find_all2(a: &[Vec<usize>]) -> usize {
    let n = a[0].len();
    let mut a = a.to_vec();
    for mask in 0usize..(1 << a.len()) {
        let cnt: usize = mask.count_ones() as usize;
        if cnt >= 3 && cnt % 2 == 1 {
            let mut b = vec![0; n];
            for i in 0..a.len() {
                if (mask >> i) & 1 == 1 {
                    for j in 0..n {
                        b[j] += a[i][j];
                    }
                }
            }
            for j in 0..n {
                b[j] = if b[j] > (cnt / 2) { 1 } else { 0 };
                assert!(b[j] <= 1);
            }
            a.push(b);
        }
    }
    a.sort();
    a.dedup();
    // for x in a.iter() {
    //     dbg!(x);
    // }
    a.len()
}

fn stress() {
    let n = 10;
    let mut rnd = Random::new(123);
    let mut a = vec![];
    for i in 0..6 {
        a.push(rnd.gen_vec(n, 0..2));
    }
    let res1 = find_all(&a);
    let res2 = find_all2(&a);
    dbg!(a, res1, res2);
}

pub(crate) fn run(mut input: Input, mut output: Output) -> bool {
    solve(&mut input, &mut output);
    output.flush();
    true
}

#[cfg(feature = "local")]
fn main() {
    const PROBLEM_NAME: &str = "d_median_of_binary_strings";
    #[allow(unused)]
    use algo_lib::misc::dragon::run_dragon;
    use algo_lib::tester::helper::*;

    // run_tests(PROBLEM_NAME, run);
    // run_single_test(PROBLEM_NAME, run, "1");
    run_stress(stress);
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
