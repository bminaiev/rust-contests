#[allow(unused)]
use algo_lib::dbg;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::pref_sum::PrefSum;
use algo_lib::misc::vec_apply_delta::ApplyDelta;

fn solve(input: &mut Input, out: &mut Output) {
    let tc = input.usize();
    for _ in 0..tc {
        let n = input.usize();
        let k = input.i64();
        let start = input.usize() - 1;
        let add = input.vec::<i64>(n);
        let add_pref_sum = add.pref_sum();
        let get_inside = |from: usize, to: usize| -> i64 {
            if from < to {
                add_pref_sum[to] - add_pref_sum[from + 1]
            } else {
                add_pref_sum[from] - add_pref_sum[to + 1]
            }
        };
        let build_at = input.vec::<i64>(n - 1).sub_from_all(1);
        let mut can_be_at = vec![i64::MAX; n];
        can_be_at[start] = -1;
        for v in (0..start).rev() {
            can_be_at[v] = (can_be_at[v + 1] + 1).max(build_at[v]);
        }
        for v in start + 1..n {
            can_be_at[v] = (can_be_at[v - 1] + 1).max(build_at[v - 1]);
        }
        let mut positions: Vec<usize> = (0..n).collect();
        positions.sort_by_key(|p| can_be_at[*p]);
        // dp[v] -- if I will be at position `v` at time `can_be_at[v]`, how much I'll earn for all times <= can_be_at[v]
        let mut dp = vec![0; n];
        let mut res = 0;
        let mut left_items: Vec<Item> = vec![];
        let mut right_items = vec![Item {
            add: add[start],
            pos: start,
        }];
        for it2 in 1..n {
            let next_v = positions[it2];
            let mut best_money = 0;
            for item_group in [&left_items, &right_items] {
                for item in item_group.iter() {
                    let cur_v = item.pos;
                    let dist = cur_v.abs_diff(next_v) as i64;
                    if can_be_at[cur_v] + dist > can_be_at[next_v] {
                        continue;
                    }
                    let extra_time = can_be_at[next_v] - can_be_at[cur_v] - dist;
                    let mut money = extra_time * add[cur_v] + dp[cur_v] + add[next_v];
                    money += get_inside(cur_v, next_v);
                    money = money.max(dp[next_v]);
                    best_money = best_money.max(money);
                }
            }
            dp[next_v] = best_money;
            let item = Item {
                add: add[next_v],
                pos: next_v,
            };
            if next_v > start {
                add_item(&mut right_items, item)
            } else {
                add_item(&mut left_items, item)
            }
        }
        for it in 0..n {
            let cur_v = positions[it];
            if can_be_at[cur_v] < k {
                let money_at_v = dp[cur_v] + (k - can_be_at[cur_v] - 1) * add[cur_v];
                res = res.max(money_at_v);
            }
        }
        out.println(res);
    }
}

#[derive(Clone, Copy)]
struct Item {
    add: i64,
    pos: usize,
}

fn add_item(items: &mut Vec<Item>, item: Item) {
    items.push(item);
    if items.len() > 2 {
        let mut smallest_pos = 0;
        for i in 1..items.len() {
            if items[i].add < items[smallest_pos].add {
                smallest_pos = i;
            }
        }
        items.swap_remove(smallest_pos);
    }
}

pub(crate) fn run(mut input: Input, mut output: Output) -> bool {
    solve(&mut input, &mut output);
    output.flush();
    true
}

#[cfg(feature = "local")]
fn main() {
    const PROBLEM_NAME: &str = "g_";
    use algo_lib::tester::helper::*;

    run_tests(PROBLEM_NAME, run);
    // run_single_test(PROBLEM_NAME, run, "2");
    // run_stress(stress);
    // run_locally(run);
}
//END MAIN

#[cfg(not(feature = "local"))]
fn main() {
    let input = algo_lib::io::input::Input::new_stdin();
    let mut output = algo_lib::io::output::Output::new_stdout();
    run(input, output);
}
