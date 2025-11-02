//{"name":"b","group":"Manual","url":"","interactive":false,"timeLimit":2000,"tests":[{"input":"","output":""},{"input":"","output":""}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

#[allow(unused)]
use algo_lib::dbg;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;

const MX: usize = 100;

struct Knapsack {
    maxs: Vec<Vec<u64>>,
}

impl Knapsack {
    fn new() -> Self {
        let mut maxs = vec![];
        for cnt in 0..=5 {
            maxs.push(vec![0; MX * cnt + 1]);
        }
        Self { maxs }
    }

    fn add_item(&mut self, a: usize, b: u64) {
        assert!(a <= MX);
        for cnt in (1..=5).rev() {
            for w in (a..=MX * (cnt - 1) + a).rev() {
                if w - a != 0 && self.maxs[cnt - 1][w - a] == 0 {
                    continue;
                }
                self.maxs[cnt][w] = self.maxs[cnt][w].max(self.maxs[cnt - 1][w - a] + b);
            }
        }
    }

    fn get_intererstings(&self, k: usize) -> Vec<(u64, u64)> {
        let mut res = vec![];
        for i in 0..self.maxs[k].len() {
            if i == 0 || self.maxs[k][i] != 0 {
                res.push((i as u64, self.maxs[k][i]));
            }
        }
        res
    }
}

fn solve(input: &mut Input, out: &mut Output) {
    let tc = input.usize();
    for _ in 0..tc {
        let n = input.usize();
        let k = input.usize().min(n);
        let mut knapsack_a = Knapsack::new();
        let mut knapsack_b = Knapsack::new();
        for _ in 0..n {
            let a = input.usize();
            let b = input.usize();
            if a <= MX {
                knapsack_a.add_item(a, b as u64);
            } else {
                knapsack_b.add_item(b, a as u64);
            }
        }
        let mut ans = 0u64;
        for k1 in 0..=k {
            let k2 = k - k1;
            let inter_a = knapsack_a.get_intererstings(k1);
            let inter_b = knapsack_b.get_intererstings(k2);
            for &(i, va) in inter_a.iter() {
                for &(j, vb) in inter_b.iter() {
                    let value = (i + vb) * (j + va);
                    ans = ans.max(value);
                }
            }
        }
        out.println(ans);
    }
}

pub(crate) fn run(mut input: Input, mut output: Output) -> bool {
    solve(&mut input, &mut output);
    output.flush();
    true
}

#[cfg(feature = "local")]
fn main() {
    const PROBLEM_NAME: &str = "b";
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
