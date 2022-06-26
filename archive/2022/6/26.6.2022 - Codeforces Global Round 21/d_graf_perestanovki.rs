//{"name":"D. Граф перестановки","group":"Codeforces - Codeforces Global Round 21","url":"https://codeforces.com/contest/1696/problem/D","interactive":false,"timeLimit":2000,"tests":[{"input":"5\n1\n1\n2\n1 2\n5\n1 4 2 3 5\n5\n2 1 5 3 4\n10\n7 4 8 1 6 10 3 5 2 9\n","output":"0\n1\n1\n4\n6\n"}],"testType":"multiNumber","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"DGrafPerestanovki"}}}

use algo_lib::io::output::output;
use algo_lib::io::task_io_settings::TaskIoType;
use algo_lib::io::task_runner::run_task;
use algo_lib::io::{input::Input, task_io_settings::TaskIoSettings};
use algo_lib::misc::vec_apply_delta::ApplyDelta;
use algo_lib::seg_trees::lazy_seg_tree_max::{MaxValNode, SegTreeMax};
#[allow(unused)]
use algo_lib::{dbg, out, out_line};

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.usize();
    let a = input.vec::<i32>(n).sub_from_all(1);
    let mut st_max = SegTreeMax::new_f(n, &|pos| MaxValNode {
        pos,
        max_val: a[pos],
    });
    let mut st_min = SegTreeMax::new_f(n, &|pos| MaxValNode {
        pos,
        max_val: -a[pos],
    });
    let pos0 = a.iter().position(|x| *x == 0).unwrap();
    let mut res = 0;
    {
        let mut pos = pos0;
        let mut search_for_max = true;
        while pos != n - 1 {
            let next_pos = if search_for_max {
                st_max.get(pos + 1..n).pos
            } else {
                st_min.get(pos + 1..n).pos
            };
            pos = next_pos;
            search_for_max = !search_for_max;
            res += 1;
        }
    }
    {
        let mut pos = pos0;
        let mut search_for_max = true;
        while pos != 0 {
            let next_pos = if search_for_max {
                st_max.get(0..pos).pos
            } else {
                st_min.get(0..pos).pos
            };
            pos = next_pos;
            search_for_max = !search_for_max;
            res += 1;
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
    tester::run_tests();
    // tester::run_single_test("1");
    // tester::run_stress(stress);
}
//END MAIN
