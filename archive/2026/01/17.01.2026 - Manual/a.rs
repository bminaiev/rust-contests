//{"name":"a","group":"Manual","url":"","interactive":false,"timeLimit":2000,"tests":[{"input":"","output":""}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

#[allow(unused)]
use algo_lib::dbg;
use algo_lib::graph::trees::binary_lifting::BinaryLifting;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::binary_search::binary_search_first_true;
use algo_lib::misc::rand::Random;
use algo_lib::misc::rec_function::{Callable3, RecursiveFunction3};

trait Interactor {
    // Returns true if dist <= dist.
    fn query(&mut self, v: usize, dist: usize) -> bool;
    fn answer(&mut self, v: usize);
}

struct FakeInteractor {
    tree: Vec<Vec<usize>>,
    target: usize,
    queries: usize,
}

impl FakeInteractor {
    fn new(tree: Vec<Vec<usize>>, target: usize) -> Self {
        Self {
            tree,
            target,
            queries: 0,
        }
    }
}

impl Interactor for FakeInteractor {
    fn query(&mut self, v: usize, dist: usize) -> bool {
        self.queries += 1;
        assert!(self.queries <= 40);
        let mut q = std::collections::VecDeque::new();
        let mut seen = vec![false; self.tree.len()];
        q.push_back((v, 0));
        seen[v] = true;
        while let Some((cur, cur_dist)) = q.pop_front() {
            if cur == self.target {
                return cur_dist <= dist;
            }
            for &to in &self.tree[cur] {
                if !seen[to] {
                    seen[to] = true;
                    q.push_back((to, cur_dist + 1));
                }
            }
        }
        false
    }

    fn answer(&mut self, v: usize) {
        dbg!(self.queries);
        assert_eq!(v, self.target);
    }
}

fn solver(tree: &Vec<Vec<usize>>, inter: &mut dyn Interactor) {
    let n = tree.len();

    let depth = binary_search_first_true(0..n + 1, |mid| inter.query(0, mid));

    let binary_lifting = BinaryLifting::new(tree, 0);
    let mut candidates = vec![];
    RecursiveFunction3::new(|f, v: usize, p: usize, h: usize| {
        if h == depth {
            candidates.push(v);
            return;
        }
        for &to in &tree[v] {
            if to != p {
                f.call(to, v, h + 1);
            }
        }
    })
    .call(0, 0, 0);
    while candidates.len() > 1 {
        let mid = candidates[candidates.len() / 2];
        let lca1 = binary_lifting.lca(candidates[0], mid);
        let lca2 = binary_lifting.lca(mid, candidates[candidates.len() - 1]);
        let all_lca = binary_lifting.lca(candidates[0], candidates[candidates.len() - 1]);
        assert!(lca1 == all_lca || lca2 == all_lca);
        assert!(lca1 != all_lca || lca2 != all_lca);
        let my_lca = lca1 ^ lca2 ^ all_lca;
        let lca_h = binary_lifting.height(my_lca);
        let dist_to_target = depth - lca_h;
        let inside = inter.query(my_lca, dist_to_target);
        let mut new_candidates = vec![];
        for &c in &candidates {
            let cur_inside = binary_lifting.lca(c, my_lca) == my_lca;
            if inside == cur_inside {
                new_candidates.push(c);
            }
        }
        assert!(candidates.len() > new_candidates.len());
        candidates = new_candidates;
    }
    inter.answer(candidates[0]);
}

struct RealInteractor<'a> {
    input: &'a mut Input,
    output: &'a mut Output,
}

impl RealInteractor<'_> {
    fn new<'a>(input: &'a mut Input, output: &'a mut Output) -> RealInteractor<'a> {
        RealInteractor { input, output }
    }
}

impl Interactor for RealInteractor<'_> {
    fn query(&mut self, v: usize, dist: usize) -> bool {
        self.output.println(format!("? {} {}", v + 1, dist));
        self.output.flush();
        let res = self.input.usize();
        res == 1
    }

    fn answer(&mut self, v: usize) {
        self.output.println(format!("! {}", v + 1));
        self.output.flush();
    }
}

fn solve(input: &mut Input, out: &mut Output) {
    let tc = input.usize();
    // let mut rnd = Random::new(123);
    for _ in 0..tc {
        // dbg!("start test");
        let n = input.usize();
        let parents = input.vec::<usize>(n - 1);
        let mut g = vec![vec![]; n];
        for (i, &p) in parents.iter().enumerate() {
            g[p - 1].push(i + 1);
            g[i + 1].push(p - 1);
        }
        // let target = rnd.gen_range(0..n);
        // let mut interactor = FakeInteractor::new(g.clone(), target);
        let mut interactor = RealInteractor::new(input, out);
        solver(&g, &mut interactor);
    }
}

fn stress() {
    for it in 1.. {
        dbg!(it);
        let mut rnd = Random::new(it);
        let n = 30_000;
        let mut p = vec![0usize; n];
        let mut cnt_children = vec![0usize; n];
        for i in 1..n {
            loop {
                p[i] = rnd.gen_range(0..i);
                if cnt_children[p[i]] < 2 {
                    cnt_children[p[i]] += 1;
                    break;
                }
            }
        }
        let mut g = vec![vec![]; n];
        for i in 1..n {
            g[p[i]].push(i);
            g[i].push(p[i]);
        }
        let target = rnd.gen_range(0..n);
        let mut interactor = FakeInteractor::new(g.clone(), target);
        solver(&g, &mut interactor);
    }
}

pub(crate) fn run(mut input: Input, mut output: Output) -> bool {
    solve(&mut input, &mut output);
    output.flush();
    true
}

#[cfg(feature = "local")]
fn main() {
    const PROBLEM_NAME: &str = "a";
    use algo_lib::tester::helper::*;

    // run_tests(PROBLEM_NAME, run);
    // run_single_test(PROBLEM_NAME, run, "1");
    // run_stress(stress);
    run_locally(run);
}
//END MAIN

#[cfg(not(feature = "local"))]
fn main() {
    let input = algo_lib::io::input::Input::new_stdin();
    let mut output = algo_lib::io::output::Output::new_stdout();
    run(input, output);
}
