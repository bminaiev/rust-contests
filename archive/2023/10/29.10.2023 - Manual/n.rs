//{"name":"n","group":"Manual","url":"","interactive":false,"timeLimit":2000,"tests":[{"input":"","output":""}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"n"}}}

use algo_lib::collections::array_2d::Array2D;
use algo_lib::io::output::output;
use algo_lib::io::task_io_settings::TaskIoType;
use algo_lib::io::task_runner::run_task;
use algo_lib::io::{input::Input, task_io_settings::TaskIoSettings};
use algo_lib::misc::rand::Random;
#[allow(unused)]
use algo_lib::{dbg, out, out_line};

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
struct State {
    used_days: i64,
    inside_day: i64,
}

fn solve_fast(a: &[i64], one_try: i64, one_day: i64) -> i64 {
    let n = a.len();
    let mut dp = Array2D::new(
        State {
            used_days: i64::MAX / 2,
            inside_day: 0,
        },
        4,
        1,
    );
    dp[0][0] = State {
        used_days: 1,
        inside_day: 0,
    };
    for len in (1..=n).rev() {
        let mut ndp = Array2D::new(
            State {
                used_days: i64::MAX / 2,
                inside_day: 0,
            },
            4,
            dp[0].len() + 1,
        );
        for l in 0..dp[0].len() {
            let r = l + len - 1;
            for mask in 0..4 {
                // dbg!(mask, l, r, dp[mask][l]);
                for i in 0..2 {
                    let pos = if i == 0 { l } else { r };
                    let need_time = a[pos].abs();

                    let mut nmask = mask;
                    let mut new_state = dp[mask][l];

                    if ((1 << i) & mask) == 0 {
                        if new_state.inside_day + need_time <= one_day {
                            new_state.inside_day += need_time;
                            nmask |= 1 << i;
                        } else {
                            new_state.inside_day = need_time;
                            new_state.used_days += 1;
                            nmask = 1 << i;
                        }
                    }

                    if new_state.inside_day + one_try <= one_day {
                        new_state.inside_day += one_try;
                    } else {
                        new_state.used_days += 1;
                        new_state.inside_day = one_try + need_time;
                        nmask = 1 << i;
                    }

                    if i == 0 {
                        if a[pos] <= 0 {
                            if ndp[nmask][l + 1] > new_state {
                                ndp[nmask][l + 1] = new_state;
                            }
                        }
                    } else if ndp[nmask][l] > new_state {
                        if a[pos] >= 0 {
                            ndp[nmask][l] = new_state;
                        }
                    }
                }
            }
        }
        dp = ndp;
    }
    let mut res = State {
        used_days: i64::MAX / 2,
        inside_day: 0,
    };
    for mask in 0..4 {
        for l in 0..dp[0].len() {
            res = res.min(dp[mask][l]);
        }
    }
    res.used_days
}

fn solve_slow(a: &[i64], one_try: i64, one_day: i64) -> i64 {
    let n = a.len();
    let mut dp = vec![i64::MAX; 1 << n];
    dp[0] = 0;
    for mask in 0..(1 << n) {
        for more_mask in 0usize..(1 << n) {
            let next_mask = mask | more_mask;
            let mut max_a = 0;
            let mut min_a = 0;
            for v in 0..n {
                if ((1 << v) & more_mask) != 0 {
                    max_a = max_a.max(a[v]);
                    min_a = min_a.min(a[v]);
                }
            }
            let mut cost = (more_mask).count_ones() as i64 * one_try;
            cost += max_a;
            cost += min_a.abs();
            if cost <= one_day {
                dp[next_mask] = dp[next_mask].min(dp[mask] + 1);
            }
        }
    }
    dp[dp.len() - 1]
}

fn stress() {
    for t in 23.. {
        dbg!(t);
        let mut rnd = Random::new(t);
        let n = rnd.gen(1..10);
        let max_sz = rnd.gen(1..10);
        let mut a: Vec<i64> = vec![];
        for _ in 0..n {
            a.push(rnd.gen(-max_sz..max_sz));
        }
        a.sort();
        let one_try = rnd.gen(1..10);
        let mut one_day = rnd.gen(1..20);
        for &x in a.iter() {
            one_day = one_day.max(x.abs() + one_try);
        }
        let my = solve_fast(&a, one_try, one_day);
        let slow = solve_slow(&a, one_try, one_day);
        if my != slow {
            dbg!(my, slow, a, one_try, one_day);
        }
        assert_eq!(my, slow);
    }
}

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.usize();
    let one_try = input.i64();
    let one_day = input.i64();
    let mut a = input.vec::<i64>(n);
    a.sort();
    for x in a.iter_mut() {
        *x *= 2;
    }
    let res = solve_fast(&a, one_try, one_day);

    out_line!(res);
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
