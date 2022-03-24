//{"name":"brazil_b","group":"Manual","url":"","interactive":false,"timeLimit":2000,"tests":[{"input":"","output":""},{"input":"","output":""},{"input":"","output":""}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"brazil_b"}}}

use std::collections::BTreeSet;

use algo_lib::io::output::output;
use algo_lib::io::task_io_settings::TaskIoType;
use algo_lib::io::task_runner::run_task;
use algo_lib::io::{input::Input, task_io_settings::TaskIoSettings};
use algo_lib::misc::binary_search::{binary_search_first_true, binary_search_last_true};
use algo_lib::misc::min_max::UpdateMinMax;
use algo_lib::strings::suffix_array::SuffixArray;
#[allow(unused)]
use algo_lib::{dbg, out, out_line};

fn solve(input: &mut Input, _test_case: usize) {
    let max_res = input.usize();
    let n = input.usize();
    let init_s = input.string();
    let mut big_string = vec![];
    for _ in 0..n {
        let mut word = input.string();
        big_string.append(&mut word);
        big_string.push(b'$');
    }
    let first_cnt = big_string.len();
    for _ in 0..2 {
        big_string.append(&mut init_s.clone());
    }
    let sa = SuffixArray::new(big_string);
    let mut good_positions = BTreeSet::new();
    for pos_in_str in 0..first_cnt {
        good_positions.insert(sa.get_pos_in_array(pos_in_str));
    }
    let mut if_start = vec![];
    for start in first_cnt..sa.len() {
        let pos = sa.get_pos_in_array(start);
        let mut cur_res = 0;
        if let Some(&right_ok) = good_positions.range(pos..).next() {
            let lcp = sa.lcp(pos, right_ok);
            cur_res.update_max(lcp);
        }
        if let Some(&left_ok) = good_positions.range(..pos).next_back() {
            let lcp = sa.lcp(left_ok, pos);
            cur_res.update_max(lcp);
        }
        cur_res.update_min(max_res);
        if_start.push(cur_res);
    }
    let res = binary_search_first_true(0..max_res + 1, |bad_len| -> bool {
        // if exist ok start, return true

        let mut balance_delta = vec![0; init_s.len()];
        for pos in 0..if_start.len() {
            if if_start[pos] < bad_len {
                continue;
            }
            let bad_from = if pos + bad_len <= init_s.len() {
                0
            } else {
                pos + bad_len - init_s.len()
            };
            if bad_from < balance_delta.len() {
                balance_delta[bad_from] += 1;
                if pos + 1 < balance_delta.len() {
                    balance_delta[pos + 1] -= 1;
                }
            }
        }
        let mut cur_balance = 0;
        for pos in 0..init_s.len() {
            cur_balance += balance_delta[pos];
            if cur_balance == 0 {
                return true;
            }
        }
        false
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
}
//END MAIN
