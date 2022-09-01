//{"name":"C. Network Transfer","group":"Yandex - Day 7","url":"https://official.contest.yandex.com/ptz-summer-2022/contest/39552/problems/C/","interactive":false,"timeLimit":2000,"tests":[{"input":"2 10\n0 100 2\n4 200 1\n","output":"13\n30\n"},{"input":"2 10\n30 200 1\n10 100 2\n","output":"50\n20\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"CNetworkTransfer"}}}

use std::collections::BTreeSet;

use algo_lib::f;
use algo_lib::io::output::output;
use algo_lib::io::task_io_settings::TaskIoType;
use algo_lib::io::task_runner::run_task;
use algo_lib::io::{input::Input, task_io_settings::TaskIoSettings};
use algo_lib::misc::gen_vector::gen_vec;
use algo_lib::misc::ord_f64::OrdF64;
#[allow(unused)]
use algo_lib::{dbg, out, out_line};

#[derive(Clone, Copy)]
struct Item {
    id: usize,
    start_time: OrdF64,
    size: OrdF64,
    priority: OrdF64,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct SetItem {
    relative_time: OrdF64,
    id: usize,
}

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.usize();
    let w = input.f64();
    let mut items = gen_vec(n, |id| Item {
        id,
        start_time: input.read(),
        size: input.read(),
        priority: input.read(),
    });
    let not_sorted_items = items.clone();
    items.sort_by_key(|item| item.start_time);
    items.push(Item {
        start_time: f!(1e123),
        id: n,
        size: f!(0.0),
        priority: f!(1.0),
    });
    let mut set = BTreeSet::<SetItem>::new();

    let mut cur_time = f!(0.0);
    let mut cur_relative_time = f!(0.0);

    let mut sum_priorities = f!(0.0);

    let mut res = vec![f!(0.0); n];
    for item in items.iter() {
        let before_time = item.start_time;
        while let Some(first) = set.iter().next() {
            assert!(sum_priorities != f!(0.0));
            let speed = w / sum_priorities;
            let real_finish_time = cur_time + (first.relative_time - cur_relative_time) / speed;
            if real_finish_time > before_time {
                break;
            }
            cur_relative_time = first.relative_time;
            cur_time = real_finish_time;
            res[first.id] = real_finish_time;
            sum_priorities -= not_sorted_items[first.id].priority;
            set.remove(&first.clone());
        }

        if sum_priorities != f!(0.0) {
            let speed = w / sum_priorities;
            cur_relative_time += speed * (before_time - cur_time);
        }
        sum_priorities += item.priority;
        cur_time = before_time;
        let item_relative_time = cur_relative_time + item.size / item.priority;
        set.insert(SetItem {
            relative_time: item_relative_time,
            id: item.id,
        });
    }
    for &x in res.iter() {
        out_line!(x);
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
    // tester::run_stress(stress);
}
//END MAIN
