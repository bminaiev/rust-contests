//{"name":"E. Сортировка перестановки","group":"Codeforces - CodeTON Round 7 (Div. 1 + Div. 2, Rated, Prizes!)","url":"https://codeforces.com/contest/1896/problem/E","interactive":false,"timeLimit":4000,"tests":[{"input":"2\n5\n3 2 4 1 5\n6\n2 1 4 6 5 3\n","output":"1 0 1 1 0\n2 1 2 1 0 1\n"}],"testType":"multiNumber","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"ESortirovkaPerestanovki"}}}

use algo_lib::io::output::output;
use algo_lib::io::task_io_settings::TaskIoType;
use algo_lib::io::task_runner::run_task;
use algo_lib::io::{input::Input, task_io_settings::TaskIoSettings};
use algo_lib::misc::vec_apply_delta::ApplyDelta;
use algo_lib::seg_trees::fenwick::Fenwick;
#[allow(unused)]
use algo_lib::{dbg, out, out_line};

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.usize();
    let a = input.vec::<usize>(n).sub_from_all(1);
    let mut res = vec![i32::MAX; n];
    let mut fenw = Fenwick::new(n);
    for i in 0..n {
        fenw.add(i, 1);
    }
    for i in (0..n).rev() {
        if a[i] >= i {
            res[a[i]] = fenw.get_range_sum(i + 1..a[i] + 1);
            fenw.add(a[i], -1);
        } else {
            res[a[i]] = fenw.get_suffix_sum(i + 1);
        }
    }
    for i in (0..n).rev() {
        if a[i] < i {
            res[a[i]] += fenw.get_sum(a[i]);
            fenw.add(a[i], -1);
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
