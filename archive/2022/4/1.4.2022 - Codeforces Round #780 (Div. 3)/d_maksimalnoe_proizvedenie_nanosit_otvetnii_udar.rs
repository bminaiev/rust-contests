//{"name":"D. Максимальное произведение наносит ответный удар","group":"Codeforces - Codeforces Round #780 (Div. 3)","url":"https://codeforces.com/contest/1660/problem/D","interactive":false,"timeLimit":2000,"tests":[{"input":"5\n4\n1 2 -1 2\n3\n1 1 -2\n5\n2 0 -2 2 -1\n3\n-2 -1 -1\n3\n-1 -2 -2\n","output":"0 2\n3 0\n2 0\n0 1\n1 0\n"}],"testType":"multiNumber","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"DMaksimalnoeProizvedenieNanositOtvetniiUdar"}}}

use algo_lib::io::output::output;
use algo_lib::io::task_io_settings::TaskIoType;
use algo_lib::io::task_runner::run_task;
use algo_lib::io::{input::Input, task_io_settings::TaskIoSettings};
use algo_lib::misc::min_max::UpdateMinMax;
#[allow(unused)]
use algo_lib::{dbg, out, out_line};

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.usize();
    let b = input.vec::<i32>(n);

    let mut res = (0, n, 0);

    let mut start = 0;
    while start != b.len() {
        if b[start] == 0 {
            start += 1;
            continue;
        }
        let mut it = start;
        while it != b.len() && b[it] != 0 {
            it += 1;
        }

        let a = &b[start..it];

        let mut sign = 1;
        let mut tot2 = 0;
        for &x in a.iter() {
            if x < 0 {
                sign *= -1;
            }
            if x.abs() == 2 {
                tot2 += 1;
            }
        }
        if sign == 1 {
            res.update_max((tot2, start, n - it));
        } else {
            {
                let mut cnt2 = 0;
                for i in 0..a.len() {
                    if a[i].abs() == 2 {
                        cnt2 += 1;
                    }
                    if a[i] < 0 {
                        res.update_max((tot2 - cnt2, i + 1 + start, n - it));
                        break;
                    }
                }
            }
            {
                let mut cnt2 = 0;
                for i in (0..a.len()).rev() {
                    if a[i].abs() == 2 {
                        cnt2 += 1;
                    }
                    if a[i] < 0 {
                        res.update_max((tot2 - cnt2, start, n - it + (a.len() - i)));
                        break;
                    }
                }
            }
        }

        start = it;
    }
    let (_, b, c) = res;
    out_line!(b, c);
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
