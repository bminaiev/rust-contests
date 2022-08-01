//{"name":"B. Люк гурман","group":"Codeforces - CodeTON Round 2 (Div. 1 + Div. 2, Rated, Prizes!)","url":"https://codeforces.com/contest/1704/problem/B","interactive":false,"timeLimit":1000,"tests":[{"input":"7\n5 3\n3 8 5 6 7\n5 3\n3 10 9 8 7\n12 8\n25 3 3 17 8 6 1 16 15 25 17 23\n10 2\n1 2 3 4 5 6 7 8 9 10\n8 2\n2 4 6 8 6 4 12 14\n8 2\n2 7 8 9 6 13 21 28\n15 5\n11 4 13 23 7 10 5 21 20 11 17 5 29 16 11\n","output":"0\n1\n2\n1\n2\n4\n6\n"}],"testType":"multiNumber","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"BLyukGurman"}}}

use algo_lib::io::output::output;
use algo_lib::io::task_io_settings::TaskIoType;
use algo_lib::io::task_runner::run_task;
use algo_lib::io::{input::Input, task_io_settings::TaskIoSettings};
use algo_lib::misc::range_intersect::range_intersect;
#[allow(unused)]
use algo_lib::{dbg, out, out_line};

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.usize();
    let x = input.i64();
    let a = input.vec::<i64>(n);
    let mut r = std::i64::MIN..std::i64::MAX;
    let mut res = 0;
    for a in a.into_iter() {
        let cur = a - x..a + x;
        let inter = range_intersect(cur.clone(), r);
        if inter.start <= inter.end {
            r = inter;
        } else {
            res += 1;
            r = cur;
        }
    }
    out_line!(res);
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
    // tester::run_stress(stress);
}
//END MAIN
