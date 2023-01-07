//{"name":"F. Reverse³","group":"TLX - TLX Regular Open Contest #30","url":"https://tlx.toki.id/contests/troc-30/problems/F","interactive":false,"timeLimit":2000,"tests":[{"input":"5 2\n2 1 5 3 1\n","output":"YES\n"},{"input":"8 3\n7 7 2 5 4 7 1 3\n","output":"NO\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"FReverse"}}}

use algo_lib::collections::inversions_count::inversions_count;
use algo_lib::collections::last_exn::LastExn;
use algo_lib::collections::sorted::SortedTrait;
use algo_lib::io::output::output;
use algo_lib::io::task_io_settings::TaskIoType;
use algo_lib::io::task_runner::run_task;
use algo_lib::io::{input::Input, task_io_settings::TaskIoSettings};
#[allow(unused)]
use algo_lib::{dbg, out, out_line};

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord)]
struct Element {
    value: i32,
    pos: usize,
}

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.usize();
    let k = input.usize();
    let a = input.vec::<i32>(n);
    if k == 1 {
        out_line!("YES");
        return;
    }
    let a_sorted = a.sorted();
    let mut parity = vec![];
    for offset in 0..k {
        let mut elems = vec![];
        for i in (offset..n).step_by(k) {
            elems.push(Element {
                value: a[i],
                pos: i,
            })
        }
        elems.sort();
        for i in 0..elems.len() {
            if a_sorted[offset + i * k] != elems[i].value {
                out_line!("NO");
                return;
            }
        }
        let mut same = false;
        for i in 0..elems.len() - 1 {
            if elems[i].value == elems[i + 1].value {
                same = true;
            }
        }
        if same {
            continue;
        }
        let mut b: Vec<_> = elems.iter().map(|e| e.pos).collect();
        let mut inside_b = b.clone();
        inside_b.sort();
        for i in 0..b.len() {
            b[i] = inside_b.binary_search(&b[i]).unwrap();
        }
        parity.push(inversions_count(&b) % 2);
    }
    parity.sort();
    if parity.is_empty() || parity[0] == *parity.last_exn() {
        out_line!("YES");
    } else {
        out_line!("NO");
    }
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
