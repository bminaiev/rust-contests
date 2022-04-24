//{"name":"F1. Перемешивание массива","group":"Codeforces - Codeforces Global Round 20","url":"https://codeforces.com/contest/1672/problem/F1","interactive":false,"timeLimit":1000,"tests":[{"input":"2\n2\n2 1\n4\n1 2 3 3\n","output":"1 2\n3 3 2 1\n"}],"testType":"multiNumber","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"F1PeremeshivanieMassiva"}}}

use algo_lib::io::output::output;
use algo_lib::io::task_io_settings::TaskIoType;
use algo_lib::io::task_runner::run_task;
use algo_lib::io::{input::Input, task_io_settings::TaskIoSettings};
use algo_lib::misc::func::id;
use algo_lib::misc::gen_vector::gen_vec;
use algo_lib::misc::vec_apply_delta::ApplyDelta;
#[allow(unused)]
use algo_lib::{dbg, out, out_line};

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.usize();
    let a = input.vec::<usize>(n).sub_from_all(1);
    let mut pos = vec![vec![]; n];
    for i in 0..n {
        pos[a[i]].push(i);
    }
    let mut by_freq = gen_vec(n, id);
    by_freq.sort_by_key(|x| pos[*x].len());
    let mut iter = 0;
    let mut more = gen_vec(n, |id| pos[id].len());
    let mut res = vec![n + 1; n];
    loop {
        while iter != n && more[by_freq[iter]] == 0 {
            iter += 1;
        }
        if iter == n {
            break;
        }
        let mut cur_positions = vec![];
        let mut cur_values = vec![];
        for &value in by_freq[iter..].iter() {
            more[value] -= 1;
            let now_pos = pos[value].pop().unwrap();
            cur_positions.push(now_pos);
            cur_values.push(value);
        }
        for i in 0..cur_positions.len() {
            res[cur_positions[i]] = cur_values[(i + 1) % cur_values.len()] + 1;
        }
    }
    for i in 0..n {
        assert_ne!(res[i], n + 1);
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
}
//END MAIN
