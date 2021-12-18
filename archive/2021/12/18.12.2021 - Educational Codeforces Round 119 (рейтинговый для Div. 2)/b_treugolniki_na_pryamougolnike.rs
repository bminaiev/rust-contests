//{"name":"B. Треугольники на прямоугольнике","group":"Codeforces - Educational Codeforces Round 119 (рейтинговый для Div. 2)","url":"https://codeforces.com/contest/1620/problem/B","interactive":false,"timeLimit":2000,"tests":[{"input":"3\n5 8\n2 1 2\n3 2 3 4\n3 1 4 6\n2 4 5\n10 7\n2 3 9\n2 1 7\n3 1 3 4\n3 4 5 6\n11 5\n3 1 6 8\n3 3 6 8\n3 1 3 4\n2 2 4\n","output":"25\n42\n35\n"}],"testType":"multiNumber","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"BTreugolnikiNaPryamougolnike"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::misc::min_max::UpdateMinMax;
use algo_lib::{dbg, out, out_line};

fn solve(input: &mut Input, _test_case: usize) {
    let w = input.i64();
    let h = input.i64();
    let mut res = 0;
    for _ in 0..2 {
        let cnt = input.usize();
        let v: Vec<i64> = input.read_vec(cnt);
        res.update_max((v.last().unwrap() - v[0]) * h);
    }
    for _ in 0..2 {
        let cnt = input.usize();
        let v: Vec<i64> = input.read_vec(cnt);
        res.update_max((v.last().unwrap() - v[0]) * w);
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
    !input.peek().is_some()
}

//START MAIN
mod tester;

fn main() {
    tester::run_tests();
}
//END MAIN
