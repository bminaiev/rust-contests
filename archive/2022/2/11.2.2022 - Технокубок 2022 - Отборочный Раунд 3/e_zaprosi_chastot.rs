//{"name":"E. Запросы частот","group":"Codeforces - Технокубок 2022 - Отборочный Раунд 3","url":"http://codeforces.com/contest/1585/problem/E","interactive":false,"timeLimit":4000,"tests":[{"input":"2\n3 3\n1 1 1\n1 2\n3 1 1\n3 1 2\n3 2 1\n5 5\n1 2 1 1 2\n1 1 2 2\n3 1 1\n2 1 2\n4 1 1\n4 2 1\n4 2 2\n","output":"1 -1 1\n1 1 2 1 -1\n"}],"testType":"multiNumber","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"EZaprosiChastot"}}}

use std::collections::BTreeSet;

use algo_lib::graph::edges::edge_trait::EdgeTrait;
use algo_lib::graph::graph_trait::GraphTrait;
use algo_lib::io::output::output;
use algo_lib::misc::binary_search::binary_search_last_true;
use algo_lib::misc::rec_function::{Callable2, RecursiveFunction2};
#[allow(unused)]
use algo_lib::{dbg, out, out_line};
use algo_lib::{graph::simple_graph::SimpleGraphT, seg_trees::fenwick::Fenwick};
use algo_lib::{io::input::Input, misc::vec_apply_delta::ApplyDelta};

#[derive(Clone)]
struct Query {
    id: usize,
    at_least: usize,
    kth: usize,
}

struct DS {
    seen_number: Vec<i32>,
    count_sizes: Fenwick,
    examples: Vec<BTreeSet<usize>>,
}

impl DS {
    pub fn new(n: usize) -> Self {
        Self {
            seen_number: vec![0; n],
            count_sizes: Fenwick::new(n + 1),
            examples: vec![BTreeSet::new(); n + 1],
        }
    }

    pub fn change_num(&mut self, number: usize, delta: i32) {
        self.examples[self.seen_number[number] as usize].remove(&number);
        self.count_sizes.add(self.seen_number[number] as usize, -1);
        self.seen_number[number] += delta;
        self.count_sizes.add(self.seen_number[number] as usize, 1);
        self.examples[self.seen_number[number] as usize].insert(number);
    }

    pub fn get_random(&self, pos: usize) -> usize {
        *self.examples[pos].iter().next().unwrap()
    }
}

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.usize();
    let q = input.usize();
    let values = input.read_vec::<usize>(n).sub_from_all(1);
    let mut graph = SimpleGraphT::new(n);
    for v in 1..n {
        let p = input.usize() - 1;
        graph.add_bi_edge(v, p);
    }
    let mut queries = vec![vec![]; n];
    for id in 0..q {
        let v = input.usize() - 1;
        let at_least = input.usize();
        let kth = input.usize() - 1;
        queries[v].push(Query { id, at_least, kth });
    }
    let mut ds = DS::new(n);
    let mut res = vec![0i32; q];
    RecursiveFunction2::new(|f, v: usize, p| {
        ds.change_num(values[v], 1);
        for query in queries[v].iter() {
            let cnt_at_least = ds.count_sizes.get_suffix_sum(query.at_least) as usize;
            res[query.id] = if cnt_at_least > query.kth {
                let from_end = cnt_at_least - query.kth;
                let pos = binary_search_last_true(0..n + 1, |check| {
                    ds.count_sizes.get_suffix_sum(check) >= from_end as i64
                })
                .unwrap();
                (ds.get_random(pos) + 1) as i32
            } else {
                -1
            };
        }
        for edge in graph.adj(v) {
            let to = edge.to();
            if to == p {
                continue;
            }
            f.call(to, v);
        }
        ds.change_num(values[v], -1);
    })
    .call(0, 0);
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

//START MAIN
mod tester;

fn main() {
    tester::run_tests();
    // tester::run_single_test("1");
}
//END MAIN
