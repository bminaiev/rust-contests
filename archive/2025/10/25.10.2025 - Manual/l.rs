//{"name":"l","group":"Manual","url":"","interactive":false,"timeLimit":2000,"tests":[{"input":"","output":""}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::array_2d::Array2D;
#[allow(unused)]
use algo_lib::dbg;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::rand::Random;

fn solve_slow(g: &[Vec<usize>]) -> usize {
    let n = g.len();
    let mut dist = vec![vec![usize::MAX / 2; n]; n];
    for i in 0..n {
        dist[i][i] = 0;
    }
    for i in 0..n {
        for &j in g[i].iter() {
            dist[i][j] = 1;
        }
    }
    for i in 0..n {
        for j in 0..n {
            for k in 0..n {
                dist[j][k] = dist[j][k].min(dist[j][i] + dist[i][k]);
            }
        }
    }
    for res in 1.. {
        let mut seen = vec![vec![false; 1 << n]; n];
        for v in 0..n {
            seen[v][1 << v] = true;
        }
        for mask in 0..(1 << n) {
            for v in 0..n {
                if !seen[v][mask] {
                    continue;
                }
                for u in 0..n {
                    if (mask & (1 << u)) != 0 {
                        continue;
                    }
                    if dist[v][u] <= res {
                        seen[u][mask | (1 << u)] = true;
                    }
                }
            }
        }
        let need_mask = (1 << n) - 1;
        for v in 0..n {
            if seen[v][need_mask] {
                return res;
            }
        }
    }
    unreachable!();
}

fn solve(input: &mut Input, out: &mut Output) {
    let tc = input.usize();
    for _ in 0..tc {
        let n = input.usize();
        let mut g = vec![vec![]; n];
        for _ in 0..n - 1 {
            let fr = input.usize() - 1;
            let to = input.usize() - 1;
            g[fr].push(to);
            g[to].push(fr);
        }
        let res = solve_slow(&g);
        out.println(res);
    }
}

fn stress() {
    for it in 100000.. {
        dbg!(it);
        let mut rnd = Random::new(it);
        let n = rnd.gen_range(1..20);
        let mut p = vec![0; n];
        for i in 1..n {
            p[i] = rnd.gen_range(0..i);
        }
        let mut g = vec![vec![]; n];
        for i in 1..n {
            g[i].push(p[i]);
            g[p[i]].push(i);
        }
        let res1 = solve_slow(&g);
        dbg!(it, res1);
        assert!(res1 <= 3);
    }
}

pub(crate) fn run(mut input: Input, mut output: Output) -> bool {
    solve(&mut input, &mut output);
    output.flush();
    true
}

#[cfg(feature = "local")]
fn main() {
    const PROBLEM_NAME: &str = "l";
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
