//{"name":"B. Interesting Subsegments","group":"Yandex - Stage 12: Grand Prix of Grushevka","url":"https://official.contest.yandex.com/opencupXXII/contest/35268/problems/B/","interactive":false,"timeLimit":1000,"tests":[{"input":"5 3\n","output":"0 1 0 1 0\n"},{"input":"5 5\n","output":"-1\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"BInterestingSubsegments"}}}

use algo_lib::io::output::output;
use algo_lib::io::task_io_settings::TaskIoType;
use algo_lib::io::task_runner::run_task;
use algo_lib::io::{input::Input, task_io_settings::TaskIoSettings};
use algo_lib::misc::binary_search::binary_search_last_true;
use algo_lib::misc::min_max::UpdateMinMax;
#[allow(unused)]
use algo_lib::{dbg, out, out_line};

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.usize();
    let need = input.i64();
    let mut res = (0, 0, 0);
    for cnt0 in 1..=n + 1 {
        let more = need - sum(cnt0);
        if more < 0 {
            continue;
        }
        let to_split = n + 1 - cnt0;
        let smallest = (to_split + 1) / 2;
        // smallest -> min sum
        // to_split -> too much
        if let Some(cnt1) = binary_search_last_true(smallest..to_split + 1, |cnt1| -> bool {
            let cnt2 = to_split - cnt1;
            let cur_sum = sum(cnt1) + sum(cnt2);
            cur_sum <= more
        }) {
            assert!(cnt1 <= to_split);
            let cnt2 = n + 1 - cnt0 - cnt1;
            if sum(cnt0) + sum(cnt1) + sum(cnt2) == need {
                assert!(cnt1 >= cnt2);
                res.update_max((cnt0, cnt1, cnt2));
            }
        }
    }
    if res.0 + res.1 + res.2 != n + 1 {
        out_line!(-1);
    } else {
        let mut a = vec![0; n];
        if res.1 != 0 {
            a[res.0 - 1] = 1;
            if res.2 != 0 {
                a[res.0 + res.1 - 1] = 1;
            }
        }
        out_line!(a);
    }
}

fn sum(n: usize) -> i64 {
    let n = n as i64;
    n * (n - 1) / 2
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
}
//END MAIN
