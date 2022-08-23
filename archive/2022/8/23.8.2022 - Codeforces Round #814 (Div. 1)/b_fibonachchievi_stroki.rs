//{"name":"B. Фибоначчиевы строки","group":"Codeforces - Codeforces Round #814 (Div. 1)","url":"https://codeforces.com/contest/1718/problem/B","interactive":false,"timeLimit":2000,"tests":[{"input":"6\n1\n1\n2\n1 1\n2\n1 2\n3\n3 1 3\n2\n7 5\n6\n26 8 3 4 13 34\n","output":"YES\nYES\nNO\nYES\nNO\nYES\n"}],"testType":"multiNumber","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"BFibonachchieviStroki"}}}

use algo_lib::io::output::output;
use algo_lib::io::task_io_settings::TaskIoType;
use algo_lib::io::task_runner::run_task;
use algo_lib::io::{input::Input, task_io_settings::TaskIoSettings};
#[allow(unused)]
use algo_lib::{dbg, out, out_line};

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.usize();
    let a = input.vec::<i64>(n);
    let need_sum = a.iter().sum::<i64>();
    let fibs = gen_fib(need_sum);
    if fibs.iter().sum::<i64>() != need_sum {
        out_line!("NO");
        return;
    }
    let mut ok = false;
    for who_use_first in 0..a.len() {
        let mut used = vec![false; fibs.len()];
        used[0] = true;
        let mut cur_ok = true;
        for i in 0..a.len() {
            let mut cur = if i == who_use_first { a[i] - 1 } else { a[i] };
            let mut prev_used = 0;
            for pos in (1..fibs.len()).rev() {
                if pos + 1 != prev_used && !used[pos] && cur >= fibs[pos] {
                    if i == who_use_first && pos == 1 {
                        continue;
                    }
                    used[pos] = true;
                    prev_used = pos;
                    cur -= fibs[pos];
                }
            }
            if cur != 0 {
                cur_ok = false;
                break;
            }
        }
        if cur_ok {
            ok = true;
            break;
        }
    }

    if ok {
        out_line!("YES");
    } else {
        out_line!("NO");
    }
}

fn gen_fib(need_sum: i64) -> Vec<i64> {
    if need_sum == 1 {
        return vec![1];
    }
    let mut a = vec![1i64, 1i64];
    while a.iter().sum::<i64>() < need_sum {
        let n = a.len();
        let nxt = a[n - 1] + a[n - 2];
        a.push(nxt);
    }
    a
}

fn stress() {
    let mut a = vec![1i64, 1i64];
    while a.iter().sum::<i64>() <= 1e11 as i64 {
        let n = a.len();
        let nxt = a[n - 1] + a[n - 2];
        a.push(nxt);
    }
    dbg!(a);
    dbg!(a.len());
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
    // tester::run_stress(stress);
    // tester::run_single_test("1");
    // tester::run_stress(stress);
}
//END MAIN
