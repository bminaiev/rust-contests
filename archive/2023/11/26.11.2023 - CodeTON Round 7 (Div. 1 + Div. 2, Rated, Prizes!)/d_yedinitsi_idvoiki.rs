//{"name":"D. Единицы и двойки","group":"Codeforces - CodeTON Round 7 (Div. 1 + Div. 2, Rated, Prizes!)","url":"https://codeforces.com/contest/1896/problem/D","interactive":false,"timeLimit":2000,"tests":[{"input":"2\n5 5\n2 1 2 1 2\n1 5\n1 6\n1 7\n2 4 2\n1 7\n3 2\n2 2 2\n1 6\n1 5\n","output":"YES\nYES\nNO\nYES\nYES\nNO\n"}],"testType":"multiNumber","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"DYedinitsiIDvoiki"}}}

use std::collections::BTreeSet;

use algo_lib::io::output::output;
use algo_lib::io::task_io_settings::TaskIoType;
use algo_lib::io::task_runner::run_task;
use algo_lib::io::{input::Input, task_io_settings::TaskIoSettings};
use algo_lib::seg_trees::fenwick::Fenwick;
#[allow(unused)]
use algo_lib::{dbg, out, out_line};

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.usize();
    let q = input.usize();
    let mut a = input.vec::<i32>(n);
    let mut fenw = Fenwick::new(n);
    let mut ones = BTreeSet::new();
    for i in 0..n {
        fenw.add(i, a[i]);
        if a[i] == 1 {
            ones.insert(i);
        }
    }
    for _ in 0..q {
        let q_type = input.i32();
        if q_type == 1 {
            let sum = input.i32();
            let mut ok = false;
            if ones.is_empty() {
                if sum % 2 == 0 {
                    ok = true;
                }
            } else {
                let first = *ones.iter().next().unwrap();
                let last = *ones.iter().next_back().unwrap();
                let tot_sum = fenw.get_sum(n - 1);
                let from_first = fenw.get_suffix_sum(first);
                let from_last = fenw.get_sum(last);
                if from_first >= sum || from_last >= sum {
                    ok = true;
                } else {
                    if tot_sum >= sum && (from_first % 2) == (sum % 2) {
                        ok = true;
                    }
                    if tot_sum >= sum && (from_last % 2) == (sum % 2) {
                        ok = true;
                    }
                }
            }
            if ok {
                out_line!("YES");
            } else {
                out_line!("NO");
            }
        } else {
            assert_eq!(q_type, 2);
            let pos = input.usize() - 1;
            let v = input.i32();
            fenw.add(pos, -a[pos]);
            if a[pos] == 1 {
                ones.remove(&pos);
            }
            a[pos] = v;
            fenw.add(pos, v);
            if v == 1 {
                ones.insert(pos);
            }
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
