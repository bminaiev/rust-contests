//{"name":"C. Перестановка столбцов","group":"Codeforces - Codeforces Round #792 (Div. 1 + Div. 2)","url":"https://codeforces.com/contest/1684/problem/C","interactive":false,"timeLimit":1000,"tests":[{"input":"5\n2 3\n1 2 3\n1 1 1\n2 2\n4 1\n2 3\n2 2\n2 1\n1 1\n2 3\n6 2 1\n5 4 3\n2 1\n1\n2\n","output":"1 1\n-1\n1 2\n1 3\n1 1\n"}],"testType":"multiNumber","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"CPerestanovkaStolbtsov"}}}

use algo_lib::collections::sorted::SortedTrait;
use algo_lib::io::output::output;
use algo_lib::io::task_io_settings::TaskIoType;
use algo_lib::io::task_runner::run_task;
use algo_lib::io::{input::Input, task_io_settings::TaskIoSettings};
use algo_lib::misc::gen_vector::gen_vec;
#[allow(unused)]
use algo_lib::{dbg, out, out_line};

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.usize();
    let m = input.usize();
    let mut a = input.matrix::<i64>(n, m).transpose();
    let sums = gen_vec(a.len(), |col| a[col].iter().sum::<i64>());
    let sorted_sums = sums.sorted();
    let bad_columns: Vec<_> = (0..sums.len())
        .filter(|&idx| sums[idx] != sorted_sums[idx])
        .collect();
    if bad_columns.len() != 2 && bad_columns.len() != 0 {
        out_line!(-1i32);
        return;
    }

    if !bad_columns.is_empty() {
        a.swap(bad_columns[0], bad_columns[1]);
    }
    a = a.transpose();
    if (0..a.len()).any(|r| a[r] != a[r].to_vec().sorted()) {
        out_line!(-1);
        return;
    }

    if bad_columns.is_empty() {
        out_line!(1, 1);
    } else {
        out_line!(bad_columns[0] + 1, bad_columns[1] + 1);
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
