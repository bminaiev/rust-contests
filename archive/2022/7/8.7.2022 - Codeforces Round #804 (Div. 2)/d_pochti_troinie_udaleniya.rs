//{"name":"D. Почти тройные удаления","group":"Codeforces - Codeforces Round #804 (Div. 2)","url":"https://codeforces.com/contest/1699/problem/D","interactive":false,"timeLimit":2000,"tests":[{"input":"5\n7\n1 2 3 2 1 3 3\n1\n1\n6\n1 1 1 2 2 2\n8\n1 1 2 2 3 3 1 1\n12\n1 5 2 3 3 3 4 4 4 4 3 3\n","output":"3\n1\n0\n4\n2\n"}],"testType":"multiNumber","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"DPochtiTroinieUdaleniya"}}}

use algo_lib::collections::array_2d::Array2D;
use algo_lib::collections::last_exn::LastExn;
use algo_lib::io::output::output;
use algo_lib::io::task_io_settings::TaskIoType;
use algo_lib::io::task_runner::run_task;
use algo_lib::io::{input::Input, task_io_settings::TaskIoSettings};
use algo_lib::misc::min_max::UpdateMinMax;
use algo_lib::misc::vec_apply_delta::ApplyDelta;
#[allow(unused)]
use algo_lib::{dbg, out, out_line};

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.usize();
    let a = input.vec::<usize>(n).sub_from_all(1);
    let mut positions = vec![vec![]; n];
    for pos in 0..n {
        positions[a[pos]].push(pos);
    }
    for i in 0..n {
        positions[i].push(n);
    }
    let mut res = 0;
    let mut cost = Array2D::new(std::i32::MAX / 2, n + 1, n + 1);
    for start in 0..n {
        let mut value = 0;
        let mut cnt = 0;
        let mut seen = vec![0; n];
        for end in start..=n {
            let len = end - start;
            let another = len - seen[value];
            if another < seen[value] {
                cost[start][end] = (seen[value] - another) as i32;
            } else {
                cost[start][end] = (len % 2) as i32;
            }
            if end != n {
                seen[a[end]] += 1;
                if cnt == 0 {
                    value = a[end];
                }
                if a[end] != value {
                    cnt -= 1;
                } else {
                    cnt += 1;
                }
            }
        }
    }
    cost[n][n] = 0;
    for left_color in 0..n {
        let positions = &positions[left_color];
        // dp[i] = stay just before positions[i]
        let mut dp = vec![std::i32::MIN / 2; positions.len()];
        for i in 0..dp.len() {
            dp[i] = -cost[0][positions[i]];
        }
        for i in 0..positions.len() {
            for j in i + 1..positions.len() {
                let my_cnt = (j - i) as i32;
                let len = (positions[j - 1] - positions[i] + 1) as i32;
                let after = cost[positions[j - 1] + 1][positions[j]];
                let add = my_cnt * 2 - len - after;
                let check = dp[i] + add;
                dp[j].update_max(check);
            }
        }
        res.update_max(*dp.last_exn());
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
    // tester::run_single_test("2");
    // tester::run_stress(stress);
}
//END MAIN
