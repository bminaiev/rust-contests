//{"name":"i","group":"Manual","url":"","interactive":false,"timeLimit":2000,"tests":[{"input":"","output":""}],"testType":"multiNumber","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"i"}}}

use algo_lib::io::output::output;
use algo_lib::io::task_io_settings::TaskIoType;
use algo_lib::io::task_runner::run_task;
use algo_lib::io::{input::Input, task_io_settings::TaskIoSettings};
#[allow(unused)]
use algo_lib::{dbg, out, out_line};

fn doit(vals: &[i32], res: &[bool]) -> Vec<i32> {
    let mut a = vec![];
    for i in 0..vals.len() {
        a.push(vals[i]);
        let is_max = res[i];
        let mut pos = i;
        while pos != 0 {
            let next_pos = (pos - 1) / 2;
            if (is_max && a[next_pos] < a[pos]) || (!is_max && a[next_pos] > a[pos]) {
                a.swap(next_pos, pos);
            } else {
                break;
            }
            pos = next_pos;
        }
    }
    a
}

fn solve_case(vals: &[i32], final_vals: &[i32]) -> Option<Vec<bool>> {
    let mut cur = final_vals.to_vec();
    let mut res = vec![];
    for i in (0..cur.len()).rev() {
        let mut pos = i;
        let mut is_max = false;
        let mut is_min = false;
        let now_val = vals[i];
        let mut to_shift = vec![];
        loop {
            to_shift.push(pos);
            if pos == 0 {
                break;
            }
            if cur[pos] == now_val {
                let next_pos = (pos - 1) / 2;
                if cur[next_pos] > now_val {
                    is_max = true;
                } else if cur[next_pos] < now_val {
                    is_min = true;
                }
                break;
            } else if cur[pos] > now_val {
                is_min = true;
            } else {
                is_max = true;
            }
            pos = (pos - 1) / 2;
        }
        to_shift.reverse();
        let last_val = cur[to_shift[0]];
        for i in 0..to_shift.len() - 1 {
            cur[to_shift[i]] = cur[to_shift[i + 1]];
        }
        cur[to_shift[to_shift.len() - 1]] = last_val;
        if is_max && is_min {
            return None;
        }
        if is_max {
            res.push(true);
        } else {
            res.push(false);
        }
    }
    res.reverse();
    let my_final_vals = doit(vals, &res);
    if my_final_vals != final_vals {
        return None;
    }
    Some(res)
}

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.usize();
    let vals = input.vec::<i32>(n);
    let final_vals = input.vec::<i32>(n);
    if let Some(res) = solve_case(&vals, &final_vals) {
        for &x in res.iter() {
            if x {
                out!("1");
            } else {
                out!("0");
            }
        }
        out_line!();
    } else {
        out_line!("Impossible");
    }
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
