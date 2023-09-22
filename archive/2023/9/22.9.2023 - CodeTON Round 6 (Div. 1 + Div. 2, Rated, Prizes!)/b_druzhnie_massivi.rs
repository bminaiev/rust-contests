//{"name":"B. Дружные массивы","group":"Codeforces - CodeTON Round 6 (Div. 1 + Div. 2, Rated, Prizes!)","url":"https://codeforces.com/contest/1870/problem/B","interactive":false,"timeLimit":2000,"tests":[{"input":"2\n2 3\n0 1\n1 2 3\n3 1\n1 1 2\n1\n","output":"0 1\n2 3\n"}],"testType":"multiNumber","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"BDruzhnieMassivi"}}}

use algo_lib::io::output::output;
use algo_lib::io::task_io_settings::TaskIoType;
use algo_lib::io::task_runner::run_task;
use algo_lib::io::{input::Input, task_io_settings::TaskIoSettings};
#[allow(unused)]
use algo_lib::{dbg, out, out_line};

const BITS: usize = 30;
fn find_max(a: &[i32], b: &[i32], cnt_ones: &[i32]) -> i32 {
    let n = a.len();
    let mut res = 0;
    for bit in (0..BITS).rev() {
        res |= 1 << bit;

        let mut must_mask = 0;
        let mut must_not_mask = 0;
        let mut ok = true;
        for i in 0..BITS {
            if (res & (1 << i)) != 0 {
                if cnt_ones[i] % 2 == 0 {
                    if n % 2 == 0 {
                        ok = false;
                    } else {
                        must_mask |= 1 << i;
                    }
                } else {
                    if n % 2 == 0 {
                        must_not_mask |= 1 << i;
                    }
                }
            }
        }

        let mut mask = 0;
        for &bi in b.iter() {
            if (bi & must_not_mask) == 0 {
                mask |= bi;
            }
        }
        if must_mask & mask != must_mask {
            ok = false;
        }
        if !ok {
            res ^= 1 << bit;
        }
    }
    res
}

fn find_min(a: &[i32], b: &[i32], cnt_ones: &[i32]) -> i32 {
    let n = a.len();
    let mut res = 0;
    for bit in (0..BITS).rev() {
        let mut must_mask = 0;
        let mut must_not_mask = 0;
        let mut ok = true;
        for i in bit..BITS {
            if (res & (1 << i)) == 0 {
                if cnt_ones[i] % 2 == 1 {
                    if n % 2 == 1 {
                        ok = false;
                    } else {
                        must_mask |= 1 << i;
                    }
                } else {
                    if n % 2 == 1 {
                        must_not_mask |= 1 << i;
                    }
                }
            }
        }

        let mut mask = 0;
        for &bi in b.iter() {
            if (bi & must_not_mask) == 0 {
                mask |= bi;
            }
        }
        if must_mask & mask != must_mask {
            ok = false;
        }
        if !ok {
            res ^= 1 << bit;
        }
    }
    res
}

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.usize();
    let m = input.usize();
    let a = input.vec::<i32>(n);
    let mut cnt_ones = vec![0; BITS];
    for bit in 0..BITS {
        for i in 0..n {
            if (a[i] >> bit) & 1 == 1 {
                cnt_ones[bit] += 1;
            }
        }
    }
    let b = input.vec::<i32>(m);
    let res_max = find_max(&a, &b, &cnt_ones);
    let res_min = find_min(&a, &b, &cnt_ones);
    out_line!(res_min, res_max);
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
