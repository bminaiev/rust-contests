//{"name":"A. Выведите пьедестал (логотип Codeforces?)","group":"Codeforces - Codeforces Round #797 (Div. 3)","url":"https://codeforces.com/contest/1690/problem/0","interactive":false,"timeLimit":1000,"tests":[{"input":"6\n11\n6\n10\n100000\n7\n8\n","output":"4 5 2\n2 3 1\n4 5 1\n33334 33335 33331\n2 4 1\n3 4 1\n"}],"testType":"multiNumber","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"AViveditePedestalLogotipCodeforces"}}}

use algo_lib::io::output::output;
use algo_lib::io::task_io_settings::TaskIoType;
use algo_lib::io::task_runner::run_task;
use algo_lib::io::{input::Input, task_io_settings::TaskIoSettings};
#[allow(unused)]
use algo_lib::{dbg, out, out_line};

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct Sol {
    first: i32,
    second: i32,
    third: i32,
}

fn solve(input: &mut Input, _test_case: usize) {
    let sum = input.i32();
    let mut ways = vec![];
    let mid = sum / 3;
    const M: i32 = 4;
    for x in -M..M {
        for y in -M..M {
            for z in -M..M {
                let first = mid + x;
                let second = mid + y;
                let third = mid + z;
                if first > second && second > third {
                    if first + second + third == sum && third > 0 {
                        ways.push(Sol {
                            first,
                            second,
                            third,
                        });
                    }
                }
            }
        }
    }
    ways.sort();
    let w = ways[0];
    out_line!(w.second, w.first, w.third);
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
