//{"name":"E - Pairing Wizards","group":"AtCoder - AtCoder Regular Contest 142","url":"https://atcoder.jp/contests/arc142/tasks/arc142_e","interactive":false,"timeLimit":2000,"tests":[{"input":"5\n1 5\n2 4\n3 3\n4 2\n5 1\n3\n1 4\n2 5\n3 5\n","output":"2\n"},{"input":"4\n1 1\n1 1\n1 1\n1 1\n3\n1 2\n2 3\n3 4\n","output":"0\n"},{"input":"9\n1 1\n2 4\n5 5\n7 10\n9 3\n9 13\n10 9\n3 9\n2 9\n7\n1 5\n2 5\n1 6\n2 4\n3 4\n4 9\n8 9\n","output":"22\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"EPairingWizards"}}}

use std::cmp::{max, min};
use std::time::Instant;

use algo_lib::graph::compressed_graph::CompressedGraph;
use algo_lib::graph::edges::edge_trait::EdgeTrait;
use algo_lib::graph::edges::simple_edge::SimpleEdge;
use algo_lib::graph::edges::weighted_edge::WeightedEdge;
use algo_lib::graph::graph_builder::GraphBuilder;
use algo_lib::graph::graph_readers::config::{Directional, Indexation};
use algo_lib::graph::graph_readers::simple::read_graph;
use algo_lib::graph::graph_trait::GraphTrait;
use algo_lib::io::output::output;
use algo_lib::io::task_io_settings::TaskIoType;
use algo_lib::io::task_runner::run_task;
use algo_lib::io::{input::Input, task_io_settings::TaskIoSettings};
use algo_lib::misc::gen_vector::gen_vec;
use algo_lib::misc::min_max::UpdateMinMax;
use algo_lib::misc::rand::Random;
#[allow(unused)]
use algo_lib::{dbg, out, out_line};

// Just a multiset of integers, with fast [max] and [second_max]
#[derive(Clone, Debug)]
struct Requirements {
    cnt: Vec<usize>,
    mask: u128,
    max_iter: usize,
}

impl Requirements {
    const MASK_MAX: usize = 110;

    pub fn new(max_val: usize) -> Self {
        Self {
            cnt: vec![0; max_val + 1],
            mask: 0,
            max_iter: 0,
        }
    }

    pub fn add(&mut self, pos: usize) {
        self.cnt[pos] += 1;
        self.max_iter.update_max(pos);
        self.mask |= 1u128 << (Self::MASK_MAX - pos);
    }

    pub fn remove(&mut self, pos: usize) {
        self.cnt[pos] -= 1;
        if self.cnt[pos] == 0 {
            self.mask ^= 1u128 << (Self::MASK_MAX - pos);
            self.max_iter = Self::MASK_MAX - self.mask.trailing_zeros() as usize;
        }
    }

    pub fn second_max(&self) -> usize {
        if self.cnt[self.max_iter] == 1 {
            let new_mask = self.mask ^ (1u128 << (Self::MASK_MAX - self.max_iter));
            Self::MASK_MAX - new_mask.trailing_zeros() as usize
        } else {
            self.max_iter
        }
    }
}

// Some heuristic to get probably better initial permutation than just totally random
fn gen_initial_ranks(rnd: &mut Random, start: &[usize], need: &[usize]) -> Vec<usize> {
    let n = start.len();

    let mx = rnd.gen(10..200i32);
    let a = rnd.gen(-mx..mx);
    let b = rnd.gen(-mx..mx);
    let mut scores = gen_vec(n, |pos| {
        (
            pos,
            start[pos] as i32 * a + need[pos] as i32 * b + rnd.gen(0..mx * mx),
        )
    });
    scores.sort_by_key(|(_, y)| *y);
    let mut ranks = vec![0; n];
    for i in 0..n {
        ranks[scores[i].0] = i;
    }
    ranks
}

fn remove_rank(ranks: &mut [usize], rank: usize) {
    for x in ranks.iter_mut() {
        if *x > rank {
            *x -= 1;
        }
    }
}

fn insert_rank(
    ranks: &mut [usize],
    elem: usize,
    new_rank: usize,
    g: &CompressedGraph<WeightedEdge<usize>>,
    requirements: &mut [Requirements],
) {
    let old_rank = ranks[elem];
    for x in ranks.iter_mut() {
        if *x >= new_rank {
            *x += 1;
        }
    }
    ranks[elem] = new_rank;

    let (from_rank, to_rank) = if old_rank < new_rank {
        (old_rank, new_rank)
    } else {
        (new_rank + 1, old_rank + 1)
    };

    for e in g.adj(elem) {
        if ranks[e.to()] < from_rank || ranks[e.to()] >= to_rank {
            continue;
        }
        if ranks[elem] < ranks[e.to()] {
            requirements[e.to()].remove(e.cost);
            requirements[elem].add(e.cost);
        } else {
            requirements[elem].remove(e.cost);
            requirements[e.to()].add(e.cost);
        }
    }
}

fn push_suffix_max(a: &mut [usize]) {
    let mut mx = 0;
    for x in a.iter_mut().rev() {
        mx.update_max(*x);
        *x = mx;
    }
}


