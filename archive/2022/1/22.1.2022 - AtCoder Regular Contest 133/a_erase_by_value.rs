//{"name":"A - Erase by Value","group":"AtCoder - AtCoder Regular Contest 133","url":"https://atcoder.jp/contests/arc133/tasks/arc133_a","interactive":false,"timeLimit":2000,"tests":[{"input":"5\n2 4 4 1 2\n","output":"2 1 2\n"},{"input":"3\n1 1 1\n","output":"\n"},{"input":"5\n1 1 2 3 3\n","output":"1 1 2\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"AEraseByValue"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::{dbg, out, out_line};

fn solve(input: &mut Input) {
    let n = input.usize();
    let a = input.read_vec::<i32>(n);
    let mut pos = n - 1;
    for i in 0..n - 1 {
        if a[i] > a[i + 1] {
            pos = i;
            break;
        }
    }
    let remove = a[pos];
    let res: Vec<_> = a.into_iter().filter(|&x| x != remove).collect();
    out_line!(res);
}

pub(crate) fn run(mut input: Input) -> bool {
    solve(&mut input);
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
