//{"name":"B. Bureaucracy","group":"Yandex - SNWS-2022, Round 5","url":"https://contest.yandex.ru/snws2022/contest/23961/problems/B/","interactive":false,"timeLimit":2000,"tests":[{"input":"9 3\n2 1\n4 2\n3 2\n5 3\n9 7\n7 4\n8 6\n6 5\n10 83 6006 6000 1 2 3 5999 1\n3 0.510\n6 0.314\n2 0.112\n","output":"6006.622000\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"BBureaucracy"}}}

use algo_lib::graph::edges::edge_trait::EdgeTrait;
use algo_lib::graph::graph_readers::config::{Directional, Indexation};
use algo_lib::graph::graph_readers::simple::read_graph;
use algo_lib::graph::graph_trait::GraphTrait;
use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::misc::rec_function::{Callable2, RecursiveFunction, RecursiveFunction2};
use algo_lib::{dbg, out, out_line};

struct Ans {
    start: usize,
    probs: Vec<f64>,
}

impl Ans {
    pub fn join(mut lhs: Self, rhs: Self) -> Self {
        if lhs.start < rhs.start {
            return Self::join(rhs, lhs);
        }
        let mut pref_right = 0.0;
        let mut it_right = 0;
        while it_right != rhs.probs.len() && rhs.start + it_right < lhs.start {
            pref_right += rhs.probs[it_right];
            it_right += 1;
        }
        let mut pref_left = 0.0;
        let mut it_left = 0;
        while it_right != rhs.probs.len() {
            if it_left == lhs.probs.len() {
                lhs.probs.push(0.0);
            }
            pref_right += rhs.probs[it_right];
            let new_prob = lhs.probs[it_left] * pref_right + rhs.probs[it_right] * pref_left;
            pref_left += lhs.probs[it_left];
            lhs.probs[it_left] = new_prob;
            it_right += 1;
            it_left += 1;
        }
        lhs
    }

    pub fn shift(&self, p: f64) -> Self {
        let mut probs = vec![0.0; self.probs.len() + 1];
        for i in 0..self.probs.len() {
            probs[i + 1] += self.probs[i] * p;
            probs[i] += self.probs[i] * (1.0 - p);
        }
        Self {
            start: self.start,
            probs,
        }
    }
}

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.usize();
    let q = input.usize();
    let graph = read_graph(
        input,
        n,
        n - 1,
        Directional::Undirected,
        Indexation::FromOne,
    );
    let init = input.read_vec::<usize>(n);
    let mut probs = vec![vec![]; n];
    for _ in 0..q {
        let v = input.usize() - 1;
        let p = input.f64().0;
        probs[v].push(p);
    }
    let ans = RecursiveFunction2::new(|f, v, p| -> Ans {
        let mut ans = Ans {
            start: init[v],
            probs: vec![1.0],
        };
        for e in graph.adj(v) {
            if e.to() == p {
                continue;
            }
            let child = f.call(e.to(), v);
            ans = Ans::join(ans, child);
        }
        for &pr in probs[v].iter() {
            ans = ans.shift(pr);
        }
        ans
    })
    .call(0, 0);
    let mut ev = 0.0;
    for (offset, &p) in ans.probs.iter().enumerate() {
        ev += p * ((ans.start + offset) as f64);
    }
    out_line!(ev);
}

pub(crate) fn run(mut input: Input) -> bool {
    solve(&mut input, 1);
    output().flush();
    input.skip_whitespace();
    input.peek().is_none()
}

//START MAIN
mod tester;

fn main() {
    tester::run_tests();
    // tester::run_single_test("1");
}
//END MAIN
