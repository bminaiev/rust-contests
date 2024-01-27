//{"name":"o","group":"Manual","url":"","interactive":false,"timeLimit":2000,"tests":[{"input":"","output":""},{"input":"","output":""},{"input":"","output":""}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"o"}}}

use std::collections::BTreeSet;
use std::ops::Range;

#[allow(unused)]
use algo_lib::dbg;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::rand::Random;
use algo_lib::seg_trees::lazy_seg_tree_add_sum::{Node, SegTreeAddSum};

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default)]
struct Point {
    y: i64,
    x: i64,
    id: usize,
    is_open: bool,
}

fn solve_case(mut a: Vec<Point>) -> Option<Vec<usize>> {
    a.sort_by_key(|p| (p.x, p.y));
    let n = a.len() / 2;
    let mut res = vec![usize::MAX; n];
    let mut i = 0;
    let mut opens = BTreeSet::new();
    while i != a.len() {
        let mut j = i;
        while j < a.len() && a[j].x == a[i].x {
            j += 1;
        }
        for p in a[i..j].iter() {
            if p.is_open {
                opens.insert(p);
            }
        }
        for p in a[i..j].iter() {
            if !p.is_open {
                if let Some(prev) = opens
                    .range(
                        ..Point {
                            y: p.y - 1,
                            x: i64::MAX,
                            id: usize::MAX,
                            is_open: false,
                        },
                    )
                    .next_back()
                    .cloned()
                {
                    res[prev.id] = p.id;
                    opens.remove(prev);
                } else {
                    return None;
                }
            }
        }
        i = j;
    }
    Some(res)
}

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord)]
struct Event {
    y: i64,
    x_range: (i64, i64),
    change: i64,
}

fn check(mut a: Vec<Point>, pairs: &[usize]) -> bool {
    let mut rnd = Random::new(787788);
    let n = a.len() / 2;
    let mut ends = vec![Point::default(); n];
    for p in a.iter_mut() {
        if !p.is_open {
            ends[p.id] = *p;
        }
    }
    let mut magic = vec![0; n];
    for i in 0..n {
        magic[i] = rnd.gen_u64() as i64;
    }
    let mut events = vec![];
    for p in a.iter() {
        if p.is_open {
            let q = ends[pairs[p.id]];
            let magic = magic[p.id];
            events.push(Event {
                y: p.y,
                x_range: (p.x, q.x),
                change: magic,
            });
            events.push(Event {
                y: q.y + 1,
                x_range: (p.x, q.x),
                change: -magic,
            })
        }
    }
    events.sort();
    let mut all_x: Vec<_> = a.iter().map(|p| p.x).collect();
    all_x.sort();
    all_x.dedup();
    a.sort_by_key(|p| p.y);
    let mut st = SegTreeAddSum::new(all_x.len(), |_| Node { len: 1, sum: 0 });
    let mut pref_sum_open = vec![0; n];
    let mut pref_sum_close = vec![0; n];
    let mut it = 0;

    for p in a.iter() {
        while it < events.len() && events[it].y <= p.y {
            let x_from = all_x.binary_search(&events[it].x_range.0).unwrap();
            let x_to = all_x.binary_search(&events[it].x_range.1).unwrap();
            st.update(x_from..x_to + 1, events[it].change);
            it += 1;
        }
        let x = all_x.binary_search(&p.x).unwrap();

        let my_sum = st.get(x..x + 1).sum;
        if p.is_open {
            pref_sum_open[p.id] = my_sum;
        } else {
            pref_sum_close[p.id] = my_sum;
        }
    }
    for i in 0..n {
        if pref_sum_open[i] != pref_sum_close[pairs[i]] {
            return false;
        }
    }
    true
}

fn solve(input: &mut Input, out: &mut Output, _test_case: usize) {
    let n = input.usize();
    let mut a = vec![];
    for open in [true, false].iter() {
        for i in 0..n {
            let x = input.i64();
            let y = input.i64();
            a.push(Point {
                x,
                y,
                id: i,
                is_open: *open,
            });
        }
    }
    if let Some(perm) = solve_case(a.clone()) {
        if check(a.clone(), &perm) {
            out.println("Yes");
            for x in perm {
                out.println(x + 1);
            }
        } else {
            out.println("No");
        }
    } else {
        out.println("No");
    }
}

pub(crate) fn run(mut input: Input, mut output: Output) -> bool {
    solve(&mut input, &mut output, 1);
    output.flush();
    true
}

//START MAIN

fn main() {
    const PROBLEM_NAME: &str = "o";
    use algo_lib::tester::helper::*;

    run_tests(PROBLEM_NAME, run);
    // run_single_test(PROBLEM_NAME, run, "1");
    // run_stress(stress);
    // run_locally(run);
}
//END MAIN
