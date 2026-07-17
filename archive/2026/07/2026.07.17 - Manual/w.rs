use std::collections::{BTreeSet, VecDeque};

#[allow(unused)]
use algo_lib::dbg;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;

fn solve(input: &mut Input, out: &mut Output) {
    let tc = input.usize();
    for _ in 0..tc {
        let n = input.usize();
        let k = input.usize();
        let h = input.vec::<usize>(n);
        let mut sorted = vec![];
        for i in 0..n {
            sorted.push((h[i], i));
        }
        sorted.sort();
        let mut alive = BTreeSet::new();
        let mut small = vec![false; n];
        for i in 0..k {
            let id = sorted[i].1;
            alive.insert(id);
            small[id] = true;
        }
        for i in 0..k {
            let id = sorted[n - i - 1].1;
            alive.insert(id);
        }
        let mut queue = VecDeque::new();
        for &k1 in alive.iter() {
            if let Some(&k2) = alive.range(k1 + 1..).next() {
                if small[k1] != small[k2] {
                    queue.push_back(k1);
                }
            }
        }
        let mut pairs = vec![];
        while let Some(k1) = queue.pop_front() {
            if let Some(&k2) = alive.range(k1 + 1..).next() {
                if alive.contains(&k1) && alive.contains(&k2) && small[k1] != small[k2] {
                    alive.remove(&k1);
                    alive.remove(&k2);
                    if small[k1] {
                        pairs.push((k2, k1));
                    } else {
                        pairs.push((k1, k2));
                    }
                    if let Some(&k0) = alive.range(..k1).next_back() {
                        if let Some(&k3) = alive.range(k2 + 1..).next() {
                            if small[k0] != small[k3] {
                                queue.push_back(k0);
                            }
                        }
                    }
                }
            }
        }
        assert_eq!(pairs.len(), k);
        for (a, b) in pairs {
            out.println(vec![a, b]);
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
    const PROBLEM_NAME: &str = "w";
    use algo_lib::{misc::dragon::run_dragon, tester::helper::*};

    // run_tests(PROBLEM_NAME, run);
    // run_single_test(PROBLEM_NAME, run, "1");
    // run_stress(stress);
    // run_locally(run);
    run_dragon(solve, "W", 1..5);
}
//END MAIN

#[cfg(not(feature = "local"))]
fn main() {
    let input = algo_lib::io::input::Input::new_stdin();
    let mut output = algo_lib::io::output::Output::new_stdout();
    run(input, output);
}
