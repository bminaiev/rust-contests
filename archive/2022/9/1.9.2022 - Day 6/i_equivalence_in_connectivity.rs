//{"name":"I. Equivalence in Connectivity","group":"Yandex - Day 6","url":"https://official.contest.yandex.com/ptz-summer-2022/contest/39551/problems/I/","interactive":false,"timeLimit":2000,"tests":[{"input":"2\n15 11 8\n6 11\n1 6\n6 9\n6 8\n1 2\n1 5\n9 10\n2 5\n1 add 3 11\n1 add 2 3\n3 add 5 8\n4 add 5 11\n3 add 7 10\n1 add 6 10\n3 add 3 10\n1 remove 6 8\n5 add 4 9\n1 add 2 9\n8 add 7 8\n3 add 2 4\n1 remove 6 9\n10 remove 6 9\n14 5 2\n1 5\n1 4\n1 add 2 4\n1 add 3 4\n1 add 2 4\n4 add 3 4\n4 add 1 3\n5 add 1 3\n2 add 2 3\n1 add 1 2\n4 add 3 4\n3 add 4 5\n9 add 2 3\n3 remove 1 5\n3 remove 3 4\n","output":"7\n2 10 13\n5 2 3 4 5 8\n3 1 7 11\n1 14\n2 6 12\n1 9\n1 15\n5\n3 2 4 9\n6 5 6 7 8 10 12\n2 1 14\n2 3 11\n1 13\n"}],"testType":"multiNumber","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"IEquivalenceInConnectivity"}}}

use std::cmp::{max, min};
use std::collections::HashMap;

use algo_lib::io::output::output;
use algo_lib::io::task_io_settings::TaskIoType;
use algo_lib::io::task_runner::run_task;
use algo_lib::io::{input::Input, task_io_settings::TaskIoSettings};
use algo_lib::misc::func::id;
use algo_lib::misc::gen_vector::gen_vec;
use algo_lib::misc::rand::Random;
use algo_lib::misc::rec_function::{Callable, Callable3, RecursiveFunction, RecursiveFunction3};
#[allow(unused)]
use algo_lib::{dbg, out, out_line};

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
struct Edge {
    fr: usize,
    to: usize,
}

impl Edge {
    pub fn new(x: usize, y: usize) -> Self {
        Self {
            fr: min(x, y),
            to: max(x, y),
        }
    }
}

#[derive(Clone, Copy)]
struct Change {
    e: Edge,
    to: usize,
}

#[derive(Clone, Copy, Debug)]
struct Event {
    from: usize,
    to: usize,
    e: Edge,
}

#[derive(Clone, Copy)]
struct DsuEvent {
    idx: usize,
    size: usize,
    p: usize,
    hash: u64,
    sum_sq_hashes: u64,
}

struct DsuWithRollbacks {
    size: Vec<usize>,
    hashes: Vec<u64>,
    p: Vec<usize>,
    events: Vec<DsuEvent>,
    sum_sq_hashes: u64,
}

impl DsuWithRollbacks {
    pub fn new(n: usize) -> Self {
        let mut rnd = Random::new(787788);
        let hashes = gen_vec(n, |_| rnd.gen_u64());
        let hashes_copy = hashes.clone();
        let p = gen_vec(n, id);
        let mut res = Self {
            size: vec![1; n],
            hashes,
            p,
            events: vec![],
            sum_sq_hashes: 0,
        };
        for &h in hashes_copy.iter() {
            res.add_sum_sq_hash(h);
        }
        res
    }

    pub fn get_current_time(&self) -> usize {
        self.events.len()
    }

    pub fn rollback(&mut self, time: usize) {
        while self.events.len() != time {
            let ev = self.events.pop().unwrap();
            self.size[ev.idx] = ev.size;
            self.hashes[ev.idx] = ev.hash;
            self.p[ev.idx] = ev.p;
            self.sum_sq_hashes = ev.sum_sq_hashes;
        }
    }

    pub fn get(&self, mut v: usize) -> usize {
        while self.p[v] != v {
            v = self.p[v];
        }
        return v;
    }

    pub fn save(&mut self, i: usize) {
        self.events.push(DsuEvent {
            idx: i,
            size: self.size[i],
            p: self.p[i],
            hash: self.hashes[i],
            sum_sq_hashes: self.sum_sq_hashes,
        })
    }

    pub fn add_sum_sq_hash(&mut self, h: u64) {
        let h = h.wrapping_mul(h);
        self.sum_sq_hashes = self.sum_sq_hashes.wrapping_add(h);
    }

    pub fn rem_sum_sq_hash(&mut self, h: u64) {
        let h = h.wrapping_mul(h);
        self.sum_sq_hashes = self.sum_sq_hashes.wrapping_sub(h);
    }

