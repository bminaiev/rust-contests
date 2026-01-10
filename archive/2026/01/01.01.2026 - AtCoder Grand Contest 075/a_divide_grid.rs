//{"name":"A - Divide Grid","group":"AtCoder - AtCoder Grand Contest 075","url":"https://atcoder.jp/contests/agc075/tasks/agc075_a","interactive":false,"timeLimit":2000,"tests":[{"input":"2\n","output":"01\n01\n"},{"input":"6\n","output":"100111\n101000\n100010\n011101\n010000\n110001\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use std::collections::HashSet;

use algo_lib::collections::array_2d::Array2D;
#[allow(unused)]
use algo_lib::dbg;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::rand::Random;

fn calc(s: &[Vec<u8>]) -> bool {
    let n = s.len();
    let mut cnt = [0; 2];
    let m = n * 2;
    let mut c = Array2D::new(0, m + 1, m + 1);
    c[0][0] = 1;
    for i in 1..=m {
        c[i][0] = 1;
        for j in 1..=m {
            c[i][j] = c[i - 1][j - 1] + c[i - 1][j];
        }
    }
    for x1 in 0..n {
        for y1 in 0..n {
            for x2 in x1..n {
                for y2 in y1..n {
                    if x1 == x2 && y1 == y2 {
                        continue;
                    }
                    if s[x1][y1] != s[x2][y2] {
                        continue;
                    }
                    let value = (s[x1][y1] - b'0') as usize;
                    cnt[value] += c[x2 - x1 + y2 - y1][x2 - x1];
                }
            }
        }
    }
    cnt[0] == cnt[1]
}

fn solve(input: &mut Input, out: &mut Output) {
    let n = input.usize();
    let mut s = vec![];
    for i in 0..n {
        // s.push(input.string());
        let mut row = vec![0u8; n];
        for i in 0..n {
            if i < n / 2 {
                row[i] = b'0';
            } else {
                row[i] = b'1';
            }
        }
        s.push(row);
    }
}

fn extend(s: &[Vec<u8>], rnd: &mut Random) -> Vec<Vec<u8>> {
    let n = s.len() + 4;
    let mut ns = vec![vec![b'0'; n]; n];
    // for i in 0..4 {
    //     for j in 0..4 {
    //         ns[i][j] = if rnd.gen_bool() { b'0' } else { b'1' };
    //     }
    // }
    for i in 0..2 {
        for j in 0..2 {
            ns[i][j] = if i == 0 { b'0' } else { b'1' };
            ns[i + 2][j + 2] = ns[i][j];
            ns[i][j + 2] = ns[i][j];
            ns[i + 2][j] = ns[i][j];
        }
    }
    for i in 0..2 {
        for j in 4..n {
            // ns[i][j] = b'0';
            // ns[j][i] = b'1';
            ns[i][j] = if rnd.gen_bool() { b'0' } else { b'1' };
            ns[j][i] = if rnd.gen_bool() { b'0' } else { b'1' };
            ns[i + 2][j] = ns[i][j];
            ns[j][i + 2] = ns[j][i];
        }
    }
    for i in 4..n {
        for j in 4..n {
            ns[i][j] = s[i - 4][j - 4];
        }
    }
    ns
}

fn extend2(s: &[Vec<u8>]) -> Vec<Vec<u8>> {
    let n = s.len() + 2;
    let mut ns = vec![vec![b'0'; n]; n];
    for i in (0..n - 1).step_by(2) {
        // dbg!(i);
        ns[i][0] = b'0';
        ns[i][1] = b'0';
        ns[i + 1][0] = b'1';
        ns[i + 1][1] = b'1';
        ns[0][i] = b'0';
        ns[0][i + 1] = b'0';
        ns[1][i] = b'1';
        ns[1][i + 1] = b'1';
    }
    // dbg!(ns[0][0], ns[1][0]);
    {
        ns[0][n - 3] = b'1';
        ns[0][n - 2] = b'0';
        ns[0][n - 1] = b'1';
        ns[1][n - 3] = b'0';
        ns[1][n - 2] = b'1';
        ns[1][n - 1] = b'0';
    }
    for j in 0..3 {
        for i in 0..2 {
            ns[n - 3 + j][i] = b'1';
        }
    }
    for i in 0..n - 2 {
        for j in 0..n - 2 {
            ns[i + 2][j + 2] = s[i][j];
        }
    }
    ns
}

fn stress() {
    let n = 5;
    let mut s = vec![vec![b'0'; n]; n];

    let mut seen = HashSet::new();
    let mut done = 0;
    for iter in 0.. {
        // dbg!(":@@@");
        let mut rnd = Random::new(iter);
        let mut zz = 0;
        for i in 0..n {
            for j in 0..n {
                if i <= j {
                    s[i][j] = if ((iter >> zz) & 1) == 1 { b'0' } else { b'1' };

                    zz += 1;
                } else {
                    s[i][j] = s[j][i];
                }
            }
        }
        if calc(&s) {
            // let ns = extend(&s, &mut rnd);
            // let ns2 = extend2(&extend2(&extend2(&extend2(&s))));
            // dbg!(calc(&ns2));
            // if !calc(&ns) {
            //     continue;
            // }
            if !seen.insert(s.clone()) {
                continue;
            }
            println!("--------------");
            for i in 0..n {
                for j in 0..n {
                    print!("{}", s[i][j] as char);
                }
                println!();
            }
            println!();
            done += 1;
            if done > 20 {
                unreachable!()
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
    const PROBLEM_NAME: &str = "a_divide_grid";
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
