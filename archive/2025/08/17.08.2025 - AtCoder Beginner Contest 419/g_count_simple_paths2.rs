//{"name":"G - Count Simple Paths 2","group":"AtCoder - AtCoder Beginner Contest 419","url":"https://atcoder.jp/contests/abc419/tasks/abc419_g","interactive":false,"timeLimit":4000,"tests":[{"input":"5 6\n1 2\n1 3\n2 4\n3 4\n3 5\n4 5\n","output":"0 1 2 1\n"},{"input":"11 15\n1 2\n1 3\n2 3\n3 4\n3 5\n4 5\n5 6\n5 7\n6 7\n7 8\n7 9\n8 9\n9 10\n9 11\n10 11\n","output":"0 0 0 0 1 5 10 10 5 1\n"},{"input":"7 18\n6 7\n4 5\n1 7\n2 7\n1 4\n2 5\n4 6\n2 3\n5 6\n5 7\n1 5\n2 4\n2 6\n1 2\n1 3\n3 4\n1 6\n3 5\n","output":"1 3 11 29 50 42\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"GCountSimplePaths2"}}}

#[allow(unused)]
use algo_lib::dbg;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::rand::Random;
use algo_lib::misc::rec_function::{Callable2, RecursiveFunction2};

#[derive(Clone, Copy)]
struct Edge {
    to: usize,
    weight: usize,
}

fn solve_case(g: &[Vec<usize>]) -> Vec<usize> {
    let n = g.len();
    let mut deg = vec![0; n];
    for i in 0..n {
        deg[i] = g[i].len();
    }
    let mut gsum = vec![0; n];
    for i in 0..n {
        for &j in &g[i] {
            gsum[i] += j;
        }
    }

    // remove all vertices with deg = 1, until they exist. Do not remove 0 or n - 1.
    let mut to_remove = vec![];
    for i in 1..n - 1 {
        if deg[i] == 1 && i != 0 && i != n - 1 {
            to_remove.push(i);
        }
    }
    let mut removed = vec![false; n];
    while let Some(i) = to_remove.pop() {
        removed[i] = true;
        for &j in &g[i] {
            deg[j] -= 1;
            gsum[j] -= i;
            if deg[j] == 1 && j != 0 && j != n - 1 {
                to_remove.push(j);
            }
        }
    }
    // now contract all vertices with deg = 2, repace with weighted edge.
    let mut new_g = vec![vec![]; n];
    let mut skip = vec![false; n];
    for i in 1..n - 1 {
        if deg[i] == 2 {
            skip[i] = true;
        }
    }
    for start in 0..n {
        if skip[start] || removed[start] {
            continue;
        }
        for &to in &g[start] {
            if removed[to] {
                continue;
            }
            if skip[to] {
                let mut prev = start;
                let mut cur = to;
                let mut weight = 1;
                while skip[cur] {
                    assert!(deg[cur] == 2);
                    let next = gsum[cur] - prev;
                    prev = cur;
                    cur = next;
                    weight += 1;
                }
                new_g[start].push(Edge { to: cur, weight });
            } else {
                new_g[start].push(Edge { to, weight: 1 });
            }
        }
    }
    let mut res = vec![0; n];
    let mut seen = vec![false; n];
    seen[0] = true;
    RecursiveFunction2::new(|f, v: usize, len: usize| {
        if v == n - 1 {
            res[len] += 1;
            return;
        }
        for e in &new_g[v] {
            if seen[e.to] {
                continue;
            }
            seen[e.to] = true;
            f.call(e.to, len + e.weight);
            seen[e.to] = false;
        }
    })
    .call(0, 0);
    res
}

fn solve(input: &mut Input, out: &mut Output, _test_case: usize) {
    let n = input.usize();
    let m = input.usize();
    let mut g = vec![vec![]; n];
    let mut deg = vec![0; n];
    for _ in 0..m {
        let u = input.usize() - 1;
        let v = input.usize() - 1;
        g[u].push(v);
        g[v].push(u);
        deg[u] += 1;
        deg[v] += 1;
    }

    out.println(solve_case(&g)[1..].to_vec());
}

fn stress() {
    for it in 1.. {
        dbg!(it);
        let mut rnd = Random::new(it);
        let n = rnd.gen(2..20);
        let m = rnd.gen(1..20);
        let mut g = vec![vec![]; n];
        for _ in 0..m {
            let u = rnd.gen(0..n);
            let v = rnd.gen(0..n);
            g[u].push(v);
            g[v].push(u);
        }
        solve_case(&g);
    }
}

pub(crate) fn run(mut input: Input, mut output: Output) -> bool {
    solve(&mut input, &mut output, 1);
    output.flush();
    true
}

//START MAIN

fn main() {
    const PROBLEM_NAME: &str = "g_count_simple_paths2";
    use algo_lib::tester::helper::*;

    run_tests(PROBLEM_NAME, run);
    // run_single_test(PROBLEM_NAME, run, "1");
    // run_stress(stress);
    // run_locally(run);
}
//END MAIN
