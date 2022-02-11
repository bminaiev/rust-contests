//{"name":"B. Выворачивание массива","group":"Codeforces - Технокубок 2022 - Отборочный Раунд 3","url":"http://codeforces.com/contest/1585/problem/B","interactive":false,"timeLimit":1000,"tests":[{"input":"3\n5\n2 4 1 5 3\n5\n5 3 2 4 1\n4\n1 1 1 1\n","output":"1\n2\n0\n"}],"testType":"multiNumber","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"BVivorachivanieMassiva"}}}

use algo_lib::io::output::output;
use algo_lib::{collections::reversed::ReversedTrait, io::input::Input};
#[allow(unused)]
use algo_lib::{dbg, out, out_line};

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.usize();
    let a = input.read_vec::<i32>(n).reversed();
    let mut max = 0;
    let mut res = 0;
    for &x in a.iter() {
        if x > max {
            res += 1;
            max = x;
        }
    }
    out_line!(res - 1);
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

//START MAIN
mod tester;

fn main() {
    tester::run_tests();
    // tester::run_single_test("1");
}
//END MAIN
