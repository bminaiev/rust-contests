//{"name":"F. Fighter","group":"Yandex - SNWS-2022, Round 2","url":"https://contest.yandex.ru/snws2022/contest/23958/problems/F/","interactive":false,"timeLimit":2000,"tests":[{"input":"4 3\n+ 111\n- 101\n+ 001\n- 111\n","output":"1\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"FFighter"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::{dbg, out, out_line};

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.usize();
    let m = input.usize();
    let mut know_more = 0i64;
    let mut res = 0;
    for _ in 0..n {
        let c = input.string_as_vec()[0];
        let mut check = 0;
        let s = input.string_as_vec();
        for i in 0..m {
            if s[i] == b'1' {
                check |= 1i64 << i;
            }
        }
        if c == b'-' {
            if (know_more & check) != check {
                res += 1;
            }
            know_more &= !check;
        } else {
            know_more |= check;
        }
    }
    out_line!(res);
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
