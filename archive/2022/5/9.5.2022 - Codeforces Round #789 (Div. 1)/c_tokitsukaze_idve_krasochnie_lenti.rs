//{"name":"C. Tokitsukaze и две красочные ленты","group":"Codeforces - Codeforces Round #789 (Div. 1)","url":"https://codeforces.com/contest/1677/problem/C","interactive":false,"timeLimit":2000,"tests":[{"input":"3\n6\n1 5 4 3 2 6\n5 3 1 4 6 2\n6\n3 5 4 6 2 1\n3 6 4 5 2 1\n1\n1\n1\n","output":"18\n10\n0\n"}],"testType":"multiNumber","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"CTokitsukazeIDveKrasochnieLenti"}}}

use algo_lib::io::output::output;
use algo_lib::io::task_io_settings::TaskIoType;
use algo_lib::io::task_runner::run_task;
use algo_lib::io::{input::Input, task_io_settings::TaskIoSettings};
use algo_lib::misc::vec_apply_delta::ApplyDelta;
#[allow(unused)]
use algo_lib::{dbg, out, out_line};

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.usize();
    let p1 = input.vec::<usize>(n).sub_from_all(1);
    let p2 = input.vec::<usize>(n).sub_from_all(1);
    let mut where_in_2 = vec![0; n];
    for i in 0..n {
        where_in_2[p2[i]] = i;
    }
    let mut seen = vec![false; n];
    let mut lengths = vec![];
    for v in 0..n {
        if seen[v] {
            continue;
        }
        let mut cur_pos = v;
        let mut len = 0;
        while !seen[cur_pos] {
            seen[cur_pos] = true;
            len += 1;
            cur_pos = where_in_2[p1[cur_pos]];
        }
        lengths.push(len);
    }
    lengths.sort();
    let mut res = 0;
    let mut cnt = n;
    for &l in lengths.iter() {
        let mut cur_l = 0;
        let mut cur_r = cnt - 1;
        for it in 0..(l - 1) {
            res += (cur_r - cur_l) as i64;
            if it % 2 == 0 {
                cur_l += 1;
            } else {
                cur_r -= 1;
            }
        }
        if l % 2 == 0 {
            cnt -= l;
            res += (cur_r) as i64;
        } else {
            cnt -= l - 1;
            res += cur_l as i64;
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
}
//END MAIN
