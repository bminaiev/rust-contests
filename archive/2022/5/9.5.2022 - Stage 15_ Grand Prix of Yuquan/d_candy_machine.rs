//{"name":"D. Candy Machine","group":"Yandex - Stage 15: Grand Prix of Yuquan","url":"https://official.contest.yandex.com/opencupXXII/contest/37831/problems/D/","interactive":false,"timeLimit":1000,"tests":[{"input":"5\n1 2 3 4 5\n","output":"2\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"DCandyMachine"}}}

use algo_lib::collections::reversed::ReversedTrait;
use algo_lib::collections::sorted::SortedTrait;
use algo_lib::io::output::output;
use algo_lib::io::task_io_settings::TaskIoType;
use algo_lib::io::task_runner::run_task;
use algo_lib::io::{input::Input, task_io_settings::TaskIoSettings};
use algo_lib::misc::binary_search::binary_search_last_true;
use algo_lib::misc::min_max::UpdateMinMax;
use algo_lib::misc::pref_sum::PrefSum;
#[allow(unused)]
use algo_lib::{dbg, out, out_line};

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.usize();
    let a = input.vec::<i64>(n).sorted();
    let pref = a.pref_sum();
    let mut res = 0;
    for cnt_pref in 1..n {
        let max_cnt = n - cnt_pref;
        let real_max = binary_search_last_true(1..max_cnt + 1, |check_cnt: usize| -> bool {
            let total_sum = pref[check_cnt + cnt_pref];
            let last_elem = a[cnt_pref];
            last_elem * (check_cnt + cnt_pref) as i64 > total_sum
        });
        if let Some(mx) = real_max {
            res.update_max(mx);
        }
    }
    out_line!(res);
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
