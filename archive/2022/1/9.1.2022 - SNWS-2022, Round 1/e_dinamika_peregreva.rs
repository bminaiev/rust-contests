//{"name":"E. Динамика перегрева","group":"Yandex - SNWS-2022, Round 1","url":"https://contest.yandex.ru/snws2022/contest/23957/problems/E/","interactive":false,"timeLimit":2000,"tests":[{"input":"2 2\n2 0\n-6 11\n1 1 6\n1 2 6\n","output":"12\n-25\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"EDinamikaPeregreva"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::misc::min_max::UpdateMinMax;
use algo_lib::{dbg, out, out_line};

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.usize();
    let q = input.usize();
    let mut a = vec![];
    let mut b = vec![];
    for _ in 0..n {
        a.push(input.i64());
        b.push(input.i64());
    }
    for _ in 0..q {
        let fr = input.usize() - 1;
        let to = input.usize();
        let d = input.i64();
        let mut res = i64::MAX;
        for i in fr..to {
            res.update_min(a[i] * d + b[i]);
        }
        out_line!(res);
    }
}

pub(crate) fn run(mut input: Input) -> bool {
    solve(&mut input, 1);
    output().flush();
    input.skip_whitespace();
    input.peek().is_none()
}

//START MAIN
mod tester;

fn main() {
    tester::run_tests();
}
//END MAIN
