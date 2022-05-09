//{"name":"C. Race","group":"Yandex - Stage 15: Grand Prix of Yuquan","url":"https://official.contest.yandex.com/opencupXXII/contest/37831/problems/C/","interactive":false,"timeLimit":1000,"tests":[{"input":"7 9 3 4\n1 2 1\n2 3 1\n3 1 2\n1 4 3\n5 6 2\n6 7 1\n6 7 3\n7 7 2\n5 5 1\n6 7\n1 4\n2 4\n2 5\n","output":"Yes\nNo\nYes\nNo\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"CRace"}}}

use algo_lib::graph::dsu::Dsu;
use algo_lib::graph::edges::edge_trait::EdgeTrait;
use algo_lib::graph::edges::edge_with_info::EdgeWithInfo;
use algo_lib::graph::graph_trait::GraphTrait;
use algo_lib::graph::simple_graph::SimpleGraphT;
use algo_lib::io::output::output;
use algo_lib::io::task_io_settings::TaskIoType;
use algo_lib::io::task_runner::run_task;
use algo_lib::io::{input::Input, task_io_settings::TaskIoSettings};
use algo_lib::misc::rec_function::{Callable, RecursiveFunction};
#[allow(unused)]
use algo_lib::{dbg, out, out_line};

type E = EdgeWithInfo<usize>;

#[derive(Clone)]
struct Gauss {
    ids: Vec<usize>,
    n: usize,
}

impl Gauss {
    pub fn new(n: usize) -> Self {
        Self { ids: vec![0; n], n }
    }

    pub fn add_value(&mut self, mut x: usize) {
        for bit in 0..self.n {
            if ((1 << bit) & x) != 0 {
                if self.ids[bit] == 0 {
                    self.ids[bit] = x;
                    break;
                }
                x ^= self.ids[bit];
            }
        }
    }

    pub fn can_make(&self, mut x: usize) -> bool {
        for bit in 0..self.n {
            if ((1 << bit) & x) != 0 {
                if self.ids[bit] == 0 {
                    return false;
                }
                x ^= self.ids[bit];
            }
        }
        x == 0
    }
}

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.usize();
    let m = input.usize();
    let k = input.usize();
    let q = input.usize();
    let mut g = SimpleGraphT::new(n);
    for _ in 0..m {
        let fr = input.usize() - 1;
        let to = input.usize() - 1;
        let id = input.usize() - 1;
        g.add_complex_bi_edge(fr, E::new(to, id));
    }
    let mut dsu = Dsu::new(n);
    for (fr, e) in g.all_edges() {
        dsu.unite(fr, e.to());
    }
    let full_mask = (1 << k) - 1;
    let mut present_mask = vec![0; n];
    for (fr, e) in g.all_edges() {
        present_mask[dsu.get(fr)] |= 1 << e.info;
    }
    let mut balance = vec![0; n];
    let mut seen = vec![false; n];
    let mut gauses = vec![Gauss::new(k); n];
    let mut go = RecursiveFunction::new(|f, v: usize| {
        if seen[v] {
            return;
        }
        seen[v] = true;
        for e in g.adj(v) {
            let to = e.to();
            if seen[to] {
                gauses[dsu.get(v)].add_value(balance[v] ^ balance[to] ^ (1 << e.info));
                continue;
            }
            balance[to] = balance[v] ^ (1 << e.info);
            f.call(to);
        }
    });
    for v in 0..n {
        go.call(v);
    }
    for _ in 0..q {
        let start = input.usize() - 1;
        let end = input.usize() - 1;
        let path_exist = {
            if start == end {
                true
            } else {
                let comp_id = dsu.get(start);
                if dsu.get(end) != comp_id || present_mask[comp_id] != full_mask {
                    false
                } else {
                    let gauss = &gauses[comp_id];
                    let need = balance[start] ^ balance[end];
                    gauss.can_make(need) || gauss.can_make(need ^ full_mask)
                }
            }
        };
        if path_exist {
            out_line!("Yes");
        } else {
            out_line!("No");
        }
    }
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
