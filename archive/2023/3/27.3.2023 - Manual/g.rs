//{"name":"g","group":"Manual","url":"","interactive":false,"timeLimit":2000,"tests":[{"input":"","output":""},{"input":"","output":""}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"g"}}}

use std::cmp::max;

use algo_lib::graph::edges::edge_trait::EdgeTrait;
use algo_lib::graph::graph_readers::weighted::read_weighted_undirected_graph;
use algo_lib::graph::graph_trait::GraphTrait;
use algo_lib::io::output::output;
use algo_lib::io::task_io_settings::TaskIoType;
use algo_lib::io::task_runner::run_task;
use algo_lib::io::{input::Input, task_io_settings::TaskIoSettings};
use algo_lib::misc::min_max::UpdateMinMax;
use algo_lib::misc::rec_function::{Callable2, RecursiveFunction2};
#[allow(unused)]
use algo_lib::{dbg, out, out_line};

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.usize();
    let k = input.usize();
    let g = read_weighted_undirected_graph::<i64>(input, n, n - 1);

    let mut sz = vec![0; n];
    const NEG_INF: i64 = -100; //std::i64::MIN / 10;

    let dp = RecursiveFunction2::new(|f, v: usize, p: usize| -> Vec<Vec<i64>> {
        sz[v] = 1;
        let mut dp = vec![vec![NEG_INF; n + 1]; 3];
        dp[0][0] = 0;
        for e in g.adj(v) {
            if e.to() == p {
                continue;
            }
            let child = f.call(e.to(), v);
            let mut ndp = vec![vec![NEG_INF; n + 1]; 3];
            for cnt0 in 0..=sz[v] {
                for st0 in 0..3 {
                    for cnt1 in 0..=sz[e.to()] {
                        for st1 in 0..3 {
                            let mut cost = dp[st0][cnt0] + child[st1][cnt1];

                            if st1 >= 1 {
                                // not connect to subtree
                                let cost = cost + e.cost;
                                ndp[st0][cnt0 + cnt1].update_max(cost);
                            }

                            let nst = st0 + st1;

                            if st1 == 1 {
                                cost += e.cost;
                            }

                            if nst <= 2 {
                                let mut ncnt = cnt0 + cnt1;
                                if st0 == 1 && st1 == 1 {
                                    if ncnt > 0 {
                                        // hacky, but should work?
                                        ncnt -= 1;
                                    }
                                }
                                ndp[nst][ncnt].update_max(cost);
                            }
                        }
                    }
                }
            }
            dp = ndp;
            sz[v] += sz[e.to()];
        }
        // start a new path
        for cnt in (0..sz[v]).rev() {
            let val = dp[0][cnt];
            dp[1][cnt + 1].update_max(val);
        }
        dp
    })
    .call(0, 0);

    let mut res = 0;
    for paths in 1..=k + 1 {
        if paths < dp[0].len() {
            let cur = max(dp[1][paths], dp[2][paths]);
            res.update_max(cur);
        }
        out!(res, "");
    }
    out_line!();
}

pub(crate) fn run(mut input: Input) -> bool {
    solve(&mut input, 1);
    output().flush();
    true
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
    // tester::run_stress(stress);
    // tester::run_locally();
    // tester::run_with_last_downloaded_file();
}
//END MAIN
