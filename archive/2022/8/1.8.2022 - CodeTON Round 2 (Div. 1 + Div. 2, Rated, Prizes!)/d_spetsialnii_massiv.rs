//{"name":"D. Специальный массив","group":"Codeforces - CodeTON Round 2 (Div. 1 + Div. 2, Rated, Prizes!)","url":"https://codeforces.com/contest/1704/problem/D","interactive":false,"timeLimit":2000,"tests":[{"input":"7\n3 9\n0 1 2 0 0 2 1 1 0\n0 1 1 1 2 0 0 2 0\n0 1 2 0 0 1 2 1 0\n3 7\n25 15 20 15 25 20 20\n26 14 20 14 26 20 20\n25 15 20 15 20 20 25\n3 9\n25 15 20 15 25 20 20 20 20\n26 14 20 14 26 20 20 20 20\n25 15 20 15 25 15 20 20 25\n3 11\n25 15 20 15 25 20 20 20 20 20 20\n26 14 20 14 26 20 20 20 20 20 20\n25 15 20 15 25 20 15 20 20 20 25\n3 13\n25 15 20 15 25 20 20 20 20 20 20 20 20\n26 14 20 14 26 20 20 20 20 20 20 20 20\n25 15 20 15 25 20 20 15 20 20 20 20 25\n3 15\n25 15 20 15 25 20 20 20 20 20 20 20 20 20 20\n26 14 20 14 26 20 20 20 20 20 20 20 20 20 20\n25 15 20 15 25 20 20 20 15 20 20 20 20 20 25\n3 9\n909459 479492 676924 224197 162866 164495 193268 742456 728277\n948845 455424 731850 327890 304150 237351 251763 225845 798316\n975446 401170 792914 272263 300770 242037 236619 334316 725899\n","output":"3 1\n3 10\n3 15\n3 20\n3 25\n3 30\n1 1378716\n"}],"testType":"multiNumber","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"DSpetsialniiMassiv"}}}

use algo_lib::io::output::output;
use algo_lib::io::task_io_settings::TaskIoType;
use algo_lib::io::task_runner::run_task;
use algo_lib::io::{input::Input, task_io_settings::TaskIoSettings};
use algo_lib::misc::min_max::FindMinMaxPos;
#[allow(unused)]
use algo_lib::{dbg, out, out_line};

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.usize();
    let m = input.usize();
    let mut sums = vec![];
    for _ in 0..n {
        let c = input.vec::<i64>(m);
        let mut sum = 0;
        for pos in 0..m {
            sum += (pos as i64) * c[pos];
        }
        sums.push(sum);
    }
    let min_ix = sums.index_of_min();
    let max_ix = sums.index_of_max();
    out_line!(max_ix + 1, sums[max_ix] - sums[min_ix]);
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
