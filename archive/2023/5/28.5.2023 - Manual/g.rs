//{"name":"g","group":"Manual","url":"","interactive":false,"timeLimit":2000,"tests":[{"input":"","output":""}],"testType":"multiNumber","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"g"}}}

use std::cmp::{max, min};
use std::collections::HashSet;

use algo_lib::graph::dsu::Dsu;
use algo_lib::io::output::output;
use algo_lib::io::task_io_settings::TaskIoType;
use algo_lib::io::task_runner::run_task;
use algo_lib::io::{input::Input, task_io_settings::TaskIoSettings};
#[allow(unused)]
use algo_lib::{dbg, out, out_line};

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct Edge {
    w: i64,
    fr: i64,
    to: i64,
}

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.i64();
    let m = input.usize();
    let mut coords = vec![1, n + 1];

    let mut seen = HashSet::new();

    let mut start_edges = vec![];
    let mut all_edges = vec![];

    for _ in 0..m {
        let fr = input.i64();
        let to = input.i64();
        let w = input.i64();
        coords.push(fr);
        coords.push(fr + 1);
        coords.push(to);
        coords.push(to + 1);

        seen.insert((fr, to));
        start_edges.push(Edge { fr, to, w });
    }

    coords.sort();
    coords.dedup();

    let conv = |x: i64| -> Option<usize> {
        if x < 1 || x > n {
            return None;
        }
        match coords.binary_search(&x) {
            Ok(pos) => Some(pos),
            Err(pos) => {
                assert!(pos > 0);
                Some(pos - 1)
            }
        }
    };
    let mut res = 0;
    for w in coords.windows(2) {
        res += w[1] - w[0] - 1;
    }

    const DELTA: i64 = 1;

    for e in start_edges.iter() {
        let fr = conv(e.fr).unwrap();
        let to = conv(e.to).unwrap();
        all_edges.push(Edge {
            fr: fr as i64,
            to: to as i64,
            w: e.w,
        });
        for &(fr, to) in [(e.fr, e.to), (e.to, e.fr)].iter() {
            for delta_fr in -DELTA..=DELTA {
                for delta_to in -DELTA..=DELTA {
                    let real_fr = fr + delta_fr;
                    let real_to = to + delta_to;
                    let mn = min(real_fr, real_to);
                    let mx = max(real_fr, real_to);
                    if !seen.contains(&(mn, mx)) {
                        let w = mx - mn;
                        if let Some(mn) = conv(mn) {
                            if let Some(mx) = conv(mx) {
                                all_edges.push(Edge {
                                    fr: mn as i64,
                                    to: mx as i64,
                                    w,
                                });
                            }
                        }
                    }
                }
            }
        }
    }
    for &mx in coords[1..].iter() {
        let mn = mx - 1;
        if !seen.contains(&(mn, mx)) {
            let w = mx - mn;
            if let Some(mn) = conv(mn) {
                if let Some(mx) = conv(mx) {
                    all_edges.push(Edge {
                        fr: mn as i64,
                        to: mx as i64,
                        w,
                    });
                }
            }
        }
    }
    all_edges.sort();
    let mut dsu = Dsu::new(coords.len());
    for e in all_edges.iter() {
        if dsu.unite(e.fr as usize, e.to as usize) {
            res += e.w;
        }
    }

    out_line!(res);
}

pub(crate) fn run(mut input: Input) -> bool {
    let t = input.read();
    for i in 0usize..t {
        solve(&mut input, i + 1);
    }
    output().flush();
    true
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
    // tester::run_locally();
    // tester::run_with_last_downloaded_file();
}
//END MAIN
