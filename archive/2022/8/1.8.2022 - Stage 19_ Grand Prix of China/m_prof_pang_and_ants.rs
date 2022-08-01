//{"name":"M. Prof. Pang and Ants","group":"Yandex - Stage 19: Grand Prix of China","url":"https://official.contest.yandex.com/opencupXXII/contest/39025/problems/M/","interactive":false,"timeLimit":8000,"tests":[{"input":"3\n2 4\n1 2\n3 10\n1 2 3\n5 1\n1 2 3 4 5\n","output":"6\n9\n4\n"}],"testType":"multiNumber","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"MProfPangAndAnts"}}}

use std::cmp::{max, min};
use std::time::Instant;

use algo_lib::collections::array_2d::Array2D;
use algo_lib::io::output::output;
use algo_lib::io::task_io_settings::TaskIoType;
use algo_lib::io::task_runner::run_task;
use algo_lib::io::{input::Input, task_io_settings::TaskIoSettings};
use algo_lib::misc::binary_search::binary_search_first_true;
use algo_lib::misc::rand::Random;
#[allow(unused)]
use algo_lib::{dbg, out, out_line};

fn solve_smart(flow_size: i64, a: &[i64]) -> i64 {
    let glob_max_time = 1000;
    binary_search_first_true(0..glob_max_time, |max_time: i64| {
        let half = max_time / 2;
        let mut all_times = vec![];
        for ai in a.iter() {
            for start in 0..half {
                all_times.push(start + ai + 1);
            }
        }
        all_times.sort();
        all_times.truncate(flow_size as usize);
        if all_times.len() != flow_size as usize {
            return false;
        }
        for i in 0..all_times.len() {
            let cur_sum = all_times[i] + all_times[all_times.len() - 1 - i];
            if cur_sum > max_time {
                return false;
            }
        }
        return true;
    })
}

