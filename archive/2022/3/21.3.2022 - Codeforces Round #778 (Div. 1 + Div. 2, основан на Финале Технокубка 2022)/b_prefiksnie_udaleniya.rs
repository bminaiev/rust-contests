//{"name":"B. Префиксные удаления","group":"Codeforces - Codeforces Round #778 (Div. 1 + Div. 2, основан на Финале Технокубка 2022)","url":"http://codeforces.com/contest/1654/problem/B","interactive":false,"timeLimit":2000,"tests":[{"input":"6\nabcabdc\na\nbbbbbbbbbb\ncodeforces\ncffcfccffccfcffcfccfcffccffcfccf\nzyzyzwxxyyxxyyzzyzzxxwzxwywxwzxxyzzw\n","output":"abdc\na\nb\ndeforces\ncf\nxyzzw\n"}],"testType":"multiNumber","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"BPrefiksnieUdaleniya"}}}

use algo_lib::io::output::output;
use algo_lib::io::task_io_settings::TaskIoType;
use algo_lib::io::task_runner::run_task;
use algo_lib::io::{input::Input, task_io_settings::TaskIoSettings};
use algo_lib::strings::utils::vec2str;
#[allow(unused)]
use algo_lib::{dbg, out, out_line};

fn solve(input: &mut Input, _test_case: usize) {
    let s = input.string();
    let mut seen = vec![false; 26];
    let mut last_ok_pos = s.len();
    for (pos, &c) in s.iter().enumerate().rev() {
        let c = (c - b'a') as usize;
        if !seen[c] {
            last_ok_pos = pos;
            seen[c] = true;
        }
    }
    let res = vec2str(&s[last_ok_pos..]);
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
