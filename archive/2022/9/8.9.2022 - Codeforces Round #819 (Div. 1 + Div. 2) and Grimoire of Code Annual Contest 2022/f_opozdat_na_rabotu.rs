//{"name":"F. Опоздать на работу","group":"Codeforces - Codeforces Round #819 (Div. 1 + Div. 2) and Grimoire of Code Annual Contest 2022","url":"https://codeforces.com/contest/1726/problem/F","interactive":false,"timeLimit":3000,"tests":[{"input":"5 10\n4 2\n7 3\n3 6\n5 2\n8 0\n1 2 3 4\n","output":"11\n"},{"input":"6 9\n5 3\n5 5\n7 0\n5 8\n7 7\n6 6\n0 0 0 0 0\n","output":"3\n"}],"testType":"multiNumber","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"FOpozdatNaRabotu"}}}

use std::collections::BTreeSet;

use algo_lib::io::output::output;
use algo_lib::io::task_io_settings::TaskIoType;
use algo_lib::io::task_runner::run_task;
use algo_lib::io::{input::Input, task_io_settings::TaskIoSettings};
use algo_lib::misc::gen_vector::gen_vec;
use algo_lib::misc::min_max::UpdateMinMax;
#[allow(unused)]
use algo_lib::{dbg, out, out_line};

#[derive(Clone, Copy, Debug)]
struct Light {
    green_time: i64,
    start: i64,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
struct Segm {
    start: i64,
    id: usize,
    len: i64,
    wait_till: i64,
}

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.usize();
    let cycle = input.i64();
    let mut glob_offset = 0;
    let lights = gen_vec(n, |_| Light {
        green_time: input.read(),
        start: input.read(),
    });
    let dists = input.vec::<i64>(n - 1);
    let mut res = 0;

    let mut set = BTreeSet::new();

    let add_segm_no_cycle = |set: &mut BTreeSet<Segm>, segm: Segm| {
        assert!(segm.len > 0);
        if let Some(&prev) = set.range(..=segm).next_back() {
            if prev.start + prev.len > segm.start {
                set.remove(&prev);
                if prev.start < segm.start {
                    set.insert(Segm {
                        start: prev.start,
                        len: segm.start - prev.start,
                        id: prev.id,
                        wait_till: prev.wait_till,
                    });
                }
                if prev.start + prev.len > segm.start + segm.len {
                    set.insert(Segm {
                        start: segm.start + segm.len,
                        len: prev.start + prev.len - (segm.start + segm.len),
                        id: prev.id,
                        wait_till: prev.wait_till,
                    });
                }
            }
        }
        while let Some(&next) = set.range(segm..).next() {
            if segm.start + segm.len > next.start {
                set.remove(&next);
                if next.start + next.len > segm.start + segm.len {
                    set.insert(Segm {
                        start: segm.start + segm.len,
                        len: next.start + next.len - (segm.start + segm.len),
                        id: next.id,
                        wait_till: next.wait_till,
                    });
                }
            } else {
                break;
            }
        }
        set.insert(segm);
    };

    let add_segm = |set: &mut BTreeSet<Segm>, segm: Segm| {
        if segm.start + segm.len > cycle {
            add_segm_no_cycle(
                set,
                Segm {
                    start: segm.start,
                    len: segm.len,
                    id: segm.id,
                    wait_till: segm.start + segm.len,
                },
            );
            add_segm_no_cycle(
                set,
                Segm {
                    start: 0,
                    len: segm.len - (cycle - segm.start),
                    id: segm.id,
                    wait_till: segm.len - (cycle - segm.start),
                },
            );
        } else {
            add_segm_no_cycle(set, segm);
        }
    };
    {
        let last = lights[n - 1];
        let start = (cycle - last.start + last.green_time) % cycle;
        let len = cycle - last.green_time;
        add_segm(
            &mut set,
            Segm {
                start,
                len,
                id: n - 1,
                wait_till: start + len,
            },
        );
    }

    let mut dp = vec![std::i64::MAX; n];
    dp[n - 1] = 0;

    #[derive(Debug)]
    struct Wait {
        next_id: usize,
        wait: i64,
    }

    let calc_wait_time = |set: &BTreeSet<Segm>, cur_time| -> Option<Wait> {
        let search = Segm {
            start: cur_time,
            id: std::usize::MAX,
            len: 0,
            wait_till: 0,
        };
        if let Some(&prev) = set.range(..=search).next_back() {
            if prev.start + prev.len > cur_time {
                return Some(Wait {
                    next_id: prev.id,
                    wait: prev.wait_till - cur_time,
                });
            }
        }
        None
    };

    for i in (0..n - 1).rev() {
        glob_offset += dists[i];
        res += dists[i];
        let green_time = (cycle - lights[i].start + glob_offset) % cycle;

        if let Some(wait) = calc_wait_time(&set, green_time) {
            dp[i] = dp[wait.next_id] + wait.wait;
        } else {
            dp[i] = 0;
        }

        let start = (green_time + lights[i].green_time) % cycle;
        let len = cycle - lights[i].green_time;
        add_segm(
            &mut set,
            Segm {
                start,
                len,
                id: i,
                wait_till: start + len,
            },
        );
    }

    let mut smallest_additional_wait = std::i64::MAX;
    let mut to_check = vec![0];

    for seg in set.iter() {
        if seg.start > 0 && seg.start - 1 < cycle {
            to_check.push(seg.start - 1);
        }
        if seg.start + seg.len < cycle {
            to_check.push(seg.start + seg.len);
        }
    }

    for &pos in to_check.iter() {
        match calc_wait_time(&set, pos) {
            None => {
                smallest_additional_wait = 0;
            }
            Some(w) => {
                smallest_additional_wait.update_min(dp[w.next_id] + w.wait);
            }
        };
    }
    out_line!(smallest_additional_wait + res);
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
    // tester::run_single_test("2");
    // tester::run_stress(stress);
}
//END MAIN
