//{"name":"a","group":"Manual","url":"","interactive":false,"timeLimit":2000,"tests":[{"input":"","output":""},{"input":"","output":""},{"input":"","output":""}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"a"}}}

use algo_lib::collections::min_priority_queue::MinPriorityQueue;
use algo_lib::collections::resettable_array::ResettableArray;
use algo_lib::graph::edges::edge_trait::EdgeTrait;
use algo_lib::graph::edges::weighted_edge::WeightedEdge;
use algo_lib::io::output::output;
use algo_lib::io::task_io_settings::TaskIoType;
use algo_lib::io::task_runner::run_task;
use algo_lib::io::{input::Input, task_io_settings::TaskIoSettings};
use algo_lib::misc::gen_vector::gen_vec;
use algo_lib::misc::min_max::UpdateMinMax;
use algo_lib::misc::rand::Random;
use algo_lib::misc::rec_function::{Callable, RecursiveFunction};
#[allow(unused)]
use algo_lib::{dbg, out, out_line};

type Edge = WeightedEdge<i64>;

fn solve_case(prev: &[usize], cost: &[i64], w: &[i64], mut queries: Vec<Query>) -> Vec<i64> {
    let n = prev.len();
    let mut base_g = vec![vec![]; n];
    for i in 1..n {
        base_g[prev[i]].push(Edge::new(i, cost[i]));
    }
    let mut leafs = vec![];
    for v in 0..n {
        if base_g[v].is_empty() {
            leafs.push(v);
        }
    }
    let mut g = vec![];
    let mut new_id = vec![std::usize::MAX; n];
    for v in (0..n).rev() {
        let mut cur = g.len();
        g.push(vec![]);
        if !base_g[v].is_empty() {
            let e = base_g[v][0];
            g[cur].push(Edge::new(new_id[e.to()], e.cost));
            g[new_id[e.to()]].push(Edge::new(cur, e.cost));
        }
        for i in 1..base_g[v].len() {
            let next = g.len();
            g.push(vec![]);
            g[cur].push(Edge::new(next, 0));
            g[next].push(Edge::new(cur, 0));
            cur = next;
            {
                let e = base_g[v][i];
                g[cur].push(Edge::new(new_id[e.to()], e.cost));
                g[new_id[e.to()]].push(Edge::new(cur, e.cost));
            }
        }
        new_id[v] = cur;
    }
    for v in 0..g.len() {
        assert!(g[v].len() <= 3);
    }

    let n = g.len();

    let mut g_circle = vec![vec![]; g.len()];
    for i in 0..leafs.len() {
        let x = new_id[leafs[i]];
        let y = new_id[leafs[(i + 1) % leafs.len()]];
        g_circle[x].push(Edge::new(y, w[i]));
        g_circle[y].push(Edge::new(x, w[i]));
    }
    for q in queries.iter_mut() {
        q.x = new_id[q.x];
        q.y = new_id[q.y];
    }

    let mut queies_by_v = vec![vec![]; g.len()];
    for &query in queries.iter() {
        queies_by_v[query.x].push(query);
        queies_by_v[query.y].push(query);
    }

    let mut res = vec![std::i64::MAX; queries.len()];

    let mut alive = vec![true; n];
    let mut sz = vec![0; n];

    let traverse =
        |v: usize, alive: &[bool], pos_in_queue: &mut ResettableArray<usize>| -> Vec<usize> {
            pos_in_queue.reset();
            let mut queue = vec![v];
            pos_in_queue.set(v, 0);
            let mut it = 0;
            while it < queue.len() {
                let v = queue[it];
                it += 1;
                for e in g[v].iter() {
                    if pos_in_queue.get(e.to()) == n && alive[e.to()] {
                        pos_in_queue.set(e.to(), queue.len());
                        queue.push(e.to());
                    }
                }
            }
            queue
        };

    let mut pos_in_queue = ResettableArray::new(n, n);

    let mut colors = ResettableArray::new(n + 10, n);

    let mut dijkstra_dist = ResettableArray::new(std::i64::MAX / 3, n);

    let mut total_dij = 0;
    let mut seen_times = vec![0; n];

    RecursiveFunction::new(|f, v: usize| {
        let queue = traverse(v, &alive, &mut pos_in_queue);
        for pos in (0..queue.len()).rev() {
            let v = queue[pos];
            sz[v] = 1;
            for e in g[v].iter() {
                let to_pos = pos_in_queue.get(e.to());
                if to_pos != n && to_pos > pos {
                    sz[v] += sz[e.to()];
                }
            }
        }
        let need_size = (sz[v] + 1) / 2;
        let mut root = v;
        let mut prev_root = v;
        loop {
            let mut changed = false;
            for e in g[root].iter() {
                if alive[e.to()] && e.to() != prev_root && sz[e.to()] >= need_size {
                    prev_root = root;
                    root = e.to();
                    changed = true;
                    break;
                }
            }
            if !changed {
                break;
            }
        }

        alive[root] = false;

        {
            let mut subtrees = vec![];
            for e in g[root].iter() {
                if alive[e.to()] {
                    subtrees.push(traverse(e.to(), &alive, &mut pos_in_queue));
                }
            }
            alive[root] = true;
            colors.reset();

            let mut interesting_points = vec![root];
            for color in 0..subtrees.len() {
                for &v in subtrees[color].iter() {
                    colors.set(v, color);
                    seen_times[v] += 1;
                }
            }

            for color in 0..subtrees.len() {
                for &v in subtrees[color].iter() {
                    for e2 in g_circle[v].iter() {
                        if colors.get(e2.to()) != color && colors.get(e2.to()) <= 2 {
                            interesting_points.push(v);
                        }
                    }
                }
            }
            interesting_points.sort();
            interesting_points.dedup();
            // dbg!(interesting_points);
            assert!(interesting_points.len() <= 10);

            for &pt in interesting_points.iter() {
                dijkstra_dist.reset();
                dijkstra_dist.set(pt, 0);
                let mut queue = MinPriorityQueue::new();
                queue.push(V { dist: 0, v: pt });
                while let Some(v) = queue.pop() {
                    total_dij += 1;
                    let cur_dist = dijkstra_dist.get(v.v);
                    for e in g[v.v].iter().chain(g_circle[v.v].iter()) {
                        let ncost = cur_dist + e.cost;
                        if alive[e.to()]
                            && dijkstra_dist.get(e.to()) > ncost
                            && (colors.get(e.to()) <= 2 || e.to() == root)
                        {
                            dijkstra_dist.set(e.to(), ncost);
                            queue.push(V {
                                dist: ncost,
                                v: e.to(),
                            });
                        }
                    }

                    // seen_times[v.v] += 1;
                    for query in queies_by_v[v.v].iter() {
                        let d1 = dijkstra_dist.get(query.x);
                        let d2 = dijkstra_dist.get(query.y);
                        res[query.id].update_min(d1 + d2);
                    }
                }
            }

            alive[root] = false;
        }

        for e in g[root].iter() {
            if alive[e.to()] {
                f.call(e.to());
            }
        }
    })
    .call(0);

    // dbg!(seen_times);
    // for i in 0..seen_times.len() {
    //     assert!(seen_times[i] <= 300);
    // }
    // dbg!(total_dij);

    res
}

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.usize();
    let mut prev = vec![0; n];
    let mut cost = vec![0; n];

    let mut is_leaf = vec![true; n];
    for i in 1..n {
        prev[i] = input.usize() - 1;
        cost[i] = input.read();
        is_leaf[prev[i]] = false;
    }

    let cnt_leafs = (0..n).filter(|&id| is_leaf[id]).count();

    let w = input.vec::<i64>(cnt_leafs);

    let queries = gen_vec(input.usize(), |id| Query {
        x: input.usize() - 1,
        y: input.usize() - 1,
        id,
    });

    let res = solve_case(&prev, &cost, &w, queries);

    for &x in res.iter() {
        out_line!(x);
    }
}

