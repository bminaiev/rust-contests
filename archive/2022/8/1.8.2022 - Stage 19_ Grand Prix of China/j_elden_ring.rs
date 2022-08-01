//{"name":"J. Elden Ring","group":"Yandex - Stage 19: Grand Prix of China","url":"https://official.contest.yandex.com/opencupXXII/contest/39025/problems/J/","interactive":false,"timeLimit":3000,"tests":[{"input":"2\n5 4 5 8\n1 2\n1 3\n1 4\n4 5\n15 1 1 1 1\n5 4 10 5\n1 2\n1 3\n1 4\n4 5\n10 4 4 4 19\n","output":"2\n4\n"}],"testType":"multiNumber","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"JEldenRing"}}}

use std::cmp::max;

use algo_lib::collections::min_priority_queue::MinPriorityQueue;
use algo_lib::graph::edges::edge_trait::EdgeTrait;
use algo_lib::graph::graph_readers::config::{Directional, Indexation};
use algo_lib::graph::graph_readers::simple::read_graph;
use algo_lib::graph::graph_trait::GraphTrait;
use algo_lib::io::output::output;
use algo_lib::io::task_io_settings::TaskIoType;
use algo_lib::io::task_runner::run_task;
use algo_lib::io::{input::Input, task_io_settings::TaskIoSettings};
#[allow(unused)]
use algo_lib::{dbg, out, out_line};

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct Vertex {
    time: i64,
    v: usize,
}

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.usize();
    let m = input.usize();
    let add_me = input.i64();
    let add_monster = input.i64();
    let g = read_graph(input, n, m, Directional::Undirected, Indexation::FromOne);
    let mut start_levels = input.vec::<i64>(n);
    for i in 1..n {
        start_levels[i] += add_monster;
    }
    let res = if add_me <= add_monster {
        let mut min_time = vec![std::i64::MAX; n];
        min_time[0] = 0;
        let mut pq = MinPriorityQueue::new();
        pq.push(Vertex { time: 0, v: 0 });
        let mut seen = vec![false; n];
        while let Some(vertex) = pq.pop() {
            let v = vertex.v;
            if seen[v] {
                continue;
            }
            seen[v] = true;
            let cur_time = min_time[v];
            let next_time = cur_time + 1;
            for e in g.adj(v) {
                let to = e.to();
                let my = start_levels[0] + cur_time * add_me;
                let his = start_levels[to] + cur_time * add_monster;
                if my > his && min_time[to] > next_time {
                    min_time[to] = next_time;
                    pq.push(Vertex {
                        time: next_time,
                        v: to,
                    });
                }
            }
        }
        if seen[n - 1] {
            min_time[n - 1]
        } else {
            -1
        }
    } else {
        let mut min_time = vec![std::i64::MAX; n];
        min_time[0] = 0;
        let mut pq = MinPriorityQueue::new();
        pq.push(Vertex { time: 0, v: 0 });
        let mut seen = vec![false; n];
        let mut cur_time = 0;
        while let Some(vertex) = pq.pop() {
            let v = vertex.v;
            if seen[v] {
                continue;
            }
            if vertex.time > cur_time {
                break;
            }
            cur_time += 1;
            seen[v] = true;
            for e in g.adj(v) {
                let to = e.to();
                let need_at_least = start_levels[to] - start_levels[0] + 1;
                let first_ok_time = if need_at_least <= 0 {
                    0
                } else {
                    let add_each_day = add_me - add_monster;
                    (need_at_least + add_each_day - 1) / add_each_day
                };
                // dbg!(to, first_ok_time, need_at_least);
                let first_ok_time = max(first_ok_time + 1, vertex.time + 1);
                if min_time[to] > first_ok_time {
                    min_time[to] = first_ok_time;
                    pq.push(Vertex {
                        time: first_ok_time,
                        v: to,
                    });
                }
            }
        }
        if seen[n - 1] {
            min_time[n - 1]
        } else {
            -1
        }
    };
    out_line!(res);
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
    // tester::run_stress(stress);
}
//END MAIN
