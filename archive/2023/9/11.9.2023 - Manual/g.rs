//{"name":"g","group":"Manual","url":"","interactive":false,"timeLimit":2000,"tests":[{"input":"","output":""}],"testType":"multiNumber","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"g"}}}

use algo_lib::io::output::output;
use algo_lib::io::task_io_settings::TaskIoType;
use algo_lib::io::task_runner::run_task;
use algo_lib::io::{input::Input, task_io_settings::TaskIoSettings};
use algo_lib::misc::vec_apply_delta::ApplyDelta;
#[allow(unused)]
use algo_lib::{dbg, out, out_line};

fn find_ops(mut p: Vec<usize>) -> Vec<Vec<[usize; 2]>> {
    let mut res = vec![];
    let n = p.len();
    let mut cycles = vec![];
    let mut seen = vec![false; n];
    let mut all_2 = true;
    for i in 0..n {
        if seen[i] {
            continue;
        }
        let mut v = i;
        let mut cycle = vec![];
        while !seen[v] {
            seen[v] = true;
            cycle.push(v);
            v = p[v];
        }
        if cycle.len() == 1 {
            continue;
        }
        if cycle.len() > 2 {
            all_2 = false;
        }
        cycles.push(cycle);
    }
    if cycles.is_empty() {
        return vec![];
    }
    if all_2 {
        let mut ops = vec![];
        for cycle in cycles {
            ops.push([cycle[0], cycle[1]]);
            p.swap(cycle[0], cycle[1]);
        }
        res.push(ops);
        for i in 0..n {
            assert!(p[i] == i);
        }
        return res;
    }
    let mut first = vec![];
    for cycle in cycles.iter() {
        let sz = cycle.len();
        let mut i = 0;
        let mut j = sz - 2;
        while i < j {
            first.push([cycle[i], cycle[j]]);
            i += 1;
            j -= 1;
        }
    }
    for pair in first.iter() {
        p.swap(pair[0], pair[1]);
    }
    res.push(first);
    let mut second = vec![];
    for v in 0..n {
        if p[v] > v {
            second.push([v, p[v]]);
        }
    }
    for pair in second.iter() {
        p.swap(pair[0], pair[1]);
    }
    res.push(second);
    for i in 0..n {
        assert!(p[i] == i);
    }

    res
}

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.usize();
    let p = input.vec::<usize>(n).sub_from_all(1);
    let ops = find_ops(p);
    out_line!(ops.len());
    for op in ops.iter() {
        out_line!(op.len());
        for pair in op.iter() {
            out_line!(pair[0] + 1, pair[1] + 1);
        }
    }
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
