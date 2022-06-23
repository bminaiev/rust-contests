//{"name":"C. Keshi ищет AmShZ","group":"Codeforces - Codeforces Round #800 (Div. 1)","url":"https://codeforces.com/contest/1693/problem/C","interactive":false,"timeLimit":2000,"tests":[{"input":"2 1\n1 2\n","output":"1\n"},{"input":"4 4\n1 2\n1 4\n2 4\n1 4\n","output":"2\n"},{"input":"5 7\n1 2\n2 3\n3 5\n1 4\n4 3\n4 5\n3 1\n","output":"4\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"CKeshiIshchetAmShZ"}}}

use algo_lib::collections::min_priority_queue::MinPriorityQueue;
use algo_lib::io::output::output;
use algo_lib::io::task_io_settings::TaskIoType;
use algo_lib::io::task_runner::run_task;
use algo_lib::io::{input::Input, task_io_settings::TaskIoSettings};
#[allow(unused)]
use algo_lib::{dbg, out, out_line};

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
struct Vertex {
    cost: i32,
    v: usize,
}

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.usize();
    let m = input.usize();
    let mut g_rev = vec![vec![]; n];
    let mut sz_out = vec![0; n];
    for _ in 0..m {
        let fr = input.usize() - 1;
        let to = input.usize() - 1;
        g_rev[to].push(fr);
        sz_out[fr] += 1;
    }
    let mut seen = vec![false; n];
    let mut pq = MinPriorityQueue::new();
    pq.push(Vertex { cost: 0, v: n - 1 });
    let mut res = vec![std::i32::MAX; n];
    res[n - 1] = 0;
    while let Some(ver) = pq.pop() {
        let v = ver.v;
        if seen[v] {
            continue;
        }
        seen[v] = true;
        for &fr in g_rev[v].iter() {
            sz_out[fr] -= 1;
            let potential_ans = sz_out[fr] + 1 + res[v];
            if potential_ans < res[fr] {
                res[fr] = potential_ans;
                pq.push(Vertex {
                    cost: potential_ans,
                    v: fr,
                });
            }
        }
    }
    assert_ne!(res[0], std::i32::MAX);
    out_line!(res[0]);
}

pub(crate) fn run(mut input: Input) -> bool {
    solve(&mut input, 1);
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

fn main() {
    tester::run_tests();
    // tester::run_single_test("1");
    // tester::run_stress(stress);
}
//END MAIN
