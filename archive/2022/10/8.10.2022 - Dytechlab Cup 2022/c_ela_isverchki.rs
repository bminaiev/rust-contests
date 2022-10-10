//{"name":"C. Эла и сверчки","group":"Codeforces - Dytechlab Cup 2022","url":"https://codeforces.com/contest/1737/problem/C","interactive":false,"timeLimit":1000,"tests":[{"input":"6\n8\n7 2 8 2 7 1\n5 1\n8\n2 2 1 2 2 1\n5 5\n8\n2 2 1 2 2 1\n6 6\n8\n1 1 1 2 2 1\n5 5\n8\n2 2 1 2 2 1\n8 8\n8\n8 8 8 7 7 8\n4 8\n","output":"YES\nNO\nYES\nNO\nYES\nYES\n"}],"testType":"multiNumber","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"CElaISverchki"}}}

use algo_lib::io::output::output;
use algo_lib::io::task_io_settings::TaskIoType;
use algo_lib::io::task_runner::run_task;
use algo_lib::io::{input::Input, task_io_settings::TaskIoSettings};
#[allow(unused)]
use algo_lib::{dbg, out, out_line};

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.i32();
    let mut xx = 0;
    let mut yy = 0;
    let mut cor = 10;
    for _ in 0..3 {
        let x1 = input.i32();
        let y1 = input.i32();
        xx ^= x1;
        yy ^= y1;
        if x1 == 1 && y1 == 1 {
            cor = 0;
        }
        if x1 == 1 && y1 == n {
            cor = 1;
        }
        if x1 == n && y1 == 1 {
            cor = 2;
        }
        if x1 == n && y1 == n {
            cor = 3;
        }
    }
    let need_x = input.i32();
    let need_y = input.i32();
    let mut fail = (need_x - xx).abs() % 2 == 0 && (need_y - yy).abs() % 2 == 0;
    if cor == 0 && need_x != 1 && need_y != 1 {
        fail = true;
    }
    if cor == 1 && need_x != 1 && need_y != n {
        fail = true;
    }
    if cor == 2 && need_x != n && need_y != 1 {
        fail = true;
    }
    if cor == 3 && need_x != n && need_y != n {
        fail = true;
    }
    if fail {
        out_line!("NO");
    } else {
        out_line!("YES");
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
    // tester::run_stress(stress);
    // tester::run_locally();
    // tester::run_with_last_downloaded_file();
}
//END MAIN
