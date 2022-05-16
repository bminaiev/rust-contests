//{"name":"Saving the Jelly","group":"Google Coding Competitions - Round 2 2022 - Code Jam 2022","url":"https://codingcompetitions.withgoogle.com/codejam/round/00000000008778ec/0000000000b158f8","interactive":false,"timeLimit":10000,"tests":[{"input":"4\n2\n-3 0\n-1 0\n3 0\n-2 -1\n-2 1\n1\n0 0\n1 1\n2 2\n3\n10 0\n-10 0\n0 0\n0 5\n-1 0\n5 0\n0 -5\n2\n3 4\n3 4\n5 7\n3 4\n5 7\n","output":"Case #1: POSSIBLE\n2 2\n1 3\nCase #2: IMPOSSIBLE\nCase #3: POSSIBLE\n3 2\n2 4\n1 3\nCase #4: POSSIBLE\n1 2\n2 3\n"}],"testType":"multiNumber","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"SavingTheJelly"}}}

use algo_lib::collections::array_2d::Array2D;
use algo_lib::flows::min_cost_max_flow::MinCostMaxFlow;
use algo_lib::geometry::point::PointT;
use algo_lib::io::output::output;
use algo_lib::io::task_io_settings::TaskIoType;
use algo_lib::io::task_runner::run_task;
use algo_lib::io::{input::Input, task_io_settings::TaskIoSettings};
use algo_lib::misc::gen_vector::gen_vec;
#[allow(unused)]
use algo_lib::{dbg, out, out_line};

type Point = PointT<i64>;

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct Opt {
    d2: i64,
    to: usize,
}

fn solve(input: &mut Input, test_case: usize) {
    let n = input.usize();
    let children = gen_vec(n, |_| Point::new(input.read(), input.read()));
    let bad = Point::new(input.read(), input.read());
    let sweets = gen_vec(n, |_| Point::new(input.read(), input.read()));

    let mut flow = MinCostMaxFlow::new(1 + n * 2 + 1);

    let mut edges = Array2D::new(None, n, n);
    let mut options = vec![vec![]; n];

    for i in 0..n {
        flow.add_edge(0, 1 + i, 1, 0);
        flow.add_edge(1 + n + i, 1 + n + n, 1, 0);
        let d_bad = children[i].dist2(&bad);
        for j in 0..n {
            let d2 = children[i].dist2(&sweets[j]);
            if d2 <= d_bad {
                let e = flow.add_edge(1 + i, 1 + n + j, 1, d2);
                edges[i][j] = Some(e);
                options[i].push(Opt { d2, to: j });
            }
        }
        options[i].sort();
    }
    out!(format!("Case #{}: ", test_case));
    let res = flow.find_min_cost_max_flow(0, 1 + n + n);
    if res.flow != n as i64 {
        out_line!("IMPOSSIBLE");
    } else {
        out_line!("POSSIBLE");
        let mut alive_sweet = vec![true; n];
        let mut alive_child = vec![true; n];

        let mut expect_edge = vec![n; n];
        for i in 0..n {
            for j in 0..n {
                if let Some(e) = edges[i][j] {
                    if flow.get_edge_flow(e) != 0 {
                        expect_edge[i] = j;
                    }
                }
            }
            assert_ne!(expect_edge[i], n);
        }

        let mut it = vec![0; n];
        for _ in 0..n {
            let mut found = false;
            for v in 0..n {
                if !alive_child[v] {
                    continue;
                }
                while it[v] != options[v].len() && !alive_sweet[options[v][it[v]].to] {
                    it[v] += 1;
                }
                assert_ne!(it[v], options[v].len());

                let smallest_d2 = options[v][it[v]].d2;
                let need_d2 = children[v].dist2(&sweets[expect_edge[v]]);
                if smallest_d2 == need_d2 {
                    found = true;
                    alive_child[v] = false;
                    assert!(alive_sweet[expect_edge[v]]);
                    alive_sweet[expect_edge[v]] = false;
                    out_line!(v + 1, expect_edge[v] + 2);
                    break;
                }
            }
            assert!(found);
        }
    }
}

pub(crate) fn run(mut input: Input) -> bool {
    let t = input.read();
    for i in 0usize..t {
        solve(&mut input, i + 1);
    }
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
