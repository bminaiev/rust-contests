//{"name":"C. Алиса и торт","group":"Codeforces - Codeforces Round #778 (Div. 1 + Div. 2, основан на Финале Технокубка 2022)","url":"http://codeforces.com/contest/1654/problem/C","interactive":false,"timeLimit":2000,"tests":[{"input":"14\n1\n327\n2\n869 541\n2\n985214736 985214737\n3\n2 3 1\n3\n2 3 3\n6\n1 1 1 1 1 1\n6\n100 100 100 100 100 100\n8\n100 100 100 100 100 100 100 100\n8\n2 16 1 8 64 1 4 32\n10\n1 2 4 7 1 1 1 1 7 2\n10\n7 1 1 1 3 1 3 3 2 3\n10\n1 4 4 1 1 1 3 3 3 1\n10\n2 3 2 2 1 2 2 2 2 2\n4\n999999999 999999999 999999999 999999999\n","output":"YES\nNO\nYES\nYES\nNO\nYES\nNO\nYES\nYES\nYES\nYES\nNO\nNO\nYES\n"}],"testType":"multiNumber","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"CAlisaITort"}}}

use std::collections::HashMap;

use algo_lib::io::output::output;
use algo_lib::io::task_io_settings::TaskIoType;
use algo_lib::io::task_runner::run_task;
use algo_lib::io::{input::Input, task_io_settings::TaskIoSettings};
use algo_lib::misc::rec_function::{Callable, RecursiveFunction};
#[allow(unused)]
use algo_lib::{dbg, out, out_line};

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.usize();
    let a = input.vec::<i64>(n);
    let sum: i64 = a.iter().sum();
    let mut map: HashMap<_, usize> = HashMap::new();
    for &val in a.iter() {
        *map.entry(val).or_default() += 1;
    }
    let mut go = RecursiveFunction::new(|f, value: i64| -> bool {
        let cur = *map.get(&value).unwrap_or(&0);
        if cur == 0 {
            if value == 1 {
                return false;
            } else {
                return f.call(value / 2) && f.call((value + 1) / 2);
            }
        } else {
            *map.entry(value).or_default() -= 1;
            return true;
        }
    });
    if go.call(sum) {
        out_line!("YES");
    } else {
        out_line!("NO");
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
