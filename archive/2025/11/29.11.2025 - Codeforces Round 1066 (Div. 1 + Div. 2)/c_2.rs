//{"name":"C. Мексимальный массив 2","group":"Codeforces - Codeforces Round 1066 (Div. 1 + Div. 2)","url":"https://codeforces.com/contest/2157/problem/C","interactive":false,"timeLimit":2000,"tests":[{"input":"4\n6 2 2\n1 1 3\n2 2 6\n3 3 1\n2 1 3\n3 3 2\n1 1 1\n1 3 3\n3 2 2\n2 1 2\n2 2 3\n","output":"2 5 4 3 0 1\n2 0 1\n3 3 3\n1 0 1\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

#[allow(unused)]
use algo_lib::dbg;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;

fn solve(input: &mut Input, out: &mut Output) {
    let tc = input.usize();
    for _ in 0..tc {
        // dbg!("TEST");
        let n = input.usize();
        let k = input.usize();
        let q = input.usize();
        let mut c = vec![];
        let mut left = vec![];
        let mut right = vec![];
        for _ in 0..q {
            c.push(input.usize());
            left.push(input.usize() - 1);
            right.push(input.usize());
        }
        let mut res = vec![usize::MAX; n];
        let mut cant_k = vec![false; n];
        let mut at_least_k = vec![false; n];
        for i in 0..q {
            if c[i] == 1 {
                // min
                for j in left[i]..right[i] {
                    at_least_k[j] = true;
                }
            } else {
                // mex
                for j in left[i]..right[i] {
                    cant_k[j] = true;
                }
            }
        }
        for i in 0..q {
            if c[i] == 1 {
                for j in left[i]..right[i] {
                    if !cant_k[j] {
                        res[j] = k;
                    } else {
                        res[j] = k + 1;
                    }
                }
            }
        }
        // dbg!(res);
        let mut all_mex_queries = vec![];
        for i in 0..q {
            if c[i] == 2 {
                all_mex_queries.push((left[i], right[i]));
            }
        }
        all_mex_queries.sort_by_key(|x| x.1);
        for (l, r) in all_mex_queries {
            let mut seen = vec![false; k];
            for i in l..r {
                if res[i] != usize::MAX && res[i] < k {
                    seen[res[i]] = true;
                }
            }
            let mut it = 0;
            for i in l..r {
                if res[i] == usize::MAX {
                    while it < k && seen[it] {
                        it += 1;
                    }
                    if it < k {
                        res[i] = it;
                        seen[it] = true;
                        it += 1;
                    }
                }
            }
            while it < k && seen[it] {
                it += 1;
            }
            // dbg!(res, l, r);
            assert!(it >= k);
        }
        for i in 0..n {
            if res[i] == usize::MAX {
                res[i] = 0;
            }
        }
        // dbg!(res);
        for i in 0..q {
            if c[i] == 1 {
                let mut mn = usize::MAX;
                for j in left[i]..right[i] {
                    mn = mn.min(res[j]);
                }
                // dbg!(mn, k);
                assert!(mn == k);
            } else {
                let mut seen = vec![false; k];
                for j in left[i]..right[i] {
                    if res[j] < k {
                        seen[res[j]] = true;
                    }
                }
                let mut mex = 0;
                while mex < k && seen[mex] {
                    mex += 1;
                }
                assert!(mex == k);
            }
        }
        out.println(res);
    }
}

pub(crate) fn run(mut input: Input, mut output: Output) -> bool {
    solve(&mut input, &mut output);
    output.flush();
    true
}

#[cfg(feature = "local")]
fn main() {
    const PROBLEM_NAME: &str = "c_2";
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
