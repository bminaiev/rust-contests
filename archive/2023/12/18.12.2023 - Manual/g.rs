//{"name":"g","group":"Manual","url":"","interactive":false,"timeLimit":2000,"tests":[{"input":"","output":""}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"g"}}}

use std::collections::VecDeque;

use algo_lib::collections::array_2d::Array2D;
use algo_lib::io::output::output;
use algo_lib::io::task_io_settings::TaskIoType;
use algo_lib::io::task_runner::run_task;
use algo_lib::io::{input::Input, task_io_settings::TaskIoSettings};
use algo_lib::misc::two_min::TwoMin;
#[allow(unused)]
use algo_lib::{dbg, out, out_line};

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.usize();
    let m = input.usize();
    let mut a = vec![];
    let mut b = vec![];
    for _ in 0..n {
        a.push(input.i64());
        b.push(input.i64());
    }
    let mut g = vec![vec![]; n];
    for _ in 0..m {
        let fr = input.usize() - 1;
        let to = input.usize() - 1;
        g[fr].push(to);
        g[to].push(fr);
    }
    g[0].sort();
    let mut dist0 = vec![std::i64::MAX; n];
    dist0[0] = 0;
    let mut queue = VecDeque::new();
    queue.push_back(0);
    while let Some(v) = queue.pop_front() {
        for &to in &g[v] {
            if dist0[to] == std::i64::MAX {
                dist0[to] = dist0[v] + 1;
                queue.push_back(to);
            }
        }
    }
    let mut res = Array2D::new(TwoMin::new(n, std::i64::MAX), n, 3);
    let mut queries = vec![];
    for v in 0..n {
        for shift in -1..=1 {
            let len = dist0[v] + shift;
            if len >= 0 {
                queries.push(Query { len, v });
            }
        }
    }
    queries.sort();
    for query in queries.iter().rev() {
        let mut cur_res = TwoMin::new(n, std::i64::MAX);
        cur_res.add(query.v, -(a[query.v] - b[query.v] * query.len));
        for &to in &g[query.v] {
            let child_res = get_res(&res, to, query.len + 1, &dist0);
            cur_res.merge(&child_res);
        }
        res[query.v][(query.len + 1 - dist0[query.v]) as usize] = cur_res;
    }
    for &to in g[0].iter() {
        let ans = get_res(&res, to, 0, &dist0);
        let ans = ans.get_by_not_id(to).unwrap();
        out_line!(-ans.1);
    }
}

fn get_res(
    res: &Array2D<TwoMin<usize, i64>>,
    v: usize,
    len: i64,
    dist0: &[i64],
) -> TwoMin<usize, i64> {
    assert!(len >= dist0[v] - 1);
    if len > dist0[v] + 1 {
        return TwoMin::new(usize::MAX, std::i64::MAX);
    }
    res[v][(len + 1 - dist0[v]) as usize].clone()
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
struct Query {
    len: i64,
    v: usize,
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
    // tester::run_single_test("1");
    // tester::run_stress(stress);
    // tester::run_locally();
    // tester::run_with_last_downloaded_file();
}
//END MAIN
