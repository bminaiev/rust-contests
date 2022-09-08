//{"name":"C. Правильная скобочная последовательность Jatayu","group":"Codeforces - Codeforces Round #819 (Div. 1 + Div. 2) and Grimoire of Code Annual Contest 2022","url":"https://codeforces.com/contest/1726/problem/C","interactive":false,"timeLimit":2000,"tests":[{"input":"4\n1\n()\n3\n()(())\n3\n((()))\n4\n(())(())\n","output":"1\n2\n3\n3\n"}],"testType":"multiNumber","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"CPravilnayaSkobochnayaPosledovatelnostJatayu"}}}

use std::collections::BTreeMap;

use algo_lib::graph::dsu::Dsu;
use algo_lib::io::output::output;
use algo_lib::io::task_io_settings::TaskIoType;
use algo_lib::io::task_runner::run_task;
use algo_lib::io::{input::Input, task_io_settings::TaskIoSettings};
#[allow(unused)]
use algo_lib::{dbg, out, out_line};

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.usize() * 2;
    let s = input.string();
    let mut balance = 0;
    let mut first = BTreeMap::new();
    let mut stack = vec![];
    let mut dsu = Dsu::new(n);
    for i in 0..n {
        if s[i] == b'(' {
            if !first.contains_key(&balance) {
                first.insert(balance, i);
            }
            balance += 1;
            stack.push(i);
        } else {
            first.remove(&balance);
            balance -= 1;
            let prev = stack.pop().unwrap();
            dsu.unite(prev, i);
            let same_balance = *first.get(&balance).unwrap();
            dsu.unite(prev, same_balance);
        }
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
    // tester::run_stress(stress);
}
//END MAIN
