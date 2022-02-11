//{"name":"A. Жизни цветов важны","group":"Codeforces - Технокубок 2022 - Отборочный Раунд 3","url":"http://codeforces.com/contest/1585/problem/A","interactive":false,"timeLimit":1000,"tests":[{"input":"4\n3\n1 0 1\n3\n0 1 1\n4\n1 0 0 1\n1\n0\n","output":"3\n7\n-1\n1\n"}],"testType":"multiNumber","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"AZhizniTsvetovVazhni"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::output;
#[allow(unused)]
use algo_lib::{dbg, out, out_line};

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.usize();
    let a = input.read_vec::<i32>(n);
    let mut cur = 1 + a.iter().sum::<i32>();
    for w in a.windows(2) {
        if w[0] == 0 && w[1] == 0 {
            out_line!(-1);
            return;
        }
        if w[0] == 1 && w[1] == 1 {
            cur += 4;
        }
    }
    out_line!(cur);
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
