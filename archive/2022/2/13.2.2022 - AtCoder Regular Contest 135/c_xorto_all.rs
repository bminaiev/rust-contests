//{"name":"C - XOR to All","group":"AtCoder - AtCoder Regular Contest 135","url":"https://atcoder.jp/contests/arc135/tasks/arc135_c","interactive":false,"timeLimit":2000,"tests":[{"input":"5\n1 2 3 4 5\n","output":"19\n"},{"input":"5\n10 10 10 10 10\n","output":"50\n"},{"input":"5\n3 1 4 1 5\n","output":"18\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"CXORToAll"}}}

use algo_lib::collections::array_2d::Array2D;
use algo_lib::io::output::output;
use algo_lib::io::task_io_settings::TaskIoType;
use algo_lib::io::task_runner::run_task;
use algo_lib::io::{input::Input, task_io_settings::TaskIoSettings};
use algo_lib::misc::min_max::UpdateMinMax;
#[allow(unused)]
use algo_lib::{dbg, out, out_line};

fn solve(input: &mut Input) {
    let n = input.usize();
    let a = input.read_vec::<i64>(n);
    let mut res: i64 = a.iter().sum();
    const BITS: usize = 31;
    let mut by_bit = Array2D::new(0, BITS, 2);
    for &val in a.iter() {
        for bit in 0..BITS {
            let bit_value = ((val >> bit) & 1) as usize;
            by_bit[bit][bit_value] += 1;
        }
    }
    for &val in a.iter() {
        let mut cur_sum = 0;
        for bit in 0..BITS {
            let bit_value = ((val >> bit) & 1) as usize;
            cur_sum += (1i64 << bit) * by_bit[bit][1 - bit_value];
        }
        res.update_max(cur_sum);
    }
    out_line!(res);
}

pub(crate) fn run(mut input: Input) -> bool {
    solve(&mut input);
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