    pub fn unite(&mut self, mut v: usize, mut u: usize) {
        v = self.get(v);
        u = self.get(u);
        if v == u {
            return;
        }
        let (smaller, larger) = if self.size[u] < self.size[v] {
            (u, v)
        } else {
            (v, u)
        };
        self.save(smaller);
        self.save(larger);
        self.p[smaller] = larger;
        self.size[larger] += self.size[smaller];
        self.rem_sum_sq_hash(self.hashes[smaller]);
        self.rem_sum_sq_hash(self.hashes[larger]);
        self.hashes[larger] ^= self.hashes[smaller];
        self.add_sum_sq_hash(self.hashes[larger]);
    }
}

fn solve_case(
    start_edges: &[(usize, usize)],
    g: &[Vec<Change>],
    n: usize,
) -> HashMap<u64, Vec<usize>> {
    let n_graphs = g.len();
    let mut events = HashMap::new();

    for &(fr, to) in start_edges.iter() {
        events.insert(Edge::new(fr, to), vec![0]);
    }

    let mut sorted_ids = vec![];

    RecursiveFunction::new(|f, v: usize| {
        sorted_ids.push(v);
        for ch in g[v].iter() {
            events.entry(ch.e).or_default().push(sorted_ids.len());
            f.call(ch.to);
            events.entry(ch.e).or_default().push(sorted_ids.len());
        }
    })
    .call(0);

    let mut vec_events = vec![];
    for (edge, v) in events.iter() {
        for i in (0..v.len()).step_by(2) {
            // if i + 1 < v.len() {
            let from = v[i];
            let to = if i + 1 < v.len() {
                v[i + 1]
            } else {
                sorted_ids.len()
            };
            vec_events.push(Event { from, to, e: *edge });
            // }
        }
    }

    // dbg!(sorted_ids);

    // for e in events.iter() {
    //     dbg!(e);
    // }

    // for e in vec_events.iter() {
    //     dbg!(e);
    // }

    assert!(sorted_ids.len() == n_graphs);
    let mut hashes = vec![0; n_graphs];

    let mut dsu = DsuWithRollbacks::new(n);

    // dbg!("Before rec?");
    // dbg!(events.len());

    RecursiveFunction3::new(|f, l: usize, r: usize, events: Vec<Event>| {
        // if l + 1 == r {
        //     dbg!(l, r, events.len());
        //     for e in events.iter() {
        //         dbg!(l, r, e);
        //     }
        // }
        let mut ev_left = vec![];
        let mut ev_right = vec![];

        let mid = (l + r) >> 1;

        let cur_time = dsu.get_current_time();
        for ev in events.iter() {
            if ev.from <= l && ev.to >= r {
                dsu.unite(ev.e.fr, ev.e.to);
            } else {
                if ev.from < mid {
                    ev_left.push(*ev);
                }
                if ev.to > mid {
                    ev_right.push(*ev);
                }
            }
        }
        if l + 1 == r {
            hashes[sorted_ids[l]] = dsu.sum_sq_hashes;
        } else {
            f.call(l, mid, ev_left);
            f.call(mid, r, ev_right);
        }
        dsu.rollback(cur_time);
    })
    .call(0, sorted_ids.len(), vec_events);
    // dbg!("after rec..");

    let mut group_by: HashMap<u64, Vec<usize>> = HashMap::new();
    for v in 0..n_graphs {
        group_by.entry(hashes[v]).or_default().push(v + 1);
    }
    group_by
}

fn solve(input: &mut Input, _test_case: usize) {
    let n_graphs = input.usize();
    let n = input.usize();
    let m = input.usize();

    let mut start_edges = vec![];
    for _ in 0..m {
        let fr = input.usize() - 1;
        let to = input.usize() - 1;
        start_edges.push((fr, to));
        // events.insert(Edge::new(fr, to), vec![0]);
    }

    let mut g = vec![vec![]; n_graphs];

    for i in 1..n_graphs {
        let prev = input.usize() - 1;
        let add = input.string()[0] == b'a';
        let fr = input.usize() - 1;
        let to = input.usize() - 1;
        g[prev].push(Change {
            e: Edge::new(fr, to),
            to: i,
        })
    }

    let group_by = solve_case(&start_edges, &g, n);

    out_line!(group_by.len());
    for (k, v) in group_by.iter() {
        out!(v.len());
        for &x in v.iter() {
            out!("", x);
        }
        out_line!();
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

fn stress() {
    let n = 100_000;
    let mut rnd = Random::new(787788);

    let mut start_edges = vec![];
    for _ in 0..n {
        let fr = rnd.gen(0..n);
        let to = rnd.gen(0..n);
        start_edges.push((fr, to));
    }

    let n_graphs = n;
    let mut g = vec![vec![]; n_graphs];

    for i in 1..n_graphs {
        let prev = rnd.gen(0..i);
        let fr = rnd.gen(0..n);
        let to = rnd.gen(0..n);
        g[prev].push(Change {
            e: Edge::new(fr, to),
            to: i,
        })
    }

    dbg!("here?");

    let group_by = solve_case(&start_edges, &g, n);
}

//START MAIN
mod tester;

fn main() {
    tester::run_tests();
    // tester::run_single_test("1");
    // tester::run_stress(stress);
}
//END MAIN
