//{"name":"d","group":"Manual","url":"","interactive":false,"timeLimit":2000,"tests":[{"input":"","output":""}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

#[allow(unused)]
use algo_lib::dbg;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::rand::Random;
use algo_lib::misc::rec_function::{Callable, RecursiveFunction};

fn solve_slow(a: &[usize]) -> Vec<usize> {
    let n = a.len();
    let mut best = vec![n; n];
    let mut a = a.to_vec();
    RecursiveFunction::new(|f, pos: usize| {
        if pos == n {
            let mut b = vec![];
            for i in 0..n {
                b.push(a[a[i]] + 1);
            }
            if b < best {
                best = b;
            }
        } else if a[pos] == n {
            for i in 0..n {
                a[pos] = i;
                f.call(pos + 1);
            }
            a[pos] = n;
        } else {
            f.call(pos + 1);
        }
    })
    .call(0);
    best
}

fn solve_fast(a: &[usize]) -> Vec<usize> {
    let n = a.len();
    let mut a = a.to_vec();
    if a[0] == n || a[0] == 0 {
        for i in 0..n {
            if a[i] == n {
                a[i] = 0;
            }
        }
    } else {
        let pos_zero = a[0];
        if a[pos_zero] == n {
            a[pos_zero] = 0;
        }
        for i in 0..n {
            if a[i] == 0 {
                for j in 0..n {
                    if a[j] == n {
                        a[j] = i;
                    }
                }
                break;
            } else if a[i] != n {
                let nxt = a[i];
                if a[nxt] == n && nxt > i {
                    a[nxt] = 0;
                    for j in 0..n {
                        if a[j] == n {
                            a[j] = i;
                        }
                    }
                    break;
                }
            }
        }
        let mut last_n = n;
        let mut first_n = n;
        for i in 0..n {
            if a[i] == n {
                last_n = i;
                if first_n == n {
                    first_n = i;
                }
            }
        }
        if last_n != n {
            if first_n == last_n {
                a[first_n] = first_n;
                for i in 0..n {
                    if a[i] < a[first_n] {
                        a[first_n] = i;
                    }
                }
            } else {
                a[last_n] = 0;
                for i in 0..n {
                    if a[i] == n {
                        a[i] = last_n;
                    }
                }
            }
        }
    }
    let mut res = vec![];
    for i in 0..n {
        res.push(a[a[i]] + 1);
    }
    res
}

fn solve(input: &mut Input, out: &mut Output) {
    let tc = input.usize();
    for _ in 0..tc {
        let n = input.usize();
        let mut a = vec![];
        for _i in 0..n {
            let x = input.i32();
            if x == -1 {
                a.push(n);
            } else {
                a.push(x as usize - 1);
            }
        }
        let res = solve_slow(&a);
        out.println(res);
    }
}

fn stress() {
    for it in 1.. {
        dbg!(it);
        let mut rnd = Random::new(it);
        const MAX_N: usize = 6;
        let n = rnd.gen_range(1..MAX_N);
        let mut a = vec![];
        let frac = rnd.gen_double();
        for _ in 0..n {
            if rnd.gen_double() < frac {
                a.push(n);
            } else {
                a.push(rnd.gen_range(0..n));
            }
        }
        let res_slow = solve_slow(&a);
        let res_fast = solve_fast(&a);
        if res_slow != res_fast {
            assert!(res_slow < res_fast);
            // if a[0] != n {
            //     continue;
            // }
            dbg!(n, &a);
            dbg!(res_slow);
            dbg!(res_fast);
            unreachable!();
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
    const PROBLEM_NAME: &str = "d";
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
