//{"name":"i","group":"Manual","url":"","interactive":false,"timeLimit":2000,"tests":[{"input":"","output":""},{"input":"","output":""}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::array_2d::Array2D;
#[allow(unused)]
use algo_lib::dbg;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::rand::Random;

fn gen_ans(n: usize) -> Option<Array2D<usize>> {
    let mut a = Array2D::new(1, n, n);
    if n % 2 == 0 {
        return Some(a);
    }
    if n == 1 || n == 3 {
        return None;
    }
    for i in 0..n - 2 {
        a[i][i] = 2;
        if i != n - 3 {
            a[i][i + 2] = 3;
        }
    }
    a[n - 3][1] = 3;
    a[n - 2][0] = 3;
    a[n - 1][n - 1] = 3;
    a[n - 2][n - 1] = 2;
    a[n - 1][n - 2] = 2;
    Some(a)
}

fn solve(input: &mut Input, out: &mut Output) {
    let n = input.usize();
    if let Some(ans) = gen_ans(n) {
        assert!(ok(&ans));
        out.println(sum(&ans));
        for i in 0..n {
            out.println(ans[i].to_vec());
        }
    } else {
        out.println(-1);
    }
}

fn ok(a: &Array2D<usize>) -> bool {
    let n = a.len();
    for i in 0..n {
        let mut xor = 0;
        for j in 0..n {
            xor ^= a[i][j];
        }
        if xor != 0 {
            return false;
        }
    }
    for j in 0..n {
        let mut xor = 0;
        for i in 0..n {
            xor ^= a[i][j];
        }
        if xor != 0 {
            return false;
        }
    }
    {
        let mut xor = 0;
        for i in 0..n {
            xor ^= a[i][i];
        }
        if xor != 0 {
            return false;
        }
    }
    {
        let mut xor = 0;
        for i in 0..n {
            xor ^= a[i][n - 1 - i];
        }
        if xor != 0 {
            return false;
        }
    }
    true
}

fn sum(a: &Array2D<usize>) -> usize {
    let n = a.len();
    let mut sum = 0;
    for i in 0..n {
        for j in 0..n {
            sum += a[i][j];
        }
    }
    sum
}

fn stress() {
    let mut rnd = Random::new(123);
    let n = 7;
    let mut a = Array2D::new(1, n, n);
    let mut best = usize::MAX;
    let max_v = 4;
    for _it in 1.. {
        if _it % 10000000 == 0 {
            println!("it: {}", _it);
        }
        if ok(&a) {
            let sum = sum(&a);
            if sum < best {
                best = sum;
                dbg!(best);
                for i in 0..n {
                    for j in 0..n {
                        print!("{} ", a[i][j]);
                    }
                    println!();
                }
            }
        }
        let x = rnd.gen_range(0..n);
        let y = rnd.gen_range(0..n);
        if rnd.gen_double() < 0.1 {
            a[x][y] = rnd.gen_range(1..max_v);
        } else {
            if rnd.gen_bool() {
                let mut xor = a[x][y];
                for i in 0..n {
                    xor ^= a[x][i];
                }
                if xor != 0 {
                    a[x][y] = xor;
                }
            } else {
                let mut xor = a[x][y];
                for i in 0..n {
                    xor ^= a[i][y];
                }
                if xor != 0 {
                    a[x][y] = xor;
                }
            }
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
    const PROBLEM_NAME: &str = "i";
    use algo_lib::tester::helper::*;

    run_tests(PROBLEM_NAME, run);
    // run_single_test(PROBLEM_NAME, run, "1");
    // run_stress(stress);
    // run_locally(run);
}
//END MAIN

#[cfg(not(feature = "local"))]
fn main() {
    let input = algo_lib::io::input::Input::new_stdin();
    let mut output = algo_lib::io::output::Output::new_stdout();
    run(input, output);
}
