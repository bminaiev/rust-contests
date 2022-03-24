//{"name":"J. Just Bootfall","group":"Codeforces - 2021-2022 ACM-ICPC Brazil Subregional Programming Contest","url":"https://codeforces.com/gym/103388/problem/J","interactive":false,"timeLimit":300,"tests":[{"input":"3 3 2 5\n5 2 1\n3 2 8\n1 9 3\n1 2\n1 3\n","output":"14\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"JJustBootfall"}}}

use std::time::Instant;

use algo_lib::collections::array_2d::Array2D;
use algo_lib::flows::min_cost_max_flow::MinCostMaxFlow;
use algo_lib::io::output::output;
use algo_lib::io::task_io_settings::TaskIoType;
use algo_lib::io::task_runner::run_task;
use algo_lib::io::{input::Input, task_io_settings::TaskIoSettings};
use algo_lib::misc::min_max::UpdateMinMax;
use algo_lib::misc::rand::Random;
use algo_lib::misc::simulated_annealing::{SearchFor, SimulatedAnnealing};
#[allow(unused)]
use algo_lib::{dbg, out, out_line};

struct Scorer {
    score: i64,
    pos: Vec<usize>,
    c: i64,
    g: Vec<Vec<usize>>,
    a: Array2D<i64>,
}

impl Scorer {
    pub fn new(n: usize, g: Vec<Vec<usize>>, c: i64, a: Array2D<i64>) -> Self {
        let mut score = 0;
        for i in 0..n {
            score += a[i][0];
        }
        Self {
            score,
            pos: vec![0; n],
            c,
            g,
            a,
        }
    }

    pub fn mv(&mut self, v: usize, to: usize) {
        let old_pos = self.pos[v];
        self.score -= self.a[v][old_pos];
        self.score += self.a[v][to];
        self.pos[v] = to;
        for &another in self.g[v].iter() {
            let another_pos = self.pos[another] as i64;
            let old_delta = (another_pos - old_pos as i64).abs();
            let new_delta = (to as i64 - another_pos).abs();
            self.score -= (new_delta - old_delta) * self.c;
        }
    }
}

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.usize();
    let m = input.usize();
    let k = input.usize();
    let c = input.i64();
    let a = input.matrix::<i64>(n, m);
    let mut g = vec![vec![]; n];
    for _ in 0..k {
        let fr = input.usize() - 1;
        let to = input.usize() - 1;
        g[fr].push(to);
        g[to].push(fr);
    }
    let mut scorer = Scorer::new(n, g, c, a);
    let mut rnd = Random::new(78577);
    let mut res = 0;
    let start = Instant::now();
    for i in 0..n {
        let to = rnd.gen_in_range(0..m);
        scorer.mv(i, to);
    }
    while start.elapsed().as_millis() < 250 {
        let mut to_change = vec![false; n];
        let offset = rnd.gen_double();
        // to_change[rnd.gen_in_range(0..n)] = true;
        for i in 0..n {
            to_change[i] = rnd.gen_double() < offset;
        }
        let mut flow = MinCostMaxFlow::new(1 + n + m + 1);
        for i in 0..m {
            flow.add_edge(1 + n + i, 1 + n + m, 1000, 0);
        }
        let mut edges = Array2D::new(0, n, m);
        for i in 0..n {
            flow.add_edge(0, 1 + i, 1, 0);
            if to_change[i] {
                for pos in 0..m {
                    let mut cost = -scorer.a[i][pos];
                    for &to_v in scorer.g[i].iter() {
                        if !to_change[to_v] {
                            let their_pos = scorer.pos[to_v];
                            let delta = (their_pos as i64 - pos as i64).abs() * scorer.c;
                            cost += delta;
                        }
                    }
                    edges[i][pos] = flow.add_edge(1 + i, 1 + n + pos, 1, cost);
                }
            } else {
                flow.add_edge(1 + i, 1 + n + scorer.pos[i], 1, 0);
            }
        }
        let cost_and_flow = flow.find_min_cost_max_flow(0, 1 + n + m);
        assert_eq!(cost_and_flow.flow, n as i64);
        for i in 0..n {
            if to_change[i] {
                for j in 0..m {
                    if flow.get_edge_flow(edges[i][j]) != 0 {
                        scorer.mv(i, j);
                        res.update_max(scorer.score);
                    }
                }
            }
        }
    }
    out_line!(res);
}

pub(crate) fn run(mut input: Input) -> bool {
    solve(&mut input, 1);
    output().flush();
    input.skip_whitespace();
    input.peek().is_none()
}

#[allow(unused)]
pub fn submit() -> bool {
    let io = TaskIoSettings {
        is_interactive: false,
        input: TaskIoType::Std,
        output: TaskIoType::Std,
    };

    run_task(io, run)
}

//START MAIN
mod tester;

fn main() {
    tester::run_tests();
    // tester::run_single_test("1");
}
//END MAIN
