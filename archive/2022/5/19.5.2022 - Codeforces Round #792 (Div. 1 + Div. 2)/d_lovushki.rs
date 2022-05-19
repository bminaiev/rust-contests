//{"name":"D. Ловушки","group":"Codeforces - Codeforces Round #792 (Div. 1 + Div. 2)","url":"https://codeforces.com/contest/1684/problem/D","interactive":false,"timeLimit":1000,"tests":[{"input":"5\n4 4\n8 7 1 4\n4 1\n5 10 11 5\n7 5\n8 2 5 15 11 2 8\n6 3\n1 2 3 4 5 6\n1 1\n7\n","output":"0\n21\n9\n6\n0\n"}],"testType":"multiNumber","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"DLovushki"}}}

use algo_lib::io::output::output;
use algo_lib::io::task_io_settings::TaskIoType;
use algo_lib::io::task_runner::run_task;
use algo_lib::io::{input::Input, task_io_settings::TaskIoSettings};
use algo_lib::misc::gen_vector::gen_vec;
#[allow(unused)]
use algo_lib::{dbg, out, out_line};

struct Element {
    pos: usize,
    value: i64,
}

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.usize();
    let k = input.usize();
    let a = input.vec::<i64>(n);
    let mut elems = gen_vec(n, |pos| Element { pos, value: a[pos] });
    elems.sort_by_key(|e| e.pos as i64 + e.value);
    let mut should_jump = vec![false; n];
    for e in elems.iter().rev().take(k) {
        should_jump[e.pos] = true;
    }
    let mut score = 0;
    let mut add = 0;
    for i in 0..n {
        if should_jump[i] {
            add += 1;
        } else {
            score += add + a[i];
        }
    }
    out_line!(score);
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

//START MAIN
mod tester;

fn main() {
    tester::run_tests();
    // tester::run_single_test("1");
}
//END MAIN
