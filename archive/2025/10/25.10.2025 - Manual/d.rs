//{"name":"d","group":"Manual","url":"","interactive":false,"timeLimit":2000,"tests":[{"input":"","output":""}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::fx_hash_map::FxHashSet;
#[allow(unused)]
use algo_lib::dbg;
use algo_lib::graph::dsu::Dsu;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;

fn solve(input: &mut Input, out: &mut Output) {
    let tc = input.usize();
    for _ in 0..tc {
        let n = input.usize();
        let m = input.usize();
        let q = input.usize();
        let mut edges = vec![];
        for _ in 0..m {
            let x = input.usize();
            let y = input.usize();
            if x < y {
                edges.push((x, y));
            } else {
                edges.push((y, x));
            }
        }
        edges.sort();
        let mut queries = vec![];
        for _ in 0..q {
            let x = input.usize();
            let y = input.usize();
            queries.push((x, y));
        }

        let mut dsu = Dsu::new(n);
        let mut alive = vec![false; m];
        // return true if connected
        let mut check = |first_queries: usize| -> bool {
            dsu.clear();
            for x in alive.iter_mut() {
                *x = true;
            }
            let mut uu = 0;
            let mut vv = 0;
            let mut hm = FxHashSet::default();
            for i in 0..first_queries {
                let mut u = uu + queries[i].0;
                let mut v = vv + queries[i].1;
                if u >= n {
                    u -= n;
                }
                if v >= n {
                    v -= n;
                }
                let (u, v) = if u < v { (u, v) } else { (v, u) };
                hm.insert((u, v));
                // if let Ok(pos) = edges.binary_search(&(u, v)) {
                //     alive[pos] = false;
                // }
                uu = uu * 2 + 1;
                vv = vv * 3 + 1;
                while uu >= n {
                    uu -= n;
                }
                while vv >= n {
                    vv -= n;
                }
            }
            for i in 0..m {
                if !hm.contains(&edges[i]) {
                    let (u, v) = edges[i];
                    dsu.unite(u, v);
                }
            }
            dsu.num_components() == 1
        };
        let first_disconnect = if !check(0) {
            0
        } else {
            let mut left = 0;
            let mut right = q + 1;
            // invariant: after first left queries, graph is still connected
            while right - left > 1 {
                let mid = (left + right) / 2;
                if check(mid) {
                    left = mid;
                } else {
                    right = mid;
                }
            }
            right
        };
        for i in 1..=q {
            if i < first_disconnect {
                out.println(1);
            } else {
                out.println(0);
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
    const PROBLEM_NAME: &str = "d";
    use algo_lib::tester::helper::*;

    run_tests(PROBLEM_NAME, run);
    // run_single_test(PROBLEM_NAME, run, "2");
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
