//{"name":"G. Посчитай поезда","group":"Codeforces - Codeforces Round #797 (Div. 3)","url":"https://codeforces.com/contest/1690/problem/G","interactive":false,"timeLimit":2000,"tests":[{"input":"3\n\n4 2\n6 2 3 7\n3 2\n4 7\n\n5 4\n10 13 5 2 6\n2 4\n5 2\n1 5\n3 2\n\n13 4\n769 514 336 173 181 373 519 338 985 709 729 702 168\n12 581\n6 222\n7 233\n5 117\n","output":"3 4\n4 4 2 3\n5 6 6 5\n"}],"testType":"multiNumber","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"GPoschitaiPoezda"}}}

use std::cmp::min;

use algo_lib::io::output::output;
use algo_lib::io::task_io_settings::TaskIoType;
use algo_lib::io::task_runner::run_task;
use algo_lib::io::{input::Input, task_io_settings::TaskIoSettings};
use algo_lib::misc::binary_search::binary_search_first_true;
#[allow(unused)]
use algo_lib::{dbg, out, out_line};

#[target_feature(enable = "avx2")]
unsafe fn do_fast(
    a: &mut [i32],
    real_speed: &mut [i32],
    pos: usize,
    sub: i32,
    starts: &mut [i32],
) -> i32 {
    a[pos] -= sub;
    let new_val = a[pos];
    let first_smaller =
        binary_search_first_true(pos..a.len(), |check_pos| real_speed[check_pos] < new_val);
    for real in real_speed[pos..first_smaller].iter_mut() {
        *real = new_val;
    }
    for start in starts[pos..first_smaller].iter_mut() {
        *start = 0;
    }
    for &check_pos in [pos, first_smaller].iter() {
        if check_pos > 0 && check_pos < real_speed.len() {
            starts[check_pos] = (real_speed[check_pos - 1] > real_speed[check_pos]) as i32;
        }
    }
    starts.iter().sum::<i32>()
}

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.usize();
    let m = input.usize();
    let mut a = input.vec::<i32>(n);
    let mut real_speed = vec![];
    let mut smallest = a[0];
    for i in 0..n {
        smallest = min(smallest, a[i]);
        real_speed.push(smallest);
    }
    let mut starts: Vec<_> = real_speed
        .windows(2)
        .map(|w| if w[0] > w[1] { 1 } else { 0 })
        .collect();
    starts.insert(0, 0);
    for _ in 0..m {
        let pos = input.usize() - 1;
        let sub = input.i32();

        let res = unsafe { do_fast(&mut a, &mut real_speed, pos, sub, &mut starts) };
        out!(res + 1, "");
    }
    out_line!()
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
