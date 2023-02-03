//{"name":"D. Много точных квадратов","group":"Codeforces - Codeforces Round #844 (Div. 1 + Div. 2, основан на Отборочном раунде VK Cup 2022)","url":"https://codeforces.com/contest/1782/problem/D","interactive":false,"timeLimit":4000,"tests":[{"input":"4\n5\n1 2 3 4 5\n5\n1 6 13 22 97\n1\n100\n5\n2 5 10 17 26\n","output":"2\n5\n1\n2\n"}],"testType":"multiNumber","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"DMnogoTochnikhKvadratov"}}}

use algo_lib::io::output::output;
use algo_lib::io::task_io_settings::TaskIoType;
use algo_lib::io::task_runner::run_task;
use algo_lib::io::{input::Input, task_io_settings::TaskIoSettings};
use algo_lib::misc::min_max::UpdateMinMax;
#[allow(unused)]
use algo_lib::{dbg, out, out_line};

fn is_square(x: i64) -> bool {
    let mut sq = (x as f64).sqrt() as i64;
    sq -= 2;
    while sq * sq < x {
        sq += 1;
    }
    sq * sq == x
}

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.usize();
    let a = input.vec::<i64>(n);
    let mut res = 1;
    for first in 0..n {
        for second in first + 1..n {
            let diff = a[second] - a[first];
            for s in 1..=diff {
                if s * s > diff {
                    break;
                }
                if diff % s == 0 {
                    let s2 = diff / s;
                    if s2 >= s && (s2 - s) % 2 == 0 {
                        let y = (s2 - s) / 2;
                        let y2 = y * y;
                        if y2 >= a[first] {
                            let x = y2 - a[first];
                            assert!(is_square(a[second] + x));
                            let mut cur_res = 2;
                            for i in second + 1..n {
                                if is_square(a[i] + x) {
                                    cur_res += 1;
                                }
                            }
                            res.update_max(cur_res);
                        }
                    }
                }
            }
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
