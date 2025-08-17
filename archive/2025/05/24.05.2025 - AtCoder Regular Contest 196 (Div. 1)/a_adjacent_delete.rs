//{"name":"A - Adjacent Delete","group":"AtCoder - AtCoder Regular Contest 196 (Div. 1)","url":"https://atcoder.jp/contests/arc196/tasks/arc196_a","interactive":false,"timeLimit":2000,"tests":[{"input":"4\n1 2 5 3\n","output":"5\n"},{"input":"7\n3 1 4 1 5 9 2\n","output":"14\n"},{"input":"5\n1 1 1 1 1\n","output":"0\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"AAdjacentDelete"}}}

use std::collections::BTreeSet;

#[allow(unused)]
use algo_lib::dbg;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
struct Item {
    value: i64,
    index: usize,
}

#[derive(Debug)]
struct Solver {
    top: BTreeSet<Item>,
    bottom: BTreeSet<Item>,
    top_sum: i64,
    bottom_sum: i64,
}

impl Solver {
    pub fn new() -> Self {
        Self {
            top: BTreeSet::new(),
            bottom: BTreeSet::new(),
            top_sum: 0,
            bottom_sum: 0,
        }
    }

    fn add_number(&mut self, number: i64, index: usize) {
        let item = Item {
            value: number,
            index,
        };
        if self.top.is_empty() || item > *self.top.iter().next().unwrap() {
            self.top.insert(item);
            self.top_sum += number;
        } else {
            self.bottom.insert(item);
            self.bottom_sum += number;
        }
        self.balance();
    }

    fn balance(&mut self) {
        if self.top.len() > self.bottom.len() + 1 {
            let item = self.top.iter().next().unwrap().clone();
            self.top.remove(&item);
            self.top_sum -= item.value;
            self.bottom.insert(item);
            self.bottom_sum += item.value;
        } else if self.bottom.len() > self.top.len() {
            let item = self.bottom.iter().next_back().unwrap().clone();
            self.bottom.remove(&item);
            self.bottom_sum -= item.value;
            self.top.insert(item);
            self.top_sum += item.value;
        }
    }

    fn get_ans(&self) -> i64 {
        self.top_sum - self.bottom_sum
    }
}

fn solve(input: &mut Input, out: &mut Output, _test_case: usize) {
    let n = input.usize();
    let a = input.vec::<i64>(n);
    let mut solver = Solver::new();
    let mut prefix_ans = vec![0];
    for i in 0..n {
        solver.add_number(a[i], i);
        prefix_ans.push(solver.get_ans());
    }
    let mut solver = Solver::new();
    let mut suffix_ans = vec![0];
    for i in (0..n).rev() {
        solver.add_number(a[i], i);
        suffix_ans.push(solver.get_ans());
        // dbg!("Add number", a[i], solver);
    }
    // dbg!(suffix_ans);
    if n % 2 == 0 {
        out.println(prefix_ans[n]);
    } else {
        let mut res = 0;
        for rem in 0..n {
            let cur = prefix_ans[rem] + suffix_ans[n - rem - 1];
            if cur > res && rem % 2 == 0 {
                res = cur;
            }
        }
        out.println(res);
    }
}

pub(crate) fn run(mut input: Input, mut output: Output) -> bool {
    solve(&mut input, &mut output, 1);
    output.flush();
    true
}

//START MAIN

fn main() {
    const PROBLEM_NAME: &str = "a_adjacent_delete";
    use algo_lib::tester::helper::*;

    run_tests(PROBLEM_NAME, run);
    // run_single_test(PROBLEM_NAME, run, "2");
    // run_stress(stress);
    // run_locally(run);
}
//END MAIN
