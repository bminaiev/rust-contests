//{"name":"G. Падение","group":"Codeforces - Codeforces Round #784 (Div. 4)","url":"https://codeforces.com/contest/1669/problem/G","interactive":false,"timeLimit":1000,"tests":[{"input":"3\n6 10\n.*.*....*.\n.*.......*\n...o....o.\n.*.*....*.\n..........\n.o......o*\n2 9\n...***ooo\n.*o.*o.*o\n5 5\n*****\n*....\n*****\n....*\n*****\n","output":"..........\n...*....*.\n.*.o....o.\n.*........\n.*......**\n.o.*....o*\n\n....**ooo\n.*o**o.*o\n\n.....\n*...*\n*****\n*****\n*****\n"}],"testType":"multiNumber","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"GPadenie"}}}

use algo_lib::io::output::output;
use algo_lib::io::task_io_settings::TaskIoType;
use algo_lib::io::task_runner::run_task;
use algo_lib::io::{input::Input, task_io_settings::TaskIoSettings};
use algo_lib::misc::gen_vector::gen_vec;
use algo_lib::strings::utils::vec2str;
#[allow(unused)]
use algo_lib::{dbg, out, out_line};

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.usize();
    let m = input.usize();
    let mut a = gen_vec(n, |_| input.string());
    a.reverse();
    for col in 0..m {
        let mut i = 0;
        while i < n {
            let mut j = i;
            let mut cnt = 0;
            while j != n && a[j][col] != b'o' {
                if a[j][col] == b'*' {
                    cnt += 1;
                }
                j += 1;
            }
            for x in 0..cnt {
                a[i + x][col] = b'*';
            }
            for x in cnt..(j - i) {
                a[i + x][col] = b'.';
            }

            i = j + 1;
        }
    }
    a.reverse();
    for s in a.iter() {
        out_line!(vec2str(s));
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