fn solve_smart3(flow_size: i64, a: &[i64]) -> i64 {
    let glob_max_time = 1000;
    binary_search_first_true(0..glob_max_time, |max_time: i64| {
        let half = max_time / 2;
        let mut all_times_left = vec![];
        let mut all_times_right = vec![];
        for (it, ai) in a.iter().enumerate() {
            let add_left = (max_time % 2) * (it as i64 % 2);
            let add_right = (max_time % 2) * (1 - (it as i64 % 2));
            for start in 0..(half + add_left) {
                all_times_left.push(start + ai + 1);
            }
            for start in 0..half + add_right {
                all_times_right.push(start + ai + 1);
            }
        }
        all_times_left.sort();
        all_times_right.sort();
        all_times_left.truncate(flow_size as usize);
        all_times_right.truncate(flow_size as usize);
        if all_times_left.len() != flow_size as usize || all_times_right.len() != flow_size as usize
        {
            return false;
        }
        for i in 0..all_times_left.len() {
            let cur_sum = all_times_left[i] + all_times_right[all_times_right.len() - 1 - i];
            if cur_sum > max_time {
                return false;
            }
        }
        return true;
    })
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
struct Delta {
    pos: i64,
    delta: i64,
}

fn solve_smart2(flow_size: i64, a: &[i64], glob_max_time: i64) -> i64 {
    let mut a = a.to_vec();
    a.sort();
    let glob_max_time = flow_size * 2 + a[0] * 2 + 3;
    let n = a.len() as i64;

    let mut used_here = Array2D::new(0, a.len(), 2);

    binary_search_first_true(0..glob_max_time, |max_time: i64| {
        let half = max_time / 2;
        let last_ok_value = binary_search_first_true(0..max_time + 1, |check_time| {
            let mut sum = 0;
            for (it, &ai) in a.iter().enumerate() {
                let first_pos_time = 1 + ai;
                let max_cnt = check_time - first_pos_time + 1;
                let cur_half = half + (it as i64 & 1) * (max_time & 1);
                if max_cnt > 0 {
                    sum += min(max_cnt, cur_half);
                }
                if sum >= flow_size {
                    break;
                }
            }
            sum >= flow_size
        });
        if last_ok_value == max_time + 1 {
            return false;
        }
        let mut deltas = vec![];
        let mut more_flow1 = flow_size;
        let mut more_flow2 = flow_size;

        for i in 0..used_here.len() {
            for j in 0..2 {
                used_here[i][j] = 0;
            }
        }

        for (it, &ai) in a.iter().enumerate() {
            let first_pos_time = 1 + ai;
            let max_cnt = last_ok_value - first_pos_time;
            let cur_half = half + (it as i64 & 1) * (max_time & 1);
            let another_half = max_time - cur_half;

            if max_cnt > 0 {
                let max_cnt = if max_cnt < 0 { 0 } else { max_cnt };
                let used1 = min(max_cnt, cur_half);
                let used1 = max(0, used1);
                let used1 = min(used1, more_flow1);
                used_here[it][0] = used1;
                more_flow1 -= used1;

                let used2 = min(max_cnt, another_half);
                let used2 = max(0, used2);
                let used2 = min(used2, more_flow2);
                used_here[it][1] = used2;
                more_flow2 -= used2;
            }
        }

        for (it, &ai) in a.iter().enumerate() {
            let first_pos_time = 1 + ai;
            let max_cnt = last_ok_value - first_pos_time + 1;

            let cur_half = half + (it as i64 & 1) * (max_time & 1);
            let another_half = max_time - cur_half;

            if max_cnt > 0 && max_cnt <= cur_half && more_flow1 > 0 {
                more_flow1 -= 1;
                used_here[it][0] += 1;
            }

            if max_cnt > 0 && max_cnt <= another_half && more_flow2 > 0 {
                more_flow2 -= 1;
                used_here[it][1] += 1;
            }
        }

        for (it, &ai) in a.iter().enumerate() {
            let first_pos_time = 1 + ai;

            let used1 = used_here[it][0];
            let used2 = used_here[it][1];

            if used1 > 0 {
                deltas.push(Delta {
                    delta: 1,
                    pos: first_pos_time,
                });
                deltas.push(Delta {
                    delta: -1,
                    pos: first_pos_time + used1,
                });
            }

            if used2 > 0 {
                let rev_pos = max_time + 1 - first_pos_time - used2;
                deltas.push(Delta {
                    delta: -1,
                    pos: rev_pos,
                });
                deltas.push(Delta {
                    delta: 1,
                    pos: rev_pos + used2,
                });
            }
        }

        if more_flow1 > 0 || more_flow2 > 0 {
            return false;
        }
        deltas.sort_unstable();

        let mut cur_value = 0i64;
        let mut cur_coef = 0i64;
        let mut cur_time = 0i64;
        for d in deltas.iter() {
            cur_value += cur_coef * (d.pos - cur_time);
            cur_time = d.pos;
            cur_coef += d.delta;
            if cur_value < 0 {
                return false;
            }
        }
        assert!(cur_coef == 0);
        return true;
    })
}

fn can_this_time(flow_size: i64, a: &[i64], rnd: &mut Random, max_time: i64) -> bool {
    let mut left = vec![];
    let mut right = vec![];
    for &ai in a.iter() {
        let left_part = rnd.gen(0..max_time + 1);
        let right_part = max_time - left_part;
        for i in 0..left_part {
            left.push(ai + 1 + i);
        }
        for i in 0..right_part {
            right.push(ai + 1 + i);
        }
    }
    left.sort();
    right.sort();

    let flow_size = flow_size as usize;
    if left.len() < flow_size || right.len() < flow_size {
        return false;
    }
    left.truncate(flow_size);
    right.truncate(flow_size);
    for i in 0..flow_size {
        if left[i] + right[flow_size - 1 - i] > max_time {
            return false;
        }
    }
    true
}

const glob_max_time_1: i64 = 1e15 as i64;
const glob_max_time_2: i64 = 2e14 as i64;
const glob_max_time_3: i64 = 1e18 as i64;

fn stress() {
    for it in 208.. {
        dbg!(it);
        let mut rnd = Random::new(it);
        let flow_size = rnd.gen(1..20);
        let n = rnd.gen(1..10);
        let a = rnd.gen_vec(n, 1..10);
        // let smart = solve_smart3(flow_size, &a);
        let smart = solve_smart2(flow_size, &a, glob_max_time_1);
        // let smart = smart2;
        for _ in 0..1000 {
            if (can_this_time(flow_size, &a, &mut rnd, smart - 1)) {
                dbg!(a, flow_size, smart - 1);
                assert!(false);
            }
        }
        for _ in 0.. {
            if can_this_time(flow_size, &a, &mut rnd, smart) {
                break;
            }
        }
        // if smart != smart2 {
        //     dbg!("ooops");
        //     dbg!(flow_size);
        //     dbg!(a);
        // }
        // assert_eq!(smart, smart2);
    }
}

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.usize();
    let flow_size = input.i64();
    assert!(flow_size <= 1e14 as i64);
    let a = input.vec::<i64>(n);
    for &x in a.iter() {
        assert!(x <= 1e9 as i64);
    }
    // let start = Instant::now();
    let res = solve_smart2(flow_size, &a, glob_max_time_2);
    // let res2 = solve_smart2(flow_size, &a, glob_max_time_3);
    // assert!(res == res2);
    // if start.elapsed().as_millis() < 5000 {
    //     let res3 = solve_smart2(flow_size, &a, glob_max_time_3);
    //     assert_eq!(res, res3);
    // }
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
    // tester::run_stress(stress);
    // tester::run_single_test("2");
    // tester::run_stress(stress);
}
//END MAIN