fn push_prefix_sum(a: &mut [usize]) {
    let mut add = 0;
    for x in a.iter_mut() {
        add += *x;
        *x = add;
    }
}

fn run_local_optimizations(
    start: &[usize],
    need: &[usize],
    rnd: &mut Random,
    g: &CompressedGraph<WeightedEdge<usize>>,
) -> usize {
    let n = need.len();

    let max_val = *need.iter().chain(start.iter()).max().unwrap();
    let mut requirements = vec![Requirements::new(max_val); n];
    for v in 0..n {
        for _ in 0..n + 1 {
            requirements[v].add(start[v]);
        }
    }
    // we maintain property:
    // if rank[i] < rank[j], and we have edge i-j, than:
    // value[i] >= max(need[i], need[j])
    // value[j] >= min(need[i], need[j])
    let mut ranks = gen_initial_ranks(rnd, &start, &need);
    for (fr, e) in g.all_edges() {
        if ranks[fr] < ranks[e.to()] {
            requirements[fr].add(e.cost);
        }
    }

    let calc_score =
        |requirements: &[Requirements]| -> usize { requirements.iter().map(|r| r.max_iter).sum() };

    let mut best_res = calc_score(&requirements);
    loop {
        let mut found_improvement = false;
        for elem in rnd.gen_permutation(n).into_iter() {
            // if we set rank[elem] = x (and move others),
            // we will need to set value[elem] to value_should_be_at_least[x], and
            // increase other values in total by additional_cost[x]
            let mut value_should_be_at_least = vec![start[elem]; n];
            let mut additional_cost = vec![0; n];

            let old_rank = ranks[elem];
            remove_rank(&mut ranks, old_rank);

            for e in g.adj(elem) {
                let req_without = if ranks[elem] <= ranks[e.to()] {
                    requirements[e.to()].max_iter
                } else {
                    requirements[e.to()].second_max()
                };
                additional_cost[ranks[e.to()] + 1] = if e.cost > req_without {
                    e.cost - req_without
                } else {
                    0
                };
            }
            for e in g.adj(elem) {
                value_should_be_at_least[ranks[e.to()]] = e.cost;
            }
            push_suffix_max(&mut value_should_be_at_least);
            push_prefix_sum(&mut additional_cost);

            let best_new_rank = value_should_be_at_least
                .iter()
                .zip(additional_cost.iter())
                .map(|(x, y)| x + y)
                .enumerate()
                .min_by_key(|(_, snd)| *snd)
                .unwrap()
                .0;

            insert_rank(&mut ranks, elem, best_new_rank, g, &mut requirements);
            let new_res = calc_score(&requirements);
            assert!(new_res <= best_res);
            if best_res > new_res {
                found_improvement = true;
            }
            best_res = new_res;
        }
        if !found_improvement {
            break;
        }
    }
    best_res
}

fn solve_case(start: &[usize], need: &[usize], g: &CompressedGraph<SimpleEdge>) -> usize {
    let mut start = start.to_vec();
    let n = start.len();
    let sum_start = start.iter().sum::<usize>();
    for v in 0..n {
        for e in g.adj(v) {
            start[v].update_max(min(need[v], need[e.to()]));
        }
    }
    let mut g_weighted = GraphBuilder::new(n);
    for (fr, e) in g.all_edges() {
        g_weighted.add_edge(fr, WeightedEdge::new(e.to(), max(need[fr], need[e.to()])));
    }
    let g_weighted = g_weighted.build();

    let mut rnd = Random::new(787788);
    let mut best_res = std::usize::MAX;
    let start_time = Instant::now();
    while start_time.elapsed().as_millis() < 500 {
        best_res.update_min(run_local_optimizations(
            &start,
            &need,
            &mut rnd,
            &g_weighted,
        ));
    }
    best_res - sum_start
}

fn solve(input: &mut Input) {
    let n = input.usize();
    let mut start = vec![0; n];
    let mut need = vec![0; n];
    for i in 0..n {
        start[i] = input.usize() - 1;
        need[i] = input.usize() - 1;
    }
    let num_edges = input.usize();
    let g = read_graph(
        input,
        n,
        num_edges,
        Directional::Undirected,
        Indexation::FromOne,
    );
    let res = solve_case(&start, &need, &g);
    out_line!(res);
}

pub(crate) fn run(mut input: Input) -> bool {
    solve(&mut input);
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

fn stress() {
    let mut rnd = Random::new(787788);
    for iter in 1..2 {
        dbg!(iter);
        const MAX: usize = 100;
        let n = MAX;
        let a = rnd.gen_vec(n, 0..MAX);
        let b = rnd.gen_vec(n, 0..MAX);
        let mut graph_builder = GraphBuilder::new(n);
        for v in 0..n {
            for to in v + 1..n {
                graph_builder.add_edge(v, SimpleEdge::new(to));
                graph_builder.add_edge(to, SimpleEdge::new(v));
            }
        }
        solve_case(&a, &b, &graph_builder.build());
    }
}

fn main() {
    tester::run_tests();
    // tester::run_single_teet("1");
    // tester::run_stress(stress);
}
//END MAIN
