//{"name":"l","group":"Manual","url":"","interactive":false,"timeLimit":2000,"tests":[{"input":"","output":""}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"l"}}}

use algo_lib::io::output::output;
use algo_lib::io::task_io_settings::TaskIoType;
use algo_lib::io::task_runner::run_task;
use algo_lib::io::{input::Input, task_io_settings::TaskIoSettings};
use algo_lib::math::gcd::gcd;
use algo_lib::misc::min_max::UpdateMinMax;
use algo_lib::misc::rand::Random;
use algo_lib::misc::rec_function::{Callable3, RecursiveFunction3};
use algo_lib::seg_trees::lazy_seg_tree::{LazySegTree, LazySegTreeNodeSpec};
#[allow(unused)]
use algo_lib::{dbg, out, out_line};

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Default)]
struct Gcd {
    gcd: i64,
    removed: i32,
}

#[derive(Clone, Default)]
struct Node {
    ways: Vec<Gcd>,
}

impl Node {
    pub fn new(x: i64) -> Self {
        Node {
            ways: vec![Gcd { gcd: x, removed: 0 }, Gcd { gcd: 0, removed: 1 }],
        }
    }
}

impl LazySegTreeNodeSpec for Node {
    fn unite(l: &Self, r: &Self, _context: &Self::Context) -> Self {
        let mut ways = vec![];
        for i in 0..l.ways.len() {
            for j in 0..r.ways.len() {
                if l.ways[i].removed + r.ways[j].removed <= 2 {
                    ways.push(Gcd {
                        gcd: gcd(l.ways[i].gcd, r.ways[j].gcd),
                        removed: l.ways[i].removed + r.ways[j].removed,
                    });
                }
            }
        }
        ways.sort();
        let mut nways: Vec<Gcd> = vec![];
        for w in ways.into_iter() {
            if nways.is_empty() || nways.last().unwrap().gcd != w.gcd {
                nways.push(w);
            }
        }
        Node { ways: nways }
    }

    fn apply_update(node: &mut Self, update: &Self::Update) {
        todo!()
    }

    fn join_updates(current: &mut Self::Update, add: &Self::Update) {
        todo!()
    }

    type Update = ();

    type Context = ();
}

#[derive(Clone, Debug)]
struct Query {
    l: usize,
    r: usize,
    remove: i32,
    id: usize,
}

fn solve_case(a: &[i64], queries: &[Query]) -> Vec<i64> {
    let n = a.len();
    let q = queries.len();
    let mut results = vec![0; q];

    let solve_3 = |query: &Query| {
        let a = &a[query.l..query.r];
        let mut dps = vec![Node {
            ways: vec![Gcd { gcd: 0, removed: 0 }],
        }];
        for i in 0..a.len() {
            let cur = Node::new(a[i]);
            let next = Node::unite(&cur, &dps[i], &());
            dps.push(next);
        }
        let mut suf_gcd = vec![0; a.len() + 1];
        for i in (0..a.len()).rev() {
            suf_gcd[i] = gcd(a[i], suf_gcd[i + 1]);
        }
        let mut res = 0;
        for rem_pos in 0..a.len() {
            // TODO: check index
            for w in dps[rem_pos].ways.iter() {
                let suf_gcd = suf_gcd[rem_pos + 1];
                let cur_res = gcd(suf_gcd, w.gcd);
                res.update_max(cur_res);
            }
        }
        res
    };

    RecursiveFunction3::new(|f, l: usize, r: usize, queries: Vec<Query>| {
        if r - l <= 1 {
            return;
        }
        let mid = (l + r) >> 1;

        let mut dp_right = vec![Node {
            ways: vec![Gcd { gcd: 0, removed: 0 }],
        }];
        for i in mid..r {
            let cur = Node::new(a[i]);
            let next = Node::unite(&cur, &dp_right[i - mid], &());
            dp_right.push(next);
        }
        let mut dp_left = vec![Node {
            ways: vec![Gcd { gcd: 0, removed: 0 }],
        }];
        for i in (l..mid).rev() {
            let cur = Node::new(a[i]);
            // TODO: check index
            let next = Node::unite(&cur, &dp_left[mid - i - 1], &());
            dp_left.push(next);
        }

        let mut queries_left = vec![];
        let mut queries_right = vec![];
        for query in queries.iter() {
            if query.r <= mid {
                queries_left.push(query.clone());
            } else if query.l >= mid {
                queries_right.push(query.clone());
            } else {
                let r = if query.remove == 3 {
                    solve_3(query)
                } else {
                    let node_left = &dp_left[mid - query.l];
                    // TODO: check index
                    let node_right = &dp_right[query.r - mid];
                    let node = Node::unite(node_left, node_right, &());
                    let mut res = 0;
                    for w in node.ways.iter() {
                        if w.removed <= query.remove {
                            res.update_max(w.gcd);
                        }
                    }
                    res
                };
                results[query.id] = r;
            }
        }
        f.call(l, mid, queries_left);
        f.call(mid, r, queries_right);
    })
    .call(0, n, queries.to_owned());
    results
}

