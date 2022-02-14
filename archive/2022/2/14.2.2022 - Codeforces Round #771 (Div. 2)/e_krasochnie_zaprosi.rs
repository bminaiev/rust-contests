//{"name":"E. Красочные запросы","group":"Codeforces - Codeforces Round #771 (Div. 2)","url":"https://codeforces.com/contest/1638/problem/E","interactive":false,"timeLimit":4000,"tests":[{"input":"5 8\nColor 2 4 2\nAdd 2 2\nQuery 3\nColor 4 5 3\nColor 2 2 3\nAdd 3 3\nQuery 2\nQuery 5\n","output":"2\n5\n3\n"},{"input":"2 7\nAdd 1 7\nQuery 1\nAdd 2 4\nQuery 2\nColor 1 1 1\nAdd 1 1\nQuery 2\n","output":"7\n7\n8\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"EKrasochnieZaprosi"}}}

use std::collections::BTreeSet;

use algo_lib::collections::last_exn::LastExn;
use algo_lib::io::output::output;
use algo_lib::io::task_io_settings::TaskIoType;
use algo_lib::io::task_runner::run_task;
use algo_lib::io::{input::Input, task_io_settings::TaskIoSettings};
use algo_lib::misc::binary_search::binary_search_last_true;
use algo_lib::seg_trees::fenwick::Fenwick;
#[allow(unused)]
use algo_lib::{dbg, out, out_line};

#[derive(Clone, Copy)]
struct Change {
    time: usize,
    pref_sum: i64,
}

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.usize();
    let q = input.usize();

    let mut fenw_prifix = Fenwick::new(n + 1);

    let mut segment_starts: BTreeSet<usize> = BTreeSet::new();
    segment_starts.insert(0);
    let mut colors = vec![0; n];
    let mut start_time = vec![0; n];

    let mut changes_by_color = vec![
        vec![Change {
            time: 0,
            pref_sum: 0
        }];
        n
    ];

    for time in 1..=q {
        let q_type = input.string()[0];
        if q_type == b'C' {
            // color
            let left = input.usize() - 1;
            let right = input.usize();
            let new_color = input.usize() - 1;
            for &split in [left, right].iter() {
                if split == n {
                    continue;
                }
                let start_pos = *segment_starts.range(..=split).next_back().unwrap();
                segment_starts.insert(split);
                colors[split] = colors[start_pos];
                start_time[split] = start_time[start_pos];
            }
            {
                let mut mid_pos = vec![];
                while let Some(&next) = segment_starts.range(left..right).next() {
                    mid_pos.push(next);
                    segment_starts.remove(&next);
                }
                mid_pos.push(right);
                for w in mid_pos.windows(2) {
                    let my_color = colors[w[0]];
                    let from_time = start_time[w[0]];

                    let changes = &changes_by_color[my_color];
                    let good_pos = binary_search_last_true(0..changes.len(), |pos| {
                        changes[pos].time <= from_time
                    })
                    .unwrap();
                    let delta = changes.last_exn().pref_sum - changes[good_pos].pref_sum;

                    fenw_prifix.add(w[0], delta);
                    fenw_prifix.add(w[1], -delta);
                }
            }

            segment_starts.insert(left);
            colors[left] = new_color;
            start_time[left] = time;
        } else if q_type == b'A' {
            // add
            let color = input.usize() - 1;
            let delta = input.i64();
            let cur_sum = changes_by_color[color].last_exn().pref_sum;
            changes_by_color[color].push(Change {
                time,
                pref_sum: cur_sum + delta,
            });
        } else if q_type == b'Q' {
            // query
            let pos = input.usize() - 1;
            let res = fenw_prifix.get_sum(pos);
            let start_pos = *segment_starts.range(..=pos).next_back().unwrap();
            let from_time = start_time[start_pos];
            let my_color = colors[start_pos];
            let changes = &changes_by_color[my_color];
            let good_pos =
                binary_search_last_true(0..changes.len(), |pos| changes[pos].time <= from_time)
                    .unwrap();
            let delta = changes.last_exn().pref_sum - changes[good_pos].pref_sum;
            out_line!(res + delta);
        } else {
            unreachable!();
        }
    }
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
