//{"name":"nerc_even","group":"Manual","url":"","interactive":false,"timeLimit":2000,"tests":[{"input":"","output":""},{"input":"","output":""}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"nerc_even"}}}

use algo_lib::collections::last_exn::LastExn;
use algo_lib::io::output::output;
use algo_lib::io::task_io_settings::TaskIoType;
use algo_lib::io::task_runner::run_task;
use algo_lib::io::{input::Input, task_io_settings::TaskIoSettings};
use algo_lib::misc::binary_search::binary_search_first_true;
use algo_lib::misc::min_max::UpdateMinMax;
use algo_lib::misc::pref_sum::PrefSum;
#[allow(unused)]
use algo_lib::{dbg, out, out_line};

#[derive(PartialEq, Eq)]
enum Result {
    TooShort,
    TooLong,
    OkLength(Vec<i32>),
}

fn solve(input: &mut Input, _test_case: usize) {
    let l = input.i32();
    let n = input.usize();
    let mut a = input.vec::<i32>(n);
    a.push(l);
    let build_answer = |min_len: i32, max_len: i32| -> Result {
        let mut possible_end = vec![(0..0)];
        for i in 0..n {
            let cur = possible_end.last_exn().clone();
            let mut next = cur.start + min_len..cur.end + max_len;
            if next.end < a[i] {
                return Result::TooShort;
            }
            if next.start > a[i + 1] {
                return Result::TooLong;
            }
            next.start.update_max(a[i]);
            next.end.update_min(a[i + 1]);
            assert!(next.start <= next.end);
            possible_end.push(next);
        }
        let last = possible_end.last_exn().clone();
        if last.end < a[n] {
            return Result::TooShort;
        }
        let mut res = vec![];
        let mut cur_pos = a[n];
        for ok_range in possible_end[0..n].iter().rev() {
            let use_len = if cur_pos - min_len <= ok_range.end {
                min_len
            } else if cur_pos - max_len >= ok_range.start {
                max_len
            } else {
                cur_pos - ok_range.end
            };
            res.push(use_len);
            cur_pos -= use_len;
            assert!(use_len >= min_len && use_len <= max_len);
            assert!(cur_pos >= ok_range.start && cur_pos <= ok_range.end);
        }
        res.reverse();
        Result::OkLength(res)
    };
    let build_answer_with_diff = |diff: i32| -> Result {
        let min_len = binary_search_first_true(1..l + 1, |min_len| {
            build_answer(min_len, min_len + diff) != Result::TooShort
        });
        build_answer(min_len, min_len + diff)
    };
    let smallest_diff = binary_search_first_true(0..l + 1, |diff| -> bool {
        if let Result::OkLength(_) = build_answer_with_diff(diff) {
            true
        } else {
            false
        }
    });
    if let Result::OkLength(lengths) = build_answer_with_diff(smallest_diff) {
        for w in lengths.pref_sum().windows(2) {
            out_line!(w[0], w[1]);
        }
    } else {
        unreachable!();
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
