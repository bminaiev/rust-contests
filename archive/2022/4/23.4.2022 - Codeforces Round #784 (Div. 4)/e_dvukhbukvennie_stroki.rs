//{"name":"E. Двухбуквенные строки","group":"Codeforces - Codeforces Round #784 (Div. 4)","url":"https://codeforces.com/contest/1669/problem/E","interactive":false,"timeLimit":2000,"tests":[{"input":"4\n6\nab\ncb\ndb\naa\ncc\nef\n7\naa\nbb\ncc\nac\nca\nbb\naa\n4\nkk\nkk\nab\nab\n5\njf\njf\njk\njk\njk\n","output":"5\n6\n0\n6\n"}],"testType":"multiNumber","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"EDvukhbukvennieStroki"}}}

use algo_lib::collections::array_2d::Array2D;
use algo_lib::io::output::output;
use algo_lib::io::task_io_settings::TaskIoType;
use algo_lib::io::task_runner::run_task;
use algo_lib::io::{input::Input, task_io_settings::TaskIoSettings};
#[allow(unused)]
use algo_lib::{dbg, out, out_line};

fn solve(input: &mut Input, _test_case: usize) {
    const N: usize = 26;
    let n = input.usize();
    let mut res = 0i64;
    let mut f1 = vec![0; N];
    let mut f2 = vec![0; N];
    let mut ff = Array2D::new(0, N, N);
    for _ in 0..n {
        let s = input.string();
        let c1 = (s[0] - b'a') as usize;
        let c2 = (s[1] - b'a') as usize;
        res += f1[c1];
        res += f2[c2];
        res -= ff[c1][c2] * 2;
        f1[c1] += 1;
        f2[c2] += 1;
        ff[c1][c2] += 1;
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
