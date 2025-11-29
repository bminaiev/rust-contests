//{"name":"G. Запросы Исаака","group":"Codeforces - Codeforces Round 1066 (Div. 1 + Div. 2)","url":"https://codeforces.com/contest/2157/problem/G","interactive":true,"timeLimit":4000,"tests":[{"input":"1\n3\n\n2\n\n-1\n\n1\n","output":"\n? 1 2\n\n? 1 3\n\n? 2 3\n\n!\n1 2 -1\n2 1\n2\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use std::usize;

use algo_lib::collections::array_2d::Array2D;
#[allow(unused)]
use algo_lib::dbg;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::rand::Random;

fn solve(input: &mut Input, out: &mut Output) {
    let tc = input.usize();
    for _ in 0..tc {
        let n = input.i32();
        if n == -2 {
            return;
        }
        let n = n as usize;
        let mut by_len = vec![];
        for l in 0..n {
            for r in l + 1..=n {
                by_len.push((l, r));
            }
        }
        by_len.sort_by_key(|(l, r)| r - l);
        by_len.reverse();

        let mut known = Array2D::new(i32::MAX, n, n + 1);
        let mut triples = vec![vec![vec![]; n + 1]; n + 1];

        for l in 0..n {
            for m in l + 1..=n {
                for r in m + 1..=n {
                    let tr = [(l, m), (m, r), (l, r)];
                    triples[l][m].push(tr);
                    triples[m][r].push(tr);
                    triples[l][r].push(tr);
                }
            }
        }

        for (l, r) in by_len {
            if known[l][r] != i32::MAX {
                continue;
            }
            out.println(format!("? {} {}", l + 1, r));
            out.flush();
            known[l][r] = input.i32();
            for &segs in &triples[l][r] {
                for is_known in [[0, 1], [0, 2], [1, 2]] {
                    let mut here = vec![];
                    for &i in is_known.iter() {
                        let (start, end) = segs[i];
                        let k = known[start][end];
                        if k == i32::MAX {
                            continue;
                        }
                        here.push(k);
                    }
                    if here.len() == 2 && here[0] != here[1] {
                        let missing_idx = 3 - is_known[0] - is_known[1];
                        let (start, end) = segs[missing_idx];
                        let missing_k = here[0].max(here[1]);
                        if known[start][end] == i32::MAX {
                            known[start][end] = missing_k;
                        }
                    }
                }
            }
        }
        out.println("!");
        for l in 0..n {
            let mut row = vec![];
            for r in l + 1..=n {
                row.push(known[l][r]);
            }
            out.println(row);
        }
        out.flush();
    }
}

fn stress() {
    for it in 1.. {
        let mut rnd = Random::new(it);
        let n = 100;
        let mut a = vec![];
        for _ in 0..n {
            a.push(rnd.gen_range(0..1 << 30));
        }
        let mut sum_cost = 0.0;
        let mut answers = Array2D::new(0, n, n + 1);
        for l in 0..n {
            let mut row = vec![];
            for r in l + 1..=n {
                let mut xor = 0;
                for i in l..r {
                    xor ^= a[i];
                }
                let log2 = (xor as f64).log2().trunc() as usize;
                answers[l][r] = log2;
                row.push(log2);
                // let query_cost = 1.0 / ((r - l) as f64);
                // if r - l > 50 {
                //     sum_cost += query_cost;
                // }
            }
            // dbg!(row);
        }

        let mut by_len = vec![];
        for l in 0..n {
            for r in l + 1..=n {
                by_len.push((l, r));
            }
        }
        by_len.sort_by_key(|(l, r)| r - l);
        by_len.reverse();

        let mut known = Array2D::new(usize::MAX, n, n + 1);
        let mut triples = vec![vec![vec![]; n + 1]; n + 1];

        for l in 0..n {
            for m in l + 1..=n {
                for r in m + 1..=n {
                    let tr = [(l, m), (m, r), (l, r)];
                    triples[l][m].push(tr);
                    triples[m][r].push(tr);
                    triples[l][r].push(tr);
                }
            }
        }

        let mut cost = 0.0;
        for (l, r) in by_len {
            if known[l][r] != usize::MAX {
                continue;
            }
            cost += 1.0 / ((r - l) as f64);
            known[l][r] = answers[l][r];
            for &segs in &triples[l][r] {
                for is_known in [[0, 1], [0, 2], [1, 2]] {
                    let mut here = vec![];
                    for &i in is_known.iter() {
                        let (start, end) = segs[i];
                        let k = known[start][end];
                        if k == usize::MAX {
                            continue;
                        }
                        here.push(k);
                    }
                    if here.len() == 2 && here[0] != here[1] {
                        let missing_idx = 3 - is_known[0] - is_known[1];
                        let (start, end) = segs[missing_idx];
                        let missing_k = here[0].max(here[1]);
                        if known[start][end] == usize::MAX {
                            known[start][end] = missing_k;
                            assert_eq!(known[start][end], answers[start][end]);
                        }
                    }
                }
            }
        }
        dbg!(cost);
    }
}

pub(crate) fn run(mut input: Input, mut output: Output) -> bool {
    solve(&mut input, &mut output);
    output.flush();
    true
}

#[cfg(feature = "local")]
fn main() {
    const PROBLEM_NAME: &str = "g_";
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
