//{"name":"A. Attack Order","group":"Yandex - Stage 13: Grand Prix of Gomel","url":"https://official.contest.yandex.com/opencupXXII/contest/35270/problems/?nc=EaYpk9H8","interactive":false,"timeLimit":2000,"tests":[{"input":"3\n2\n15 25\n10 5\n3\n7 0\n7 3\n10 0\n3\n10 10\n20 20\n30 30\n","output":"Yes\nYes\nNo\n"}],"testType":"multiNumber","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"AAttackOrder"}}}

use algo_lib::io::output::output;
use algo_lib::io::task_io_settings::TaskIoType;
use algo_lib::io::task_runner::run_task;
use algo_lib::io::{input::Input, task_io_settings::TaskIoSettings};
use algo_lib::misc::gen_vector::gen_vec;
#[allow(unused)]
use algo_lib::{dbg, out, out_line};

#[derive(Debug)]
struct Person {
    a: i64,
    b: i64,
}

fn ok(a: &mut [Person]) -> bool {
    if a.len() == 2 {
        return true;
    }
    a.sort_by_key(|p| (p.a, -p.b));
    a.reverse();
    let mut sum_b = 0;
    for p in a.iter() {
        sum_b += p.b;
    }
    for w in a.windows(2) {
        let first = w[0].a;
        let second = w[1].a + sum_b - w[1].b;
        if second > first {
            return false;
        }
    }
    true
}

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.usize();
    let mut a = gen_vec(n, |_| Person {
        a: input.read(),
        b: input.read(),
    });
    if ok(&mut a) {
        out_line!("Yes");
    } else {
        out_line!("No");
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
