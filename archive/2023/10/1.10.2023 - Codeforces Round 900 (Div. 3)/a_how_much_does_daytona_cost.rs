//{"name":"A. How Much Does Daytona Cost?","group":"Codeforces - Codeforces Round 900 (Div. 3)","url":"https://codeforces.com/contest/1878/problem/A","interactive":false,"timeLimit":1000,"tests":[{"input":"7\n5 4\n1 4 3 4 1\n4 1\n2 3 4 4\n5 6\n43 5 60 4 2\n2 5\n1 5\n4 1\n5 3 3 1\n1 3\n3\n5 3\n3 4 1 5 5\n","output":"YES\nNO\nNO\nYES\nYES\nYES\nYES\n"}],"testType":"multiNumber","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"AHowMuchDoesDaytonaCost"}}}

use algo_lib::io::output::output;
use algo_lib::io::task_io_settings::TaskIoType;
use algo_lib::io::task_runner::run_task;
use algo_lib::io::{input::Input, task_io_settings::TaskIoSettings};
use algo_lib::misc::vec_apply_delta::ApplyDelta;
#[allow(unused)]
use algo_lib::{dbg, out, out_line};

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.usize();
    let k = input.usize() - 1;
    let a = input.vec::<usize>(n).sub_from_all(1);
    let mut glob_ok = false;
    for left in 0..n {
        let mut used = vec![0; 101];
        for right in left..n {
            used[a[right]] += 1;
            let mut ok = true;
            for x in 0..used.len() {
                if x != k && used[x] > used[k] {
                    ok = false;
                }
            }
            glob_ok |= ok;
        }
    }
    if glob_ok {
        out_line!("YES");
    } else {
        out_line!("NO");
    }
}

pub(crate) fn run(mut input: Input) -> bool {
    let t = input.read();
    for i in 0usize..t {
        solve(&mut input, i + 1);
    }
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
