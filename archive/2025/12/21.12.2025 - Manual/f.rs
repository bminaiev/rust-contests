//{"name":"f","group":"Manual","url":"","interactive":false,"timeLimit":2000,"tests":[{"input":"","output":""}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use std::time::Instant;

#[allow(unused)]
use algo_lib::dbg;
use algo_lib::graph::dsu::Dsu;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::rand::Random;

struct TestCase {
    groups: Vec<Vec<usize>>,
    res: Vec<i32>,
    n: usize,
}

impl TestCase {
    fn iter(&mut self, rnd: &mut Random) {
        let n = self.n;
        for _ in 0..((1 << n) / 10).max(10) {
            let p = rnd.gen_permutation(self.groups.len());
            let mut dsu = Dsu::new(n);
            let start_joins = rnd.gen_range(2..6);
            let mut cost = 0i32;
            self.update(&dsu, cost);
            for need_joins in (1..=start_joins).rev() {
                for &idx in p.iter() {
                    let mut new_dsu = dsu.clone();
                    let group = &self.groups[idx];
                    for i in 0..n {
                        new_dsu.unite(i, group[i]);
                    }
                    if new_dsu.num_components() + need_joins <= dsu.num_components() {
                        dsu = new_dsu;
                        cost += 1;
                        self.update(&dsu, cost);
                    }
                }
            }
        }
    }

    fn update(&mut self, dsu: &Dsu, cost: i32) {
        let ncomps = dsu.num_components();
        if self.res[ncomps] > cost {
            self.res[ncomps] = cost;
        }
    }
}

fn solve(input: &mut Input, out: &mut Output) {
    let tc = input.usize();
    let mut tests = vec![];
    let start = Instant::now();
    for _ in 0..tc {
        let n = input.usize();
        let k = input.usize();
        let mut groups = vec![];
        for _ in 0..k {
            let mut dsu = Dsu::new(n);
            let cnt = input.usize();
            for _ in 0..cnt {
                let x = input.usize() - 1;
                let y = input.usize() - 1;
                dsu.unite(x, y);
            }
            let mut pp = vec![0; n];
            for i in 0..n {
                pp[i] = dsu.get(i);
            }
            groups.push(pp);
        }
        tests.push(TestCase {
            groups,
            res: vec![i32::MAX; n + 1],
            n,
        });
    }
    let mut rnd = Random::new(787788);
    while start.elapsed().as_secs_f64() < 2.7 {
        for t in tests.iter_mut() {
            t.iter(&mut rnd);
        }
    }
    for t in tests.iter_mut() {
        for i in 0..t.res.len() - 1 {
            if t.res[i] < t.res[i + 1] {
                t.res[i + 1] = t.res[i];
            }
        }
        for x in t.res.iter_mut() {
            if *x == i32::MAX {
                *x = -1;
            }
        }
        out.println(t.res[1..].to_vec());
    }
}

pub(crate) fn run(mut input: Input, mut output: Output) -> bool {
    solve(&mut input, &mut output);
    output.flush();
    true
}

#[cfg(feature = "local")]
fn main() {
    const PROBLEM_NAME: &str = "f";
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