fn solve_case_st(a: &[i64], queries: &[Query]) -> Vec<i64> {
    let n = a.len();
    let mut st = LazySegTree::new_f(n, &|i| Node {
        ways: vec![
            Gcd {
                gcd: a[i],
                removed: 0,
            },
            Gcd { gcd: 0, removed: 1 },
        ],
    });
    let mut results = vec![0; queries.len()];
    for i in 0..queries.len() {
        let q = &queries[i];
        let node = st.get(q.l..q.r);
        let mut res = 0;
        for w in node.ways.iter() {
            if w.removed <= q.remove {
                res.update_max(w.gcd);
            }
        }
        if q.remove == 3 {
            for mid in q.l + 1..q.r - 1 {
                let node_left = st.get(q.l..mid);
                let node_right = st.get(mid + 1..q.r);
                let node = Node::unite(&node_left, &node_right, &());
                for w in node.ways.iter() {
                    res.update_max(w.gcd);
                }
            }
        }
        results[i] = res;
    }
    results
}

fn stress() {
    const MAX_N: usize = 30;
    const MAX_V: i64 = 100000000;
    const MAX_Q: usize = 100;
    for it in 1.. {
        dbg!(it);
        let mut rnd = Random::new(787788);
        let n = rnd.gen(1..MAX_N);
        let mut a = rnd.gen_vec(n, 1..MAX_V);
        for i in 0..a.len() {
            if rnd.gen_double() < 0.99 {
                a[i] = 2 * 3 * 5 * 7 * 11 * 13 * 17 * 19;
            }
        }
        let q = rnd.gen(1..MAX_Q);
        let mut queries = vec![];
        for _id in 0..q {
            let range = rnd.gen_nonempty_range(n);
            let k = rnd.gen(1..4);
            if range.len() > k {
                let id = queries.len();
                queries.push(Query {
                    l: range.start,
                    r: range.end,
                    remove: k as i32,
                    id,
                });
            }
        }
        let my = solve_case(&a, &queries);
        let st = solve_case_st(&a, &queries);
        if my != st {
            dbg!(a);
            dbg!(queries);
            dbg!(my);
            dbg!(st);
            assert!(false);
        }
    }
}

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.usize();
    let q = input.usize();
    let a = input.vec::<i64>(n);

    let mut queries = vec![];
    for id in 0..q {
        let l = input.usize() - 1;
        let r = input.usize();
        let remove = input.i32();

        queries.push(Query { l, r, remove, id });
    }

    let results = solve_case(&a, &queries);

    for &x in results.iter() {
        assert_ne!(x, 0);
        out_line!(x);
    }
    // out_line!(results);
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

#[test]
fn test() {
    let mut a: Vec<_> = (3..=1000_000).step_by(3).collect();
    let mut rnd = Random::new(787788);
    rnd.shuffle(&mut a);
    let mut cnt = 0;
    for &x in &a[..1000] {
        if x % 2 == 0 {
            cnt += 1;
        }
    }
    dbg!(cnt, cnt * a.len() / 1000);
}

fn main() {
    tester::run_tests();
    // tester::run_single_test("1");
    // tester::run_stress(stress);
    // tester::run_locally();
    // tester::run_with_last_downloaded_file();
}
//END MAIN
