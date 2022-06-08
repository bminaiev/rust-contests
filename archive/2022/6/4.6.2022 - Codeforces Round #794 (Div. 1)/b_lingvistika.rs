//{"name":"B. Лингвистика","group":"Codeforces - Codeforces Round #794 (Div. 1)","url":"https://codeforces.com/contest/1685/problem/B","interactive":false,"timeLimit":1000,"tests":[{"input":"8\n1 0 0 0\nB\n0 0 1 0\nAB\n1 1 0 1\nABAB\n1 0 1 1\nABAAB\n1 1 2 2\nBAABBABBAA\n1 1 2 3\nABABABBAABAB\n2 3 5 4\nAABAABBABAAABABBABBBABB\n1 3 3 10\nBBABABABABBBABABABABABABAABABA\n","output":"NO\nYES\nYES\nYES\nYES\nYES\nNO\nYES\n"}],"testType":"multiNumber","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"BLingvistika"}}}

use std::cmp::min;

use algo_lib::io::output::output;
use algo_lib::io::task_io_settings::TaskIoType;
use algo_lib::io::task_runner::run_task;
use algo_lib::io::{input::Input, task_io_settings::TaskIoSettings};
#[allow(unused)]
use algo_lib::{dbg, out, out_line};

fn can(a: usize, b: usize, ab: usize, ba: usize, s: &[u8]) -> bool {
    let have_a = a + ab + ba;
    let have_b = b + ab + ba;
    let need_a = s.iter().filter(|&&x| x == b'A').count();
    let need_b = s.iter().filter(|&&x| x == b'B').count();
    if have_a != need_a || have_b != need_b {
        return false;
    }
    let mut i = 0;
    let mut any = 0;
    let mut only_ab = vec![];
    let mut only_ba = vec![];
    while i + 1 < s.len() {
        if s[i] == s[i + 1] {
            i += 1;
            continue;
        }
        let mut j = i + 1;
        while j + 1 != s.len() && s[j] != s[j + 1] {
            j += 1;
        }
        let len = j - i + 1;
        let half = len / 2;
        if len % 2 == 1 {
            any += half;
        } else {
            if s[i] == b'A' {
                only_ab.push(half);
            } else {
                only_ba.push(half);
            }
        }
        i = j;
    }
    only_ab.sort();
    only_ba.sort();

    let mut more_ab = ab;
    let mut more_ba = ba;

    let mut it_ab = 0;
    while it_ab != only_ab.len() {
        let cur = only_ab[it_ab];
        if cur <= more_ab {
            more_ab -= cur;
            it_ab += 1;
        } else {
            let used = min(cur, more_ab);
            let more = only_ab[it_ab] - used - 1;
            any += more;
            it_ab += 1;
            more_ab = 0;
            break;
        }
    }
    while it_ab != only_ab.len() {
        any += only_ab[it_ab] - 1;
        it_ab += 1;
    }

    let mut it_ba = 0;
    while it_ba != only_ba.len() {
        let cur = only_ba[it_ba];
        if cur <= more_ba {
            more_ba -= cur;
            it_ba += 1;
        } else {
            let used = min(cur, more_ba);
            let more = only_ba[it_ba] - used - 1;
            any += more;
            it_ba += 1;
            more_ba = 0;
            break;
        }
    }
    while it_ba != only_ba.len() {
        any += only_ba[it_ba] - 1;
        it_ba += 1;
    }

    let used = min(any, more_ab);
    more_ab -= used;
    any -= used;

    let used = min(any, more_ba);
    more_ba -= used;
    any -= used;

    more_ab == 0 && more_ba == 0
}

fn solve(input: &mut Input, _test_case: usize) {
    let a = input.usize();
    let b = input.usize();
    let ab = input.usize();
    let ba = input.usize();
    let s = input.string();
    if can(a, b, ab, ba, &s) {
        out_line!("YES");
    } else {
        out_line!("NO");
    }
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
