//{"name":"I. Optimal Assortment","group":"Yandex - Stage 18: Grand Prix of Bytedance","url":"https://official.contest.yandex.com/opencupXXII/contest/39023/problems/I/","interactive":false,"timeLimit":2000,"tests":[{"input":"2 5\n4 2\n4 3 2\n4 3 2\n2 1 2\n1 1 2 3\n1 0 0 0\n1 1 0 0\n1 2 0 0\n","output":"16/9\n10/9\n1/1\n2/1\n2/1\n0/1\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"IOptimalAssortment"}}}

use algo_lib::io::output::output;
use algo_lib::io::task_io_settings::TaskIoType;
use algo_lib::io::task_runner::run_task;
use algo_lib::io::{input::Input, task_io_settings::TaskIoSettings};
use algo_lib::math::gcd::gcd;
use algo_lib::misc::binary_search::binary_search_first_true;
use algo_lib::seg_trees::fenwick::Fenwick;
#[allow(unused)]
use algo_lib::{dbg, out, out_line};

#[derive(Clone, Copy)]
enum Query {
    ChangeProfit(usize, i64),
    ChangeRange(usize, i64, i64),
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct ProfitPos {
    profit: i64,
    who: usize,
    timestamp: usize,
    idx: usize,
}

#[derive(Clone, Copy)]
struct Frac {
    up: i64,
    down: i64,
}

impl Frac {
    pub fn new(up: i64, down: i64) -> Self {
        if down == 0 {
            return Self { up: 0, down: 1 };
        }
        let g = gcd(up, down);
        Self {
            up: up / g,
            down: down / g,
        }
    }

    pub fn other_is_better(&self, other: &Self) -> bool {
        (other.up as i128) * (self.down as i128) > (self.up as i128) * (other.down as i128)
    }
}

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.usize();
    let m = input.usize();
    let mut cur_profit = input.vec::<i64>(n);
    let mut left = vec![0; n];
    let mut right = vec![0; n];
    let _ = input.i64();
    for i in 0..n {
        left[i] = input.i64();
    }
    let mut w0 = input.i64();
    for i in 0..n {
        right[i] = input.i64();
    }
    let mut queries = vec![];
    for i in 0..m {
        let q_type = input.usize() - 1;
        if q_type == 0 {
            let x = input.usize();
            let fr = input.i64();
            let to = input.i64();
            queries.push(Query::ChangeRange(x, fr, to));
        } else {
            let x = input.usize() - 1;
            let prof = input.i64();
            queries.push(Query::ChangeProfit(x, prof));
        }
    }
    let mut all_profits = vec![];
    for i in 0..n {
        all_profits.push(ProfitPos {
            profit: cur_profit[i],
            who: i,
            timestamp: 0,
            idx: 0,
        });
    }
    for i in 0..m {
        if let Query::ChangeProfit(who, new_profit) = queries[i] {
            all_profits.push(ProfitPos {
                profit: new_profit,
                who,
                timestamp: i + 1,
                idx: 0,
            });
        }
    }
    all_profits.sort_by_key(|p| p.profit);
    for i in 0..all_profits.len() {
        all_profits[i].idx = i;
    }
    let mut prof_by_who = vec![vec![]; n];
    for p in all_profits.iter() {
        prof_by_who[p.who].push(p.clone());
    }
    for i in 0..n {
        prof_by_who[i].sort_by_key(|p| p.timestamp);
    }

    let total_len = all_profits.len();
    let mut sum_l: Fenwick<i64> = Fenwick::new(total_len);
    let mut sum_lv: Fenwick<i64> = Fenwick::new(total_len);
    let mut who_idx = vec![0; n];
    for who in 0..n {
        let cur_pos = prof_by_who[who][who_idx[who]].idx;
        sum_l.add(cur_pos, left[who]);
        sum_lv.add(cur_pos, left[who] * cur_profit[who]);
    }

    let mut alive = vec![false; total_len];
    for i in 0..n {
        alive[prof_by_who[i][who_idx[i]].idx] = true;
    }

    let print_ans = |sum_l: &Fenwick<i64>,
                     sum_lv: &Fenwick<i64>,
                     w0: i64,
                     cur_profit: &[i64],
                     right: &[i64],
                     alive: &[bool]| {
        let first_idx = binary_search_first_true(0..total_len, |idx| {
            let sum_cost = sum_lv.get_suffix_sum(idx);
            let sum_l = sum_l.get_suffix_sum(idx) + w0;
            let cur_profit = all_profits[idx].profit;
            // TODO: think about sign
            sum_cost <= cur_profit * sum_l
        });
        let mut cur_res = Frac::new(
            sum_lv.get_suffix_sum(first_idx),
            sum_l.get_suffix_sum(first_idx) + w0,
        );
        if first_idx != 0 {
            const M: usize = 0;
            let start = if first_idx > M { first_idx - M } else { 0 };
            let sum_lv = sum_lv.get_suffix_sum(first_idx);
            let sum_l = sum_l.get_suffix_sum(first_idx);
            for i in start..first_idx {
                if !alive[i] {
                    continue;
                }
                let who = all_profits[i].who;
                let another_res = Frac::new(
                    sum_lv + right[who] * cur_profit[who],
                    sum_l + w0 + right[who],
                );
                if cur_res.other_is_better(&another_res) {
                    cur_res = another_res;
                }
            }
        }
        out_line!(format!("{}/{}", cur_res.up, cur_res.down));
    };
    print_ans(&sum_l, &sum_lv, w0, &cur_profit, &right, &alive);
    for i in 0..m {
        if let Query::ChangeProfit(who, new_profit) = queries[i] {
            let cur_pos = prof_by_who[who][who_idx[who]].idx;
            sum_l.add(cur_pos, -left[who]);
            sum_lv.add(cur_pos, -left[who] * cur_profit[who]);
            alive[cur_pos] = false;

            cur_profit[who] = new_profit;
            who_idx[who] += 1;

            assert!(prof_by_who[who][who_idx[who]].timestamp == i + 1);

            let cur_pos = prof_by_who[who][who_idx[who]].idx;
            alive[cur_pos] = true;
            sum_l.add(cur_pos, left[who]);
            sum_lv.add(cur_pos, left[who] * cur_profit[who]);
        } else if let Query::ChangeRange(who, fr, to) = queries[i] {
            if who == 0 {
                w0 = to;
            } else {
                let who = who - 1;

                let cur_pos = prof_by_who[who][who_idx[who]].idx;
                sum_l.add(cur_pos, -left[who]);
                sum_lv.add(cur_pos, -left[who] * cur_profit[who]);

                left[who] = fr;
                right[who] = to;

                sum_l.add(cur_pos, left[who]);
                sum_lv.add(cur_pos, left[who] * cur_profit[who]);
            }
        } else {
            assert!(false);
        }
        print_ans(&sum_l, &sum_lv, w0, &cur_profit, &right, &alive);
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
