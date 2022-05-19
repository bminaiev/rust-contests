//{"name":"F. Разнообразные отрезки","group":"Codeforces - Codeforces Round #792 (Div. 1 + Div. 2)","url":"https://codeforces.com/contest/1684/problem/F","interactive":false,"timeLimit":2000,"tests":[{"input":"5\n7 3\n1 1 2 1 3 3 5\n1 4\n4 5\n2 4\n5 2\n10 1 6 14 1\n4 5\n2 4\n4 5\n5 7 5 6\n2 2\n1 3\n2 4\n3 3\n3 4\n7 3\n2 2 2 7 8 2 2\n4 4\n4 4\n5 5\n1 1\n123\n1 1\n","output":"2\n0\n1\n0\n0\n"}],"testType":"multiNumber","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"FRaznoobraznieOtrezki"}}}

use std::cmp::max;
use std::collections::{BTreeSet, HashMap};

use algo_lib::collections::last_exn::LastExn;
use algo_lib::io::output::output;
use algo_lib::io::task_io_settings::TaskIoType;
use algo_lib::io::task_runner::run_task;
use algo_lib::io::{input::Input, task_io_settings::TaskIoSettings};
use algo_lib::misc::binary_search::binary_search_first_true;
use algo_lib::misc::min_max::UpdateMinMax;
use algo_lib::seg_trees::lazy_seg_tree::{LazySegTree, LazySegTreeNodeSpec};
use algo_lib::seg_trees::lazy_seg_tree_max_update::{Node, SegTreeMaxUpdate};
#[allow(unused)]
use algo_lib::{dbg, out, out_line};

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
struct Segment {
    r: usize,
    l: usize,
    id: usize,
}

type SegTree = SegTreeMaxUpdate;

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.usize();
    let m = input.usize();
    let a = input.vec::<i32>(n);
    let mut segs_by_right = vec![vec![]; n];
    let mut segs_by_left = vec![vec![]; n];
    for id in 0..m {
        let l = input.usize() - 1;
        let r = input.usize();
        let s = Segment { l, r, id };
        segs_by_right[r - 1].push(s);
        segs_by_left[l].push(s);
    }
    let mut all_segs = BTreeSet::new();

    let mut idx_by_value: HashMap<i32, Vec<usize>> = HashMap::new();

    let mut st = SegTree::new_f(n, &|_| Node { max_val: 0 });

    let max = n + 2;

    for pos in (0..n).rev() {
        for s in segs_by_right[pos].iter() {
            all_segs.insert(s.clone());
        }
        let max_end = if all_segs.is_empty() {
            0
        } else {
            all_segs.iter().next_back().unwrap().r
        };
        let value = a[pos];

        let entry = idx_by_value.entry(value).or_default();

        let first_covered_idx =
            binary_search_first_true(0..entry.len(), |idx| entry[idx] < max_end);

        if first_covered_idx < entry.len() {
            let last = entry[first_covered_idx];

            if first_covered_idx + 1 == entry.len() {
                st.update(0..pos + 1, pos + 1);
                st.update(pos + 1..last + 1, last + 1);
                st.update(last + 1..n, max);
            } else {
                let prev_last = entry[first_covered_idx + 1];
                let second = *entry.last_exn();
                st.update(0..pos + 1, prev_last + 1);
                st.update(pos + 1..second + 1, last + 1);
                st.update(second + 1..n, max);
            }
        }

        entry.push(pos);

        for s in segs_by_left[pos].iter() {
            all_segs.remove(s);
        }
    }

    let mut res = n;
    for start in 0..n {
        let r = st.get(start..start + 1);
        if r.max_val != max {
            if r.max_val == 0 {
                res = 0;
            } else {
                res.update_min(r.max_val - start);
            }
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
    // tester::run_tests();
    tester::run_single_test("1");
}
//END MAIN
