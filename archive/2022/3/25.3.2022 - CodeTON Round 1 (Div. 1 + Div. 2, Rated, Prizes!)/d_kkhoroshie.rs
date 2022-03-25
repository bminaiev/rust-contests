//{"name":"D. K-хорошие","group":"Codeforces - CodeTON Round 1 (Div. 1 + Div. 2, Rated, Prizes!)","url":"https://codeforces.com/contest/1656/problem/D","interactive":false,"timeLimit":3000,"tests":[{"input":"5\n2\n4\n6\n15\n20\n","output":"-1\n-1\n3\n3\n5\n"}],"testType":"multiNumber","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"DKKhoroshie"}}}

use algo_lib::io::output::output;
use algo_lib::io::task_io_settings::TaskIoType;
use algo_lib::io::task_runner::run_task;
use algo_lib::io::{input::Input, task_io_settings::TaskIoSettings};
#[allow(unused)]
use algo_lib::{dbg, out, out_line};

fn solve_n(n: i64) -> i64 {
    if n.count_ones() == 1 {
        return -1;
    }
    {
        let mut k = 2;
        while k <= n {
            let sum = (1 + k) * k / 2;
            if sum > n {
                break;
            }
            if (n - sum) % k == 0 {
                return k;
                // return k;
            }
            k *= 2;
        }
    }
    {
        let mut k = n;
        while k % 2 == 0 {
            k /= 2;
        }
        let sum = (1 + k) * k / 2;
        if sum <= n && (n - sum) % k == 0 {
            return k;
            // return k;
        }
    }
    // for k in 2..=n {
    //     let sum = (1 + k) * k / 2;
    //     if sum <= n && (n - sum) % k == 0 {
    //         dbg!(k);
    //         return k;
    //     }
    // }
    unreachable!();
}

fn stress() {
    for n in 2..100000000 {
        let res = solve_n(n);

        if res > 20 && res.count_ones() != 1 {
            let r = solve_n(n);
        }
    }
    // solve_n(9801728);
    // const BIG: i64 = 1e18 as i64;
    // for n in (0..BIG).rev() {
    //     dbg!(n, solve_n(n));
    // }
}

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.i64();
    out_line!(solve_n(n));
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
    // tester::run_stress(stress);
    // tester::run_single_test("1");
}
//END MAIN
