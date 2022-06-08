//{"name":"B. Задача каменного века","group":"Codeforces - Codeforces Round #791 (Div. 2)","url":"https://codeforces.com/contest/1679/problem/B","interactive":false,"timeLimit":2000,"tests":[{"input":"5 5\n1 2 3 4 5\n1 1 5\n2 10\n1 5 11\n1 4 1\n2 1\n","output":"19\n50\n51\n42\n5\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"BZadachaKamennogoVeka"}}}

use algo_lib::io::output::output;
use algo_lib::io::task_io_settings::TaskIoType;
use algo_lib::io::task_runner::run_task;
use algo_lib::io::{input::Input, task_io_settings::TaskIoSettings};
use algo_lib::seg_trees::lazy_seg_tree_set_sum::{Node, SegTreeSetSum};
#[allow(unused)]
use algo_lib::{dbg, out, out_line};

type SegTree = SegTreeSetSum<i64>;

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.usize();
    let q = input.usize();
    let a = input.vec::<i64>(n);
    let mut seg_tree = SegTree::new_f(n, &|pos| Node {
        sum: a[pos],
        len: 1,
    });
    for _ in 0..q {
        let cmd = input.usize();
        if cmd == 1 {
            let pos = input.usize() - 1;
            let value = input.i64();
            seg_tree.update(pos..pos + 1, value);
        } else {
            assert_eq!(cmd, 2);
            let value = input.i64();
            seg_tree.update(0..n, value);
        }
        out_line!(seg_tree.get(0..n).sum);
    }
}

pub(crate) fn run(mut input: Input) -> bool {
    solve(&mut input, 1);
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
}
//END MAIN
