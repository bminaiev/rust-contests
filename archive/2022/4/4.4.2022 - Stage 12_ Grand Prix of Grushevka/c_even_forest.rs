//{"name":"C. Even Forest","group":"Yandex - Stage 12: Grand Prix of Grushevka","url":"https://official.contest.yandex.com/opencupXXII/contest/35268/problems/C/","interactive":false,"timeLimit":3000,"tests":[{"input":"4\n1 2\n2 3\n3 4\n","output":"1\n"},{"input":"4\n1 2\n1 3\n1 4\n","output":"0\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"CEvenForest"}}}

use std::cmp::min;

use algo_lib::collections::array_2d::Array2D;
use algo_lib::graph::edges::edge_trait::EdgeTrait;
use algo_lib::graph::graph_readers::config::{Directional, Indexation};
use algo_lib::graph::graph_readers::simple::read_graph;
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
    let graph = read_graph(
        input,
        n,
        n - 1,
        Directional::Undirected,
        Indexation::FromOne,
    );
    const MAX: i32 = std::i32::MAX / 2 - 10;
    // 0 - fully remove
    // 1 - odd
    // 2 - even
    let mut g_dp = Array2D::new(MAX, n, 3);
    RecursiveFunction2::new(|f, v: usize, p: usize| {
        // cntOdd, cntEven
        let mut dp = Array2D::new(MAX, 3, 3);
        dp[0][0] = 0;
        for e in graph.adj(v) {
            let to = e.to();
            if to == p {
                continue;
            }
            f.call(to, v);
            let mut ndp = Array2D::new(MAX, 3, 3);
            for odd in 0..=2 {
                for even in 0..=2 {
                    let cur = dp[odd][even];
                    ndp[odd][even].update_min(cur + 1 + g_dp[to][0]);
                    {
                        // add odd
                        let nodd = min(2, odd + 1);
                        ndp[nodd][even].update_min(cur + g_dp[to][1]);
                    }
                    {
                        // add even
                        let neven = min(2, even + 1);
                        ndp[odd][neven].update_min(cur + g_dp[to][2]);
                    }
                }
            }
            dp = ndp;
        }
        for odd in 0..=2 {
            for even in 0..=2 {
                let cur = dp[odd][even];
                if odd != 0 && even != 0 {
                    continue;
                }
                if odd == 0 && even == 0 {
                    g_dp[v][0].update_min(cur);
                    g_dp[v][1].update_min(cur);
                } else {
                    if even != 0 {
                        g_dp[v][0].update_min(cur);
                        g_dp[v][1].update_min(cur);
                    } else {
                        assert_ne!(odd, 0);
                        g_dp[v][2].update_min(cur);
                    }
                }
                if odd == 0 && even == 2 || odd == 2 && even == 0 {
                    g_dp[v][0].update_min(cur);
                }
            }
        }
    })
    .call(0, 0);
    let res = g_dp[0][0];
    assert!(res < MAX);
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
    // tester::run_single_test("2");
}
//END MAIN
