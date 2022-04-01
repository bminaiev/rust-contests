//{"name":"F1. Перспективные подстроки (простая версия)","group":"Codeforces - Codeforces Round #780 (Div. 3)","url":"https://codeforces.com/contest/1660/problem/F1","interactive":false,"timeLimit":3000,"tests":[{"input":"5\n3\n+-+\n5\n-+---\n4\n----\n7\n--+---+\n6\n+++---\n","output":"2\n4\n2\n7\n4\n"}],"testType":"multiNumber","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"F1PerspektivniePodstrokiProstayaVersiya"}}}

use algo_lib::io::output::output;
use algo_lib::io::task_io_settings::TaskIoType;
use algo_lib::io::task_runner::run_task;
use algo_lib::io::{input::Input, task_io_settings::TaskIoSettings};
use algo_lib::seg_trees::fenwick::Fenwick;
#[allow(unused)]
use algo_lib::{dbg, out, out_line};

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.usize();
    let s = input.string();
    let mut res = 0;
    let mut balance = n + 1;
    let mut fenw = vec![Fenwick::<i64>::new(n * 2 + 3); 3];
    const C: usize = 3;
    fenw[balance % C].add(balance, 1);
    for &x in s.iter() {
        if x == b'+' {
            balance += 1;
        } else {
            balance -= 1;
        }
        res += fenw[balance % C].get_suffix_sum(balance);
        fenw[balance % C].add(balance, 1);
    }
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
