//{"name":"E. Iva & Pav","group":"Codeforces - Codeforces Round 900 (Div. 3)","url":"https://codeforces.com/contest/1878/problem/E","interactive":false,"timeLimit":5000,"tests":[{"input":"3\n5\n15 14 17 42 34\n3\n1 7\n2 15\n4 5\n5\n7 5 3 1 7\n4\n1 7\n5 7\n2 3\n2 2\n7\n19 20 15 12 21 7 11\n4\n1 15\n4 4\n7 12\n5 7\n","output":"2 -1 5 1 5 2 2 2 6 -1 5\n"}],"testType":"multiNumber","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"EIvaPav"}}}

use algo_lib::collections::sparse_table::{SparseTable, SparseTableSpec};
use algo_lib::collections::sparse_table_min::SparseTableMin;
use algo_lib::io::output::output;
use algo_lib::io::task_io_settings::TaskIoType;
use algo_lib::io::task_runner::run_task;
use algo_lib::io::{input::Input, task_io_settings::TaskIoSettings};
use algo_lib::misc::binary_search::binary_search_last_true;
use algo_lib::misc::gen_vector::gen_vec;
#[allow(unused)]
use algo_lib::{dbg, out, out_line};

struct Node {
    value: i32,
}

impl SparseTableSpec for Node {
    type Element = i32;

    type Result = i32;

    fn convert(pos: usize, elem: &Self::Element) -> Self::Result {
        *elem
    }

    fn join(lhs: &Self::Result, rhs: &Self::Result, elements: &[Self::Element]) -> Self::Result {
        *lhs & *rhs
    }
}

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.usize();
    let a = input.vec::<i32>(n);
    let st = SparseTable::<Node>::new(&a);
    let q = input.usize();
    for _ in 0..q {
        let l = input.usize() - 1;
        let k = input.i32();
        let r = binary_search_last_true(l + 1..n + 1, |r| st.query(l..r) >= k);
        if let Some(r) = r {
            out_line!(r);
        } else {
            out_line!(-1);
        }
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
