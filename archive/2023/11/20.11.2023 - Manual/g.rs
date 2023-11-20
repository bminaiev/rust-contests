//{"name":"g","group":"Manual","url":"","interactive":false,"timeLimit":2000,"tests":[{"input":"","output":""},{"input":"","output":""},{"input":"","output":""}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"g"}}}

use std::collections::HashMap;

use algo_lib::io::output::output;
use algo_lib::io::task_io_settings::TaskIoType;
use algo_lib::io::task_runner::run_task;
use algo_lib::io::{input::Input, task_io_settings::TaskIoSettings};
use algo_lib::misc::binary_search::binary_search_first_true;
#[allow(unused)]
use algo_lib::{dbg, out, out_line};

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct Segment {
    fr: i64,
    to: i64,
}

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.i64();
    let m = input.i64();
    let k = input.usize();
    let mut by_y: HashMap<i64, Vec<Segment>> = HashMap::new();
    let mut tot_v = n * m;
    let mut tot_edges = (n - 1) * m + (m - 1) * n;
    for _ in 0..k {
        let x1 = input.i64();
        let x2 = input.i64();
        let y = input.i64();
        let len = x2 - x1 + 1;
        tot_v -= len;
        tot_edges -= len - 1;
        if y != 1 {
            tot_edges -= len;
        }
        if y != m {
            tot_edges -= len;
        }
        if x1 != 1 {
            tot_edges -= 1;
        }
        if x2 != n {
            tot_edges -= 1;
        }

        by_y.entry(y).or_default().push(Segment { fr: x1, to: x2 });
    }
    for (_k, v) in by_y.iter_mut() {
        v.sort();
    }

    for (&y, list1) in by_y.iter() {
        for w in list1.windows(2) {
            if w[0].to + 1 == w[1].fr {
                tot_edges += 1;
            }
        }
        if let Some(list2) = by_y.get(&(y + 1)) {
            tot_edges += find_intersection(list1, list2);
        }
    }

    if tot_v == tot_edges + 1 {
        out_line!("YES");
    } else {
        out_line!("NO");
    }
}

fn intersection_len(s1: &Segment, s2: &Segment) -> i64 {
    let fr = s1.fr.max(s2.fr);
    let to = s1.to.min(s2.to);
    (to - fr + 1).max(0)
}

fn find_intersection(list1: &[Segment], list2: &[Segment]) -> i64 {
    let mut res = 0;
    for seg in list1.iter() {
        let mut it = binary_search_first_true(0..list2.len(), |it| list2[it].to >= seg.fr);
        while it < list2.len() && list2[it].fr <= seg.to {
            res += intersection_len(seg, &list2[it]);
            it += 1;
        }
    }
    res
}

pub(crate) fn run(mut input: Input) -> bool {
    solve(&mut input, 1);
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
