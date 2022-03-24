//{"name":"M. Monarchy in Vertigo","group":"Codeforces - 2021-2022 ACM-ICPC Brazil Subregional Programming Contest","url":"https://codeforces.com/gym/103388/problem/M","interactive":false,"timeLimit":250,"tests":[{"input":"8\n1 1\n1 1\n1 2\n2 1\n2 4\n1 2\n2 2\n2 5\n","output":"2\n2\n5\n3\n"},{"input":"4\n1 1\n1 1\n2 2\n2 1\n","output":"1\n3\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"MMonarchyInVertigo"}}}

use algo_lib::io::output::output;
use algo_lib::io::task_io_settings::TaskIoType;
use algo_lib::io::task_runner::run_task;
use algo_lib::io::{input::Input, task_io_settings::TaskIoSettings};
#[allow(unused)]
use algo_lib::{dbg, out, out_line};

fn solve(input: &mut Input, _test_case: usize) {
    let q = input.usize();
    let mut iters = vec![0];
    let mut is_alive = vec![true];
    let mut cur_monarch = 0;
    let mut parents = vec![0];
    let mut child = vec![vec![]];
    for _ in 0..q {
        let q_type = input.usize();
        if q_type == 1 {
            let id = parents.len();
            let parent = input.usize() - 1;
            child[parent].push(id);
            parents.push(parent);
            is_alive.push(true);
            iters.push(0);
            child.push(vec![]);
        } else {
            assert_eq!(q_type, 2);
            let v = input.usize() - 1;
            is_alive[v] = false;
            while !is_alive[cur_monarch] {
                if iters[cur_monarch] < child[cur_monarch].len() {
                    iters[cur_monarch] += 1;
                    cur_monarch = child[cur_monarch][iters[cur_monarch] - 1];
                } else {
                    cur_monarch = parents[cur_monarch];
                }
            }
            out_line!(cur_monarch + 1);
        }
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
