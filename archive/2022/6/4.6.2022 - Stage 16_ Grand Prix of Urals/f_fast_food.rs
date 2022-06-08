//{"name":"F. Fast Food","group":"Yandex - Stage 16: Grand Prix of Urals","url":"https://official.contest.yandex.com/opencupXXII/contest/38278/problems/F/","interactive":false,"timeLimit":3000,"tests":[{"input":"5 2\n2 2\n1 3 6 4 1\n5 2 3 1 1\n","output":"9\n1 1 2 2 1\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"FFastFood"}}}

use std::collections::VecDeque;

use algo_lib::collections::array_2d::Array2D;
use algo_lib::io::output::output;
use algo_lib::io::task_io_settings::TaskIoType;
use algo_lib::io::task_runner::run_task;
use algo_lib::io::{input::Input, task_io_settings::TaskIoSettings};
use algo_lib::misc::gen_vector::gen_vec;
use algo_lib::misc::min_max::UpdateMinMax;
use algo_lib::misc::pref_sum::PrefSum;
use algo_lib::misc::two_min::TwoMin;
#[allow(unused)]
use algo_lib::{dbg, out, out_line};

#[derive(Clone, Copy)]
struct QueueItem {
    cost: i64,
    pos: usize,
    prev: usize,
}

#[derive(Clone, Copy)]
struct Prev {
    person: usize,
    day: usize,
}

fn solve(input: &mut Input, _test_case: usize) {
    let days = input.usize();
    let n = input.usize();
    let max_days = input.vec::<usize>(n);
    let one_day_cost = input.matrix::<i64>(n, days);
    let balance_prefix = gen_vec(n, |id| (&one_day_cost[id]).pref_sum());

    let mut queues = vec![VecDeque::default(); n];
    for i in 0..n {
        queues[i].push_back(QueueItem {
            cost: 0,
            pos: 0,
            prev: 0,
        });
    }

    let mut dp = Array2D::new(std::i64::MAX / 3, n, days);
    let mut prev = Array2D::new(Prev { person: 0, day: 0 }, n, days);
    for day in 0..days {
        for i in 0..n {
            loop {
                let first = queues[i].front().unwrap();
                if first.pos + max_days[i] < day + 1 {
                    // TODO: think
                    queues[i].pop_front();
                } else {
                    break;
                }
            }
            let best = queues[i].front().unwrap();
            let cur_cost = best.cost + balance_prefix[i][day + 1];
            dp[i][day] = cur_cost;
            prev[i][day] = Prev {
                person: best.prev,
                day: best.pos,
            }
        }
        let mut two_min = TwoMin::new(0usize, 0i64);
        for i in 0..n {
            two_min.add(i, dp[i][day]);
        }
        for i in 0..n {
            let check = two_min.get_by_not_id(i).unwrap();
            let check_cost = check.1;
            let prev = check.0;
            let elem = QueueItem {
                cost: check_cost - balance_prefix[i][day + 1],
                pos: day + 1,
                prev,
            };
            while let Some(last) = queues[i].back() {
                if last.cost >= elem.cost {
                    queues[i].pop_back();
                } else {
                    break;
                }
            }
            queues[i].push_back(elem);
        }
    }
    let mut best_id = 0;
    for i in 0..n {
        if dp[i][days - 1] < dp[best_id][days - 1] {
            best_id = i;
        }
    }
    let res = dp[best_id][days - 1];
    out_line!(res);

    let mut who = vec![0; days];
    {
        let mut cur_day = days;
        let mut cur_id = best_id;
        while cur_day != 0 {
            let p = prev[cur_id][cur_day - 1];
            for d in p.day..cur_day {
                who[d] = cur_id + 1;
            }
            cur_id = p.person;
            cur_day = p.day;
        }
    }
    for i in 0..days {
        assert_ne!(who[i], 0);
    }
    out_line!(who);
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
