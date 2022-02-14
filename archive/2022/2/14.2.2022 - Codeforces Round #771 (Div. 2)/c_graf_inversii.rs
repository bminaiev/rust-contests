//{"name":"C. Граф инверсий","group":"Codeforces - Codeforces Round #771 (Div. 2)","url":"https://codeforces.com/contest/1638/problem/C","interactive":false,"timeLimit":1000,"tests":[{"input":"6\n3\n1 2 3\n5\n2 1 4 3 5\n6\n6 1 4 2 5 3\n1\n1\n6\n3 2 1 6 5 4\n5\n3 1 5 2 4\n","output":"3\n3\n1\n1\n2\n1\n"}],"testType":"multiNumber","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"CGrafInversii"}}}

use std::collections::BTreeSet;

use algo_lib::collections::last_exn::LastExn;
use algo_lib::graph::dsu::Dsu;
use algo_lib::io::output::output;
use algo_lib::io::task_io_settings::TaskIoType;
use algo_lib::io::task_runner::run_task;
use algo_lib::io::{input::Input, task_io_settings::TaskIoSettings};
use algo_lib::misc::min_max::UpdateMinMax;
use algo_lib::misc::vec_apply_delta::ApplyDelta;
#[allow(unused)]
use algo_lib::{dbg, out, out_line};

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.usize();
    let p = input.read_vec::<usize>(n).sub_from_all(1);
    let mut dsu = Dsu::new(n);

    let mut uniq = BTreeSet::new();

    for &val in p.iter() {
        let mut to_add = val;
        while !uniq.is_empty() && *uniq.last_exn() > val {
            let last = *uniq.last_exn();
            uniq.remove(&last);
            to_add.update_max(last);
            dsu.unite(last, val);
        }
        uniq.insert(to_add);
    }

    out_line!(dsu.num_components());
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
