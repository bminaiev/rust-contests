//{"name":"C. Привнеси баланс","group":"Codeforces - Codeforces Round #794 (Div. 1)","url":"https://codeforces.com/contest/1685/problem/C","interactive":false,"timeLimit":1000,"tests":[{"input":"3\n2\n(())\n5\n())((()))(\n6\n())((()))(()\n","output":"0\n2\n3 4\n9 10\n1\n2 11\n"}],"testType":"multiNumber","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"CPrivnesiBalans"}}}

use algo_lib::io::output::output;
use algo_lib::io::task_io_settings::TaskIoType;
use algo_lib::io::task_runner::run_task;
use algo_lib::io::{input::Input, task_io_settings::TaskIoSettings};
#[allow(unused)]
use algo_lib::{dbg, out, out_line};

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug)]
struct Elem {
    balance: i32,
    pos: usize,
}

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.usize() * 2;
    let s = input.string();
    let mut segs = vec![];
    let mut balance = 0;
    let mut elems = vec![];
    let mut cur_balance = vec![0; n];
    for (pos, x) in s.iter().enumerate() {
        if *x == b'(' {
            balance += 1;
        } else {
            balance -= 1;
        }
        cur_balance[pos] = balance;
        elems.push(Elem { balance, pos });
    }
    elems.sort();
    elems.reverse();
    dbg!(elems);
    dbg!(cur_balance);
    let mut it = 0;
    let mut i = 0;
    while i != n {
        if cur_balance[i] < 0 {
            while elems[it].pos < i {
                it += 1;
            }
            segs.push((i, elems[it].pos + 1));
            i = elems[it].pos + 1;
        } else {
            i += 1;
        }
    }
    out_line!(segs.len());
    for (fr, to) in segs.into_iter() {
        out_line!(fr + 1, to);
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

//START MAIN
mod tester;

fn main() {
    tester::run_tests();
    // tester::run_single_test("1");
}
//END MAIN
