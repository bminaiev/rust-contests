//{"name":"A - Big Clique Everywhere","group":"AtCoder - AtCoder Grand Contest 067","url":"https://atcoder.jp/contests/agc067/tasks/agc067_a","interactive":false,"timeLimit":4000,"tests":[{"input":"4\n3 3\n1 2\n1 3\n2 3\n3 2\n1 2\n1 3\n3 1\n1 2\n3 0\n","output":"Yes\nYes\nYes\nNo\n"}],"testType":"multiNumber","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"ABigCliqueEverywhere"}}}

use std::collections::{BTreeSet, HashSet};

#[allow(unused)]
use algo_lib::dbg;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::rand::Random;

fn good(n: usize, edges: &HashSet<(usize, usize)>) -> bool {
    let mut alive = BTreeSet::new();
    for i in 0..n {
        alive.insert(i);
    }
    while !alive.is_empty() {
        let first = *alive.iter().next().unwrap();
        alive.remove(&first);
        let mut not_connected = vec![];
        for &i in alive.iter() {
            if !edges.contains(&(first, i)) {
                not_connected.push(i);
            }
        }
        for &x in not_connected.iter() {
            alive.remove(&x);
        }
        for i in 0..not_connected.len() {
            for j in i + 1..not_connected.len() {
                if !edges.contains(&(not_connected[i], not_connected[j])) {
                    return false;
                }
            }
        }
    }
    true
}

fn good3(n: usize, edges: &HashSet<(usize, usize)>) -> bool {
    for mask in 1usize..(1 << n) {
        if mask.count_ones() > 5 {
            continue;
        }
        let mut exist = false;
        for submask in 0usize..(1 << n) {
            if (mask & submask) != submask {
                continue;
            }
            if (submask.count_ones() * 2) < mask.count_ones() {
                continue;
            }
            let mut good = true;
            for i in 0..n {
                if (submask & (1 << i)) == 0 {
                    continue;
                }
                for j in i + 1..n {
                    if (submask & (1 << j)) == 0 {
                        continue;
                    }
                    if !edges.contains(&(i, j)) {
                        good = false;
                        break;
                    }
                }
            }
            if good {
                exist = true;
                break;
            }
        }
        if !exist {
            return false;
        }
    }
    true
}

fn solve(input: &mut Input, out: &mut Output, _test_case: usize) {
    let n = input.usize();
    let m = input.usize();
    let mut edges = HashSet::new();
    for _ in 0..m {
        let a = input.usize() - 1;
        let b = input.usize() - 1;
        edges.insert((a, b));
        edges.insert((b, a));
    }
    if good(n, &edges) {
        out.println("Yes");
    } else {
        out.println("No");
    }
}

fn slow_good(n: usize, edges: &HashSet<(usize, usize)>) -> bool {
    for mask in 1..(1 << n) {
        let mut exist = false;
        for submask in 0usize..(1 << n) {
            if (mask & submask) != submask {
                continue;
            }
            if (submask.count_ones() * 2) < mask.count_ones() {
                continue;
            }
            let mut good = true;
            for i in 0..n {
                if (submask & (1 << i)) == 0 {
                    continue;
                }
                for j in i + 1..n {
                    if (submask & (1 << j)) == 0 {
                        continue;
                    }
                    if !edges.contains(&(i, j)) {
                        good = false;
                        break;
                    }
                }
            }
            if good {
                exist = true;
                break;
            }
        }
        if !exist {
            return false;
        }
    }
    true
}

fn stress() {
    for it in 1.. {
        dbg!(it);
        let mut rnd = Random::new(it);
        let n = rnd.gen(1..10);
        let mut edges = HashSet::new();
        let prob = rnd.gen_double();
        for i in 0..n {
            for j in i + 1..n {
                if rnd.gen_double() < prob {
                    edges.insert((i, j));
                    edges.insert((j, i));
                }
            }
        }
        let my_good = good3(n, &edges);
        let slow_good = slow_good(n, &edges);
        if my_good != slow_good {
            dbg!(n, my_good, slow_good);
            for i in 0..n {
                for j in i + 1..n {
                    if edges.contains(&(i, j)) {
                        dbg!(i, j);
                    }
                }
            }
            break;
        }
        assert_eq!(my_good, slow_good);
    }
}

pub(crate) fn run(mut input: Input, mut output: Output) -> bool {
    let t = input.read();
    for i in 0usize..t {
        solve(&mut input, &mut output, i + 1);
    }
    output.flush();
    true
}

//START MAIN

fn main() {
    const PROBLEM_NAME: &str = "a_big_clique_everywhere";
    use algo_lib::tester::helper::*;

    // run_tests(PROBLEM_NAME, run);
    // run_single_test(PROBLEM_NAME, run, "1");
    run_stress(stress);
    // run_locally(run);
}
//END MAIN
