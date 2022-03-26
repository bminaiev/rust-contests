//{"name":"C. Digital Path","group":"Codeforces - The 2019 ICPC Asia Nanjing Regional Contest","url":"https://codeforces.com/gym/103466/problem/C","interactive":false,"timeLimit":1000,"tests":[{"input":"3 5\n1 2 3 8 7\n-1 -1 4 5 6\n1 2 3 8 7\n","output":"4\n"},{"input":"4 4\n1 2 3 4\n2 3 4 3\n3 4 3 2\n4 3 2 1\n","output":"16\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"CDigitalPath"}}}

use std::cmp::min;

use algo_lib::collections::array_2d::Array2D;
use algo_lib::io::output::output;
use algo_lib::io::task_io_settings::TaskIoType;
use algo_lib::io::task_runner::run_task;
use algo_lib::io::{input::Input, task_io_settings::TaskIoSettings};
use algo_lib::iters::shifts::SHIFTS_4;
use algo_lib::iters::shifts_iter::ShiftsIterator;
use algo_lib::math::modulo::Mod7;
#[allow(unused)]
use algo_lib::{dbg, out, out_line};

type Mod = Mod7;

struct Elem {
    value: i32,
    x: usize,
    y: usize,
}

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.usize();
    let m = input.usize();
    let a = input.matrix::<i32>(n, m);
    let mut all = vec![];
    for x in 0..n {
        for y in 0..m {
            all.push(Elem {
                value: a[x][y],
                x,
                y,
            })
        }
    }
    all.sort_by_key(|elem| elem.value);
    let mut res = Mod::ZERO;
    const NEED_LEN: usize = 4;
    let mut dp = vec![Array2D::new(Mod::ZERO, n, m); NEED_LEN + 1];
    let shifts_iter = ShiftsIterator::new(&SHIFTS_4, n, m);
    for elem in all.iter() {
        let x = elem.x;
        let y = elem.y;
        let mut any_smaller = false;
        for (nx, ny) in shifts_iter.iter(x, y) {
            if a[nx][ny] == elem.value - 1 {
                any_smaller = true;
            }
        }
        if !any_smaller {
            dp[1][x][y] = Mod::ONE;
        }
        let mut any_bigger = false;
        for (nx, ny) in shifts_iter.iter(x, y) {
            if a[nx][ny] == elem.value + 1 {
                any_bigger = true;
                for cur_len in 1..=NEED_LEN {
                    let new_len = min(cur_len + 1, NEED_LEN);
                    let cur_val = dp[cur_len][x][y];
                    dp[new_len][nx][ny] += cur_val;
                }
            }
        }
        if !any_bigger {
            res += dp[NEED_LEN][x][y];
        }
    }
    out_line!(res);
}

pub(crate) fn run(mut input: Input) -> bool {
    solve(&mut input, 1);
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
