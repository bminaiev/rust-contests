//{"name":"i","group":"Manual","url":"","interactive":false,"timeLimit":2000,"tests":[{"input":"","output":""}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"i"}}}

use algo_lib::io::output::output;
use algo_lib::io::task_io_settings::TaskIoType;
use algo_lib::io::task_runner::run_task;
use algo_lib::io::{input::Input, task_io_settings::TaskIoSettings};
#[allow(unused)]
use algo_lib::{dbg, out, out_line};

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.usize();
    let mut a = input.vec::<i32>(n);
    const BITS: usize = 30;
    let mut must_zero = vec![0; BITS];
    let mut must_one = vec![0; BITS];
    let change_pair = |x: i32, y: i32, delta: i32, must_zero: &mut [i32], must_one: &mut [i32]| {
        for bit in (0..BITS).rev() {
            let set1 = ((1 << bit) & x) != 0;
            let set2 = ((1 << bit) & y) != 0;
            if !set1 && set2 {
                must_zero[bit] += delta;
                return;
            } else if set1 && !set2 {
                must_one[bit] += delta;
                return;
            }
        }
    };
    let print_ans = |must_zero: &[i32], must_one: &[i32]| {
        let mut res = 0;
        for bit in (0..BITS).rev() {
            if must_zero[bit] > 0 && must_one[bit] > 0 {
                res = -1;
                break;
            }
            if must_one[bit] > 0 {
                res |= 1 << bit;
            }
        }
        out_line!(res);
    };
    for w in a.windows(2) {
        change_pair(w[0], w[1], 1, &mut must_zero, &mut must_one);
    }
    print_ans(&must_zero, &must_one);
    let q = input.usize();
    for _ in 0..q {
        let p = input.usize() - 1;
        let val = input.i32();
        if p > 0 {
            change_pair(a[p - 1], a[p], -1, &mut must_zero, &mut must_one);
        }
        if p + 1 < n {
            change_pair(a[p], a[p + 1], -1, &mut must_zero, &mut must_one);
        }
        a[p] = val;
        if p > 0 {
            change_pair(a[p - 1], a[p], 1, &mut must_zero, &mut must_one);
        }
        if p + 1 < n {
            change_pair(a[p], a[p + 1], 1, &mut must_zero, &mut must_one);
        }
        print_ans(&must_zero, &must_one);
    }
}

pub(crate) fn run(mut input: Input) -> bool {
    solve(&mut input, 1);
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
