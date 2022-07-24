//{"name":"E. Card Shark","group":"Yandex - Stage 18: Grand Prix of Bytedance","url":"https://official.contest.yandex.com/opencupXXII/contest/39023/problems/E/","interactive":false,"timeLimit":1000,"tests":[{"input":"5 4 3\n0100010\n00100\n001000100\n0010\n0100010\n","output":"2 1 3 5 4\n"},{"input":"4 2 1\n010\n10101\n010\n10101\n","output":"2 1 4 3\n"},{"input":"1 5 3\n001000010000100\n","output":"1\n"},{"input":"2 5 3\n01000\n00010\n","output":"-1\n"},{"input":"1 5 3\n11111\n","output":"-1\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"ECardShark"}}}

use algo_lib::collections::last_exn::LastExn;
use algo_lib::collections::reversed::ReversedTrait;
use algo_lib::io::output::output;
use algo_lib::io::task_io_settings::TaskIoType;
use algo_lib::io::task_runner::run_task;
use algo_lib::io::{input::Input, task_io_settings::TaskIoSettings};
use algo_lib::misc::gen_vector::gen_vec;
#[allow(unused)]
use algo_lib::{dbg, out, out_line};

#[derive(Clone, Copy)]
struct Edge {
    id: usize,
    to: usize,
}

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.usize();
    let m = input.usize();
    let good_pos = input.usize() - 1;
    let strings = gen_vec(n, |_| input.string());
    let mut g = vec![vec![]; m];
    for i in 0..n {
        let mut first_pos = 0;
        while strings[i][first_pos] == b'0' {
            first_pos += 1;
        }
        first_pos %= m;
        let start = (good_pos + m - first_pos) % m;
        let to = (start + strings[i].len()) % m;
        g[start].push(Edge { id: i, to });
    }
    for i in 0..g.len() {
        g[i].sort_by_key(|e| e.id);
    }
    let mut stack: Vec<(Option<Edge>, usize)> = vec![(None, 0)];
    let mut g_it = vec![0; g.len()];
    let mut rev_path = vec![];
    while !stack.is_empty() {
        let (prev_edge, v) = *stack.last_exn();
        if g_it[v] == g[v].len() {
            stack.pop();
            if let Some(e) = prev_edge {
                rev_path.push(e);
            }
        } else {
            let edge = g[v][g_it[v]];
            g_it[v] += 1;
            let to = edge.to;
            stack.push((Some(edge), to));
        }
    }
    let path = rev_path.reversed();
    if path.len() != n {
        out_line!(-1);
        return;
    }
    let mut full_string = vec![];
    for e in path.iter() {
        for c in strings[e.id].iter() {
            full_string.push(*c);
        }
    }
    for pos in 0..full_string.len() {
        let expected_one = (pos % m) == good_pos;
        let real_one = full_string[pos] == b'1';
        if expected_one != real_one {
            out_line!(-1);
            return;
        }
    }
    for e in path.iter() {
        out!(e.id + 1, "");
    }
    out_line!();
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
