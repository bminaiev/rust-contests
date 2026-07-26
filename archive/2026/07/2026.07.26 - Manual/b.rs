use std::assert_eq;
use std::collections::VecDeque;

use algo_lib::collections::fx_hash_map::FxHashSet;
use algo_lib::collections::min_priority_queue::MinPriorityQueue;
#[allow(unused)]
use algo_lib::dbg;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord)]
struct Item {
    cost: i64,
    s: String,
}

fn solve_case(a: &[i64], h: i64) -> (i64, Vec<String>) {
    let n = a.len();

    let mut a_sorted_indexes: Vec<usize> = (0..n).collect();
    a_sorted_indexes.sort_by_key(|i| a[*i]);
    a_sorted_indexes.reverse();

    let mut seen = FxHashSet::default();
    let mut start = vec![1, 1, h];
    let mut queue: VecDeque<Vec<i64>> = VecDeque::new();
    seen.insert(start.clone());
    queue.push_back(start);
    let mut best_score = i64::MAX;
    let mut best_lens = vec![];
    while let Some(cur) = queue.pop_front() {
        if cur.len() >= n {
            let mut score = 0;
            for i in 0..n {
                score += a[a_sorted_indexes[i]] * cur[i];
            }
            if score < best_score {
                best_score = score;
                best_lens = cur.clone();
            }
        }
        let mut score_at_least = 0;
        for i in 0..cur.len() {
            if i > n {
                break;
            }
            score_at_least += a[a_sorted_indexes[i]] * cur[i];
            if score_at_least >= best_score {
                break;
            }
            if i > 0 && cur[i - 1] == cur[i] {
                continue;
            }
            let mut next = cur.clone();
            let value = next[i];
            next.remove(i);
            next.push(value + 1);
            next.push(value + 1);
            next.push(value + h);
            next.sort();
            next.truncate(n);
            if !seen.contains(&next) {
                seen.insert(next.clone());
                queue.push_back(next);
            }
        }
    }
    let mut iter = 0;
    let mut queue = MinPriorityQueue::new();
    queue.push(Item {
        cost: 1,
        s: "0".to_string(),
    });
    queue.push(Item {
        cost: 1,
        s: "1".to_string(),
    });
    queue.push(Item {
        cost: h,
        s: "*".to_string(),
    });
    let mut res = vec!["".to_string(); n];
    while let Some(Item { cost, s }) = queue.pop() {
        if cost == best_lens[iter] {
            res[a_sorted_indexes[iter]] = s.clone();
            iter += 1;
            if iter == n {
                break;
            }
            continue;
        }
        queue.push(Item {
            cost: cost + 1,
            s: format!("{}0", s),
        });
        queue.push(Item {
            cost: cost + 1,
            s: format!("{}1", s),
        });
        queue.push(Item {
            cost: cost + h,
            s: format!("{}*", s),
        });
    }
    (best_score, res)
}

fn solve(input: &mut Input, out: &mut Output) {
    let tc = input.usize();
    for i in 0..tc {
        dbg!(i, tc);
        let n = input.usize();
        let h = input.i64();
        let a = input.vec::<i64>(n);
        let (cost, res) = solve_case(&a, h);
        let mut test_cost = 0;
        for i in 0..n {
            let mut local_cost = 0;
            for j in 0..res[i].len() {
                if res[i].as_bytes()[j] == b'0' || res[i].as_bytes()[j] == b'1' {
                    local_cost += 1;
                } else {
                    local_cost += h;
                }
            }
            test_cost += local_cost * a[i];
        }
        assert_eq!(cost, test_cost);
        out.println(cost);
        out.println(res);
    }
}

pub(crate) fn run(mut input: Input, mut output: Output) -> bool {
    solve(&mut input, &mut output);
    output.flush();
    true
}

#[cfg(feature = "local")]
fn main() {
    const PROBLEM_NAME: &str = "b";
    #[allow(unused)]
    use algo_lib::misc::dragon::run_dragon;
    use algo_lib::tester::helper::*;

    // run_tests(PROBLEM_NAME, run);
    // run_single_test(PROBLEM_NAME, run, "1");
    // run_stress(stress);
    // run_locally(run);
    run_dragon(solve, "B", 4..5);
}
//END MAIN

#[cfg(not(feature = "local"))]
fn main() {
    let input = algo_lib::io::input::Input::new_stdin();
    let mut output = algo_lib::io::output::Output::new_stdout();
    run(input, output);
}
