//{"name":"F. Стабилизируй массив (НОД-версия)","group":"Codeforces - Codeforces Round #731 (Div. 3)","url":"https://codeforces.com/contest/1547/problem/F","interactive":false,"timeLimit":4000,"tests":[{"input":"5\n4\n16 24 10 5\n4\n42 42 42 42\n3\n4 6 4\n5\n1 2 3 4 5\n6\n9 9 27 9 9 63\n","output":"3\n0\n2\n1\n1\n"}],"testType":"multiNumber","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"FStabiliziruiMassivNODVersiya"}}}

use algo_lib::collections::sparse_table_gcd::SparseTableGCD;
use algo_lib::io::output::output;
use algo_lib::io::task_io_settings::TaskIoType;
use algo_lib::io::task_runner::run_task;
use algo_lib::io::{input::Input, task_io_settings::TaskIoSettings};
use algo_lib::misc::binary_search::binary_search_first_true;
#[allow(unused)]
use algo_lib::{dbg, out, out_line};

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.usize();
    let a = input.read_vec::<i32>(n);
    let mut b = a.clone();
    b.extend(&a);
    let gcd_table = SparseTableGCD::new(&b);
    let res = binary_search_first_true(0..n, |steps| -> bool {
        let len = steps + 1;
        let first = gcd_table.query(0..len);
        for pos in 1..n {
            let cur_gcd = gcd_table.query(pos..pos + len);
            if cur_gcd != first {
                return false;
            }
        }
        true
    });
    out_line!(res);
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