fn preorder(prev: &[usize]) -> Vec<usize> {
    let n = prev.len();
    let mut g = vec![vec![]; n];
    for i in 1..n {
        g[prev[i]].push(i);
    }
    let mut queue = vec![];

    RecursiveFunction::new(|f, v: usize| {
        queue.push(v);
        for &to in g[v].iter() {
            f.call(to);
        }
    })
    .call(0);

    let mut new_id = vec![0; n];
    for i in 0..n {
        new_id[queue[i]] = i;
    }
    let mut new_prev = vec![0; n];
    for v in 1..n {
        new_prev[new_id[v]] = new_id[prev[v]];
    }
    new_prev
}

fn stress() {
    let n = 100_000;
    let mut prev = vec![0; n];
    let mut cost = vec![0; n];
    let mut rnd = Random::new(787788);
    const MAX_COST: i64 = 1e8 as i64;
    let mut is_leaf = vec![true; n];
    for i in 1..n {
        prev[i] = rnd.gen(0..i);
        cost[i] = rnd.gen(0..MAX_COST);
        // is_leaf[prev[i]] = false;
    }
    let prev = preorder(&prev);
    for i in 1..n {
        is_leaf[prev[i]] = false;
    }
    let cnt_leafs = (0..n).filter(|&id| is_leaf[id]).count();

    let w = gen_vec(cnt_leafs, |_| rnd.gen(0..MAX_COST));
    let queries = gen_vec(250_000, |id| Query {
        x: rnd.gen(0..n),
        y: rnd.gen(0..n),
        id,
    });
    solve_case(&prev, &cost, &w, queries);
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct V {
    dist: i64,
    v: usize,
}

#[derive(Clone, Copy)]
struct Query {
    x: usize,
    y: usize,
    id: usize,
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
    // tester::run_single_test("3");
    // tester::run_stress(stress);
    // tester::run_locally();
    // tester::run_with_last_downloaded_file();
}
//END MAIN
