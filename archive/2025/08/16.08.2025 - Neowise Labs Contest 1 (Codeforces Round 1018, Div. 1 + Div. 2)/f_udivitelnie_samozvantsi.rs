//{"name":"F. Удивительные самозванцы","group":"Codeforces - Neowise Labs Contest 1 (Codeforces Round 1018, Div. 1 + Div. 2)","url":"https://codeforces.com/contest/2096/problem/F","interactive":false,"timeLimit":2000,"tests":[{"input":"4\n4 3\n1 1 3\n1 2 4\n0 2 3\n1\n1 3\n5 2\n0 1 5\n1 1 5\n3\n1 1\n2 2\n1 2\n1 2\n0 1 1\n1 1 1\n2\n1 1\n2 2\n7 9\n1 2 2\n1 4 5\n0 5 6\n1 2 2\n1 1 1\n0 4 7\n0 3 7\n0 2 7\n0 6 6\n5\n1 5\n2 6\n3 7\n4 8\n5 9\n","output":"YES\nYES\nYES\nNO\nYES\nYES\nYES\nNO\nYES\nNO\nYES\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"FUdivitelnieSamozvantsi"}}}

#[allow(unused)]
use algo_lib::dbg;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::rec_function::{Callable3, RecursiveFunction3};

#[derive(Clone, Copy, Default)]
struct Seg {
    all_good: bool,
    left: usize,
    right: usize,
}

#[derive(Clone, Copy, Default)]
struct Query {
    id: usize,
    left: usize,
    right: usize,
}

#[derive(Clone, Copy, Default)]
struct Node {
    time: usize,
    lost: bool,
    first_empty: usize,
    lost_if_covered: bool,
    need_first_empty_before: usize,
    end: usize,
}

fn join(lhs: &Node, rhs: &Node) -> Node {
    let mut lost = lhs.lost || rhs.lost;
    if lhs.need_first_empty_before <= rhs.first_empty {
        lost = true;
    }
    let mut need_first_empty_before = rhs.need_first_empty_before;
    if lhs.need_first_empty_before >= rhs.end {
        need_first_empty_before = need_first_empty_before.min(lhs.first_empty);
    }
    Node {
        time: lhs.time.max(rhs.time),
        lost,
        first_empty: lhs.first_empty.min(rhs.first_empty),
        lost_if_covered: lhs.lost_if_covered
            || rhs.lost_if_covered
            || lhs.need_first_empty_before <= rhs.end,
        need_first_empty_before,
        end: rhs.end,
    }
}

struct Solver {
    n: usize,
    timer: usize,
    nodes: Vec<Vec<Node>>,
}

impl Solver {
    fn new(n: usize) -> Self {
        let mut nodes = vec![vec![]; 4 * n];
        Self { n, timer: 0, nodes }
    }

    fn init(&mut self, v: usize, l: usize, r: usize) {
        if l + 1 == r {
            self.nodes[v] = vec![Node {
                time: self.timer,
                lost: false,
                first_empty: l,
                lost_if_covered: false,
                need_first_empty_before: usize::MAX,
                end: r,
            }];
        } else {
            let m = (l + r) / 2;
            self.init(2 * v + 1, l, m);
            self.init(2 * v + 2, m, r);
            let node = join(&self.nodes[2 * v + 1][0], &self.nodes[2 * v + 2][0]);
            self.nodes[v] = vec![node];
        }
    }
}

fn solve(input: &mut Input, out: &mut Output, _test_case: usize) {
    let tc = input.usize();
    for _ in 0..tc {
        let n = input.usize();
        let m = input.usize();
        let mut segs = vec![];
        for i in 0..n {
            let all_good = input.usize() == 0;
            let left = input.usize() - 1;
            let right = input.usize();
            segs.push(Seg {
                all_good,
                left,
                right,
            });
        }
        let mut solver = Solver::new(n);
        solver.init(0, 0, n);
        let mut queries = vec![];
        for i in 0..m {
            let left = input.usize() - 1;
            let right = input.usize();
            queries.push(Query { id: i, left, right });
        }
        let mut res = vec![false; m];
        // rec(queries, left, right)
        RecursiveFunction3::new(|f, queries: Vec<Query>, left: usize, right: usize| {
            
        })
            .call(queries, 0, n);
        for i in 0..m {
            if res[i] {
                out.println("Yes");
            } else {
                out.println("No");
            }
        }
    }
}

pub(crate) fn run(mut input: Input, mut output: Output) -> bool {
    solve(&mut input, &mut output, 1);
    output.flush();
    true
}

//START MAIN

fn main() {
    const PROBLEM_NAME: &str = "f_udivitelnie_samozvantsi";
    use algo_lib::tester::helper::*;

    run_tests(PROBLEM_NAME, run);
    // run_single_test(PROBLEM_NAME, run, "1");
    // run_stress(stress);
    // run_locally(run);
}
//END MAIN
