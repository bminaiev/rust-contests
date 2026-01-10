//{"name":"f","group":"Manual","url":"","interactive":false,"timeLimit":2000,"tests":[{"input":"","output":""}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::min_priority_queue::MinPriorityQueue;
#[allow(unused)]
use algo_lib::dbg;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;

#[derive(Clone, Copy)]
struct Remote {
    to: usize,
    dist: i64,
}

struct CentroidDecomposition {
    alive: Vec<bool>,
    size: Vec<usize>,
    ups: Vec<Vec<Remote>>,
    children: Vec<Vec<Remote>>,
}

impl CentroidDecomposition {
    fn new(g: &[Vec<usize>]) -> Self {
        let n = g.len();
        let mut res = Self {
            alive: vec![true; n],
            size: vec![0; n],
            ups: vec![vec![]; n],
            children: vec![vec![]; n],
        };
        res.rec(g, 0);
        res
    }

    fn rec(&mut self, g: &[Vec<usize>], mut root: usize) {
        self.calc_sizes(g, root, root);
        let full_size = self.size[root];
        let mut prev = root;
        loop {
            let mut found = false;
            for &to in &g[root] {
                if to != prev && self.alive[to] && self.size[to] * 2 > full_size {
                    prev = root;
                    root = to;
                    found = true;
                    break;
                }
            }
            if !found {
                break;
            }
        }
        self.alive[root] = false;
        self.build_paths(g, root, root, 0, root);
        for &to in &g[root] {
            if self.alive[to] {
                self.rec(g, to);
            }
        }
    }

    fn calc_sizes(&mut self, g: &[Vec<usize>], v: usize, p: usize) {
        self.size[v] = 1;
        for &to in &g[v] {
            if to != p && self.alive[to] {
                self.calc_sizes(g, to, v);
                self.size[v] += self.size[to];
            }
        }
    }

    fn build_paths(&mut self, g: &[Vec<usize>], v: usize, p: usize, dist: i64, centroid: usize) {
        self.ups[v].push(Remote { to: centroid, dist });
        self.children[centroid].push(Remote { to: v, dist });
        for &to in &g[v] {
            if to != p && self.alive[to] {
                self.build_paths(g, to, v, dist + 1, centroid);
            }
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
struct Event {
    time: i64,
    v: usize,
    lang: i64,
    visit: bool,
}

fn solve(input: &mut Input, out: &mut Output) {
    let n = input.usize();
    let mut g = vec![vec![]; n];
    for _ in 0..n - 1 {
        let u = input.usize() - 1;
        let v = input.usize() - 1;
        g[u].push(v);
        g[v].push(u);
    }
    let mut from = vec![];
    let mut to = vec![];
    for _ in 0..n {
        from.push(input.i64());
        to.push(input.i64());
    }
    if to[n - 1] < from[0] {
        // reverse
        for i in 0..n {
            let nfr = -to[i];
            let nto = -from[i];
            from[i] = nfr;
            to[i] = nto;
        }
    }
    let start = from[0];
    const INF: i64 = 1e18 as i64;
    for i in 0..n {
        if to[i] < start {
            from[i] = INF;
            to[i] = INF;
        } else {
            from[i] = (from[i] - start).max(0);
            to[i] = to[i] - start;
            assert!(to[i] >= from[i]);
        }
    }
    assert!(from[n - 1] != INF);
    let mut cd = CentroidDecomposition::new(&g);
    for v in 0..n {
        cd.children[v].sort_by_key(|u| from[u.to]);
    }
    let mut pq = MinPriorityQueue::new();
    pq.push(Event {
        time: 0,
        v: 0,
        lang: to[0],
        visit: true,
    });
    let mut child_iter = vec![0; n];
    let mut max_seen_lang = vec![-1; n];
    let mut seen = vec![false; n];
    while let Some(Event {
        time,
        v,
        lang,
        visit,
    }) = pq.pop()
    {
        if visit {
            if seen[v] {
                continue;
            }
            if v == n - 1 {
                out.println(time - 1);
                return;
            }
            seen[v] = true;
            for tos in &cd.ups[v] {
                pq.push(Event {
                    time: time + tos.dist,
                    v: tos.to,
                    lang,
                    visit: false,
                });
            }
        } else {
            if max_seen_lang[v] >= lang {
                continue;
            }
            max_seen_lang[v] = lang;
            while child_iter[v] < cd.children[v].len()
                && from[cd.children[v][child_iter[v]].to] <= lang
            {
                let child = &cd.children[v][child_iter[v]];
                let arrive_at = time + child.dist + 1;
                pq.push(Event {
                    time: arrive_at,
                    v: child.to,
                    lang: to[child.to],
                    visit: true,
                });

                child_iter[v] += 1;
            }
        }
    }
    unreachable!();
}

pub(crate) fn run(mut input: Input, mut output: Output) -> bool {
    solve(&mut input, &mut output);
    output.flush();
    true
}

#[cfg(feature = "local")]
fn main() {
    const PROBLEM_NAME: &str = "f";
    use algo_lib::tester::helper::*;

    run_tests(PROBLEM_NAME, run);
    // run_single_test(PROBLEM_NAME, run, "1");
    // run_stress(stress);
    // run_locally(run);
}
//END MAIN

#[cfg(not(feature = "local"))]
fn main() {
    let input = algo_lib::io::input::Input::new_stdin();
    let mut output = algo_lib::io::output::Output::new_stdout();
    run(input, output);
}
