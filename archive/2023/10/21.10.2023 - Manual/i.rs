//{"name":"i","group":"Manual","url":"","interactive":false,"timeLimit":2000,"tests":[{"input":"","output":""}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"i"}}}

use algo_lib::io::output::output;
use algo_lib::io::task_io_settings::TaskIoType;
use algo_lib::io::task_runner::run_task;
use algo_lib::io::{input::Input, task_io_settings::TaskIoSettings};
use algo_lib::misc::binary_search::binary_search_first_true;
#[allow(unused)]
use algo_lib::{dbg, out, out_line};

fn solve(input: &mut Input, _test_case: usize) {
    let tc = input.usize();
    for _t in 0..tc {
        let n = input.usize();
        let mut a = vec![];
        for _i in 0..n {
            let mut x = input.i64();
            let mut pw = 0;
            while x != 1 {
                x /= 2;
                pw += 1;
            }
            a.push(pw);
        }
        a.sort();
        let external = binary_search_first_true(0..n, |external| {
            let mut cnt = vec![0i64; 50];
            let mut need = vec![0i64; 50];
            for i in n - external..n {
                if a[i] > 0 {
                    cnt[a[i] - 1] += 1;
                }
            }
            for i in 0..n - external {
                need[a[i]] += 1;
            }
            // let mut add = 0i64;
            let mut extra = 0i64;
            for i in (0..need.len()).rev() {
                // add = add.saturating_add(cnt[i]);
                extra = extra.saturating_add(cnt[i]);
                if need[i] > extra {
                    return false;
                }
                extra -= need[i];
                extra = extra.saturating_add(extra);
                extra = extra.saturating_add(need[i]);
            }
            true
        });
        out_line!(external);
    }
}

pub(crate) fn run(mut input: Input) -> bool {
    solve(&mut input, 1);
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
