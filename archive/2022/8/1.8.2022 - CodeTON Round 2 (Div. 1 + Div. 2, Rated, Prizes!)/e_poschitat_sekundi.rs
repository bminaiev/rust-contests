//{"name":"E. Посчитать секунды","group":"Codeforces - CodeTON Round 2 (Div. 1 + Div. 2, Rated, Prizes!)","url":"https://codeforces.com/contest/1704/problem/E","interactive":false,"timeLimit":1000,"tests":[{"input":"5\n3 2\n1 1 1\n1 2\n2 3\n5 5\n1 0 0 0 0\n1 2\n2 3\n3 4\n4 5\n1 5\n10 11\n998244353 0 0 0 998244353 0 0 0 0 0\n1 2\n2 3\n3 4\n4 5\n5 6\n6 7\n7 8\n8 9\n9 10\n1 3\n7 9\n5 6\n1293 1145 9961 9961 1919\n1 2\n2 3\n3 4\n5 4\n1 4\n2 4\n6 9\n10 10 10 10 10 10\n1 2\n1 3\n2 3\n4 3\n6 3\n3 5\n6 5\n6 1\n6 2\n","output":"3\n5\n4\n28010\n110\n"}],"testType":"multiNumber","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"EPoschitatSekundi"}}}

use algo_lib::collections::last_exn::LastExn;
use algo_lib::graph::edges::edge_trait::EdgeTrait;
use algo_lib::graph::graph_readers::config::{Directional, Indexation};
use algo_lib::graph::graph_readers::simple::read_graph;
use algo_lib::graph::graph_trait::GraphTrait;
use algo_lib::io::output::output;
use algo_lib::io::task_io_settings::TaskIoType;
use algo_lib::io::task_runner::run_task;
use algo_lib::io::{input::Input, task_io_settings::TaskIoSettings};
use algo_lib::math::modulo::Mod_998_244_353;
use algo_lib::misc::min_max::UpdateMinMax;
#[allow(unused)]
use algo_lib::{dbg, out, out_line};

type Mod = Mod_998_244_353;

#[derive(Clone, Copy)]
struct Segment {
    start: i64,
    len_mod: Mod,
    len: i64,
}

const M: i64 = std::i64::MAX / 10;

fn merge(a: &[Segment], b: &[Segment]) -> Vec<Segment> {
    let mut res: Vec<Segment> = vec![];
    let mut it1 = 0;
    let mut it2 = 0;
    while it1 < a.len() || it2 < b.len() {
        let cur_seg = if it2 == b.len() || (it1 != a.len() && a[it1].start < b[it2].start) {
            it1 += 1;
            a[it1 - 1]
        } else {
            it2 += 1;
            b[it2 - 1]
        };
        if !res.is_empty() && res.last_exn().start + res.last_exn().len >= cur_seg.start {
            let lp = res.len() - 1;
            res[lp].len += cur_seg.len;
            res[lp].len.update_min(M);
            res[lp].len_mod += cur_seg.len_mod;
        } else {
            res.push(cur_seg);
        }
    }
    res
}

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.usize();
    let m = input.usize();
    let a = input.vec::<i64>(n);
    let g = read_graph(input, n, m, Directional::Directed, Indexation::FromOne);
    let mut inside = vec![0; n];
    for v in 0..n {
        for e in g.adj(v) {
            inside[e.to()] += 1;
        }
    }
    let mut queue = vec![];
    for v in 0..n {
        if inside[v] == 0 {
            queue.push(v);
        }
    }
    let mut it = 0;
    while it != queue.len() {
        let v = queue[it];
        it += 1;
        for e in g.adj(v) {
            inside[e.to()] -= 1;
            if inside[e.to()] == 0 {
                queue.push(e.to());
            }
        }
    }
    let last = *queue.last_exn();
    assert!(g.adj(last).is_empty());

    let mut segs = vec![vec![]; n];
    for v in 0..n {
        if a[v] == 0 {
            continue;
        }
        segs[v].push(Segment {
            start: 0,
            len_mod: Mod::new(a[v]),
            len: a[v],
        });
    }
    for &v in queue.iter() {
        let cur_segs: Vec<_> = segs[v]
            .iter()
            .map(|s| Segment {
                start: s.start + 1,
                len_mod: s.len_mod,
                len: s.len,
            })
            .collect();
        for e in g.adj(v) {
            segs[e.to()] = merge(&segs[e.to()], &cur_segs);
        }
    }
    if segs[last].is_empty() {
        out_line!(0);
    } else {
        let seg = segs[last].last_exn();
        let time = Mod::new(seg.start) + seg.len_mod;
        out_line!(time);
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
    // tester::run_stress(stress);
}
//END MAIN
