//{"name":"B - Two LIS Sum","group":"AtCoder - AtCoder Regular Contest 149","url":"https://atcoder.jp/contests/arc149/tasks/arc149_b","interactive":false,"timeLimit":2000,"tests":[{"input":"5\n5 2 1 4 3\n3 1 2 5 4\n","output":"8\n"},{"input":"5\n1 2 3 4 5\n1 2 3 4 5\n","output":"10\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"BTwoLISSum"}}}

use algo_lib::io::output::output;
use algo_lib::io::task_io_settings::TaskIoType;
use algo_lib::io::task_runner::run_task;
use algo_lib::io::{input::Input, task_io_settings::TaskIoSettings};
use algo_lib::misc::longest_increasing_subsequence::longest_increasing_subsequence;
use algo_lib::misc::min_max::UpdateMinMax;
use algo_lib::misc::vec_apply_delta::ApplyDelta;
#[allow(unused)]
use algo_lib::{dbg, out, out_line};

fn calc(a: &[usize], perm: &[usize]) -> usize {
    let mut applied = vec![];
    for &p in perm.iter() {
        applied.push(a[p]);
    }
    longest_increasing_subsequence(&applied)
}

fn solve(input: &mut Input) {
    let n = input.usize();
    let a = input.vec::<usize>(n).sub_from_all(1);
    let b = input.vec::<usize>(n).sub_from_all(1);

    let mut res = 0;
    for idx in [&a, &b].iter() {
        let mut pos = vec![0; n];
        for i in 0..n {
            pos[idx[i]] = i;
        }
        let cur_res = calc(&a, &pos) + calc(&b, &pos);
        res.update_max(cur_res);
    }
    out_line!(res);
}

pub(crate) fn run(mut input: Input) -> bool {
    solve(&mut input);
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
