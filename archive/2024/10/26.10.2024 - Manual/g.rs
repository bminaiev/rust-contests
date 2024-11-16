//{"name":"g","group":"Manual","url":"","interactive":false,"timeLimit":2000,"tests":[{"input":"","output":""},{"input":"","output":""},{"input":"","output":""},{"input":"","output":""}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"g"}}}

use std::collections::{BTreeSet, HashMap};

#[allow(unused)]
use algo_lib::dbg;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::binary_search::binary_search_first_true;
use algo_lib::misc::rand::Random;
use algo_lib::misc::vec_apply_delta::ApplyDelta;
use algo_lib::seg_trees::fenwick::Fenwick;

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct Event {
    left: usize,
    start: bool,
    value: usize,
}

fn solve_case(a: &[usize]) -> Vec<i64> {
    let n = a.len() / 2;
    let mut singles = BTreeSet::new();
    let mut positions = vec![vec![]; n];
    for i in 0..a.len() {
        positions[a[i]].push(i);
    }
    let mut events = vec![vec![]; a.len() + 1];
    for left in (0..a.len()).rev() {
        let x = a[left];
        if positions[x][0] == left {
            let second = positions[x][1];
            singles.remove(&second);
        } else {
            singles.insert(left);
        }
        let mut iter = singles.iter();
        if let Some(&first) = iter.next() {
            let mut second = a.len();
            if let Some(&second_) = iter.next() {
                second = second_;
            }
            events[first].push(Event {
                left,
                start: true,
                value: first,
            });
            events[second].push(Event {
                left,
                start: false,
                value: first,
            });
        }
    }
    let mut rnd = Random::new(123123);
    let mut hashes = vec![0; n];
    for i in 0..n {
        hashes[i] = rnd.gen_u64();
    }
    let mut by_hash = HashMap::<u64, Vec<usize>>::new();
    let mut res = vec![0; a.len()];
    assert!(singles.is_empty());
    let mut who = vec![None; a.len()];
    let mut cur_hash = 0;
    let mut pref_hashes = vec![0; a.len() + 1];
    by_hash.entry(0).or_default().push(0);
    let mut fenw = Fenwick::<i64>::new(a.len() + 1);
    for right in 0..a.len() {
        let x = a[right];
        cur_hash ^= hashes[x];
        if positions[x][0] == right {
            singles.insert(right);
        } else {
            singles.remove(&positions[x][0]);
        }
        for ev in events[right].iter() {
            if ev.start {
                assert!(who[ev.left].is_none());
                who[ev.left] = Some(ev.value);
                res[ev.value] -= fenw.get_sum(ev.left);
            } else {
                who[ev.left] = None;
                res[ev.value] += fenw.get_sum(ev.left);
            }
        }
        let mut iter = singles.iter();
        let first = iter.next_back();
        let start_from = if let Some(&first) = first {
            first + 1
        } else {
            0
        };
        fenw.add(start_from, 1);
        fenw.add(right + 1, -1);
        // for left in start_from..=right {
        //     if let Some(who) = who[left] {
        //         res[who] += 1;
        //     }
        // }
        if let Some(&first) = first {
            let mut cur_start_from = 0;
            if let Some(second) = iter.next_back() {
                cur_start_from = second + 1;
            }
            let need_hash = cur_hash ^ hashes[a[first]];
            if let Some(vec) = by_hash.get(&need_hash) {
                let first_ok = binary_search_first_true(0..vec.len(), |i| vec[i] >= cur_start_from);
                res[first] += (vec.len() - first_ok) as i64;
                // for &left in vec.iter() {
                //     if left >= cur_start_from {
                //         res[first] += 1;
                //     }
                // }
            }
            // for left in cur_start_from..=first {

            //     if pref_hashes[left] == need_hash {
            //         res[first] += 1;
            //     }
            // }
        }
        by_hash.entry(cur_hash).or_default().push(right + 1);
        pref_hashes[right + 1] = cur_hash;
    }
    for ev in events[a.len()].iter() {
        if ev.start {
            assert!(who[ev.left].is_none());
            who[ev.left] = Some(ev.value);
            res[ev.value] -= fenw.get_sum(ev.left);
        } else {
            who[ev.left] = None;
            res[ev.value] += fenw.get_sum(ev.left);
        }
    }

    res
}

fn solve_slow(a: &[usize]) -> Vec<i64> {
    let mut res = vec![0; a.len()];
    let mut rnd = Random::new(345345);
    let mut hashes = vec![0; a.len()];
    for i in 0..a.len() {
        hashes[i] = rnd.gen_u64();
    }
    for mid in 0..a.len() {
        for left in 0..=mid {
            for right in mid + 1..=a.len() {
                let mut cur_hash = 0;
                for i in left..right {
                    cur_hash ^= hashes[a[i]];
                }
                if cur_hash == hashes[a[mid]] {
                    res[mid] += 1;
                }
            }
        }
    }
    res
}

fn stress() {
    for it in 1.. {
        dbg!(it);
        let mut rnd = Random::new(it);
        let n = rnd.gen(1..100000);
        let mut a = vec![0; n * 2];
        for i in 0..n {
            a[i] = i;
            a[i + n] = i;
        }
        rnd.shuffle(&mut a);
        let my = solve_case(&a);
        // let slow = solve_slow(&a);
        // // dbg!(my);
        // if my != slow {
        //     dbg!(a);
        //     dbg!(my);
        //     dbg!(slow);
        //     assert_eq!(my, slow);
        // }
    }
}

fn solve(input: &mut Input, out: &mut Output, _test_case: usize) {
    let n = input.usize();
    let a = input.vec::<usize>(n * 2).sub_from_all(1);
    let res = solve_case(&a);
    out.println(res);
}

pub(crate) fn run(mut input: Input, mut output: Output) -> bool {
    solve(&mut input, &mut output, 1);
    output.flush();
    true
}

//START MAIN

fn main() {
    const PROBLEM_NAME: &str = "g";
    use algo_lib::tester::helper::*;

    run_tests(PROBLEM_NAME, run);
    // run_single_test(PROBLEM_NAME, run, "1");
    // run_stress(stress);
    // run_locally(run);
}
//END MAIN
