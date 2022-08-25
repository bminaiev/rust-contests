//{"name":"I. Improvement Through Crisis","group":"Yandex - Day 1","url":"https://official.contest.yandex.com/ptz-summer-2022/contest/39546/problems/I/","interactive":false,"timeLimit":7000,"tests":[{"input":"3 5\n17 5\n5 2\n15 4\n","output":"3\n"},{"input":"2 1345\n1344 1\n10 10\n","output":"0\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"IImprovementThroughCrisis"}}}

use std::cmp::min;

use algo_lib::collections::multiset::MultiSet;
use algo_lib::io::output::output;
use algo_lib::io::task_io_settings::TaskIoType;
use algo_lib::io::task_runner::run_task;
use algo_lib::io::{input::Input, task_io_settings::TaskIoSettings};
use algo_lib::misc::binary_search::binary_search_first_true;
#[allow(unused)]
use algo_lib::{dbg, out, out_line};

#[derive(Clone, Copy)]
struct Research {
    need_time: i64,
    sub: i64,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct Elem {
    sub: i64,
    last_row: bool,
}

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.usize();
    let max_time = input.i64();
    let mut researches = vec![];
    let mut sum = 0;
    for _ in 0..n {
        let need_time = input.i64();
        let sub = input.i64();
        sum += need_time;
        researches.push(Research { need_time, sub });
    }
    researches.sort_by_key(|r| -r.sub);
    const MAX_BS: i64 = 1.1e18 as i64;
    let res = binary_search_first_true(1..MAX_BS, |iters| {
        let mut ms: MultiSet<Elem> = MultiSet::new();
        let mut cur_sum = sum;
        let mut left_last_level = max_time;
        let mut left_total = if MAX_BS / max_time < iters {
            MAX_BS
        } else {
            iters * max_time
        };
        for r in researches.iter() {
            loop {
                if left_total == 0 {
                    break;
                }
                let best_ms = *ms.last().unwrap_or(&Elem {
                    sub: 0,
                    last_row: false,
                });

                if best_ms.sub > r.sub {
                    ms.remove(&best_ms);
                    if best_ms.last_row {
                        if left_last_level == 0 {
                            continue;
                        }
                        left_last_level -= 1;
                    }
                    cur_sum -= best_ms.sub;
                    left_total -= 1;

                    continue;
                }

                let max_full = r.need_time / r.sub;
                if max_full >= iters {
                    let used_here = min(left_total, iters);
                    cur_sum -= used_here * r.sub;
                    left_total -= used_here;
                    if used_here == iters {
                        left_last_level -= 1;
                    }
                } else {
                    let used_here = min(max_full, left_total);
                    cur_sum -= used_here * r.sub;
                    left_total -= used_here;
                    let left_here = r.need_time - used_here * r.sub;
                    if left_here != 0 {
                        let last_row = max_full == iters - 1;
                        ms.insert(Elem {
                            sub: left_here,
                            last_row,
                        });
                    }
                }
                break;
            }
        }
        while !ms.is_empty() {
            if left_total == 0 {
                break;
            }
            let best_ms = *ms.last().unwrap();

            ms.remove(&best_ms);
            if best_ms.last_row {
                if left_last_level == 0 {
                    continue;
                }
                left_last_level -= 1;
            }
            cur_sum -= best_ms.sub;
            left_total -= 1;

            continue;
        }
        assert!(left_total >= 0);
        assert!(left_last_level >= 0);
        cur_sum <= min(left_total, left_last_level)
    });
    out_line!(res - 1);
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
    // tester::run_stress(stress);
}
//END MAIN
