//{"name":"D. Reverse Madness","group":"Codeforces - Codeforces Round 900 (Div. 3)","url":"https://codeforces.com/contest/1878/problem/D","interactive":false,"timeLimit":1000,"tests":[{"input":"5\n4 2\nabcd\n1 3\n2 4\n2\n1 3\n5 3\nabcde\n1 2 3\n1 2 5\n3\n1 2 3\n3 1\ngaf\n1\n3\n2\n2 2\n10 1\naghcdegdij\n1\n10\n5\n1 2 3 4 2\n1 1\na\n1\n1\n1\n1\n","output":"badc\nabedc\ngaf\njihgedcdga\na\n"}],"testType":"multiNumber","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"DReverseMadness"}}}

use std::cmp::min;

use algo_lib::io::output::output;
use algo_lib::io::task_io_settings::TaskIoType;
use algo_lib::io::task_runner::run_task;
use algo_lib::io::{input::Input, task_io_settings::TaskIoSettings};
use algo_lib::misc::binary_search::binary_search_first_true;
#[allow(unused)]
use algo_lib::{dbg, out, out_line};

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.usize();
    let k = input.usize();
    let mut s = input.string();
    let mut rev = vec![false; n];
    let l = input.vec::<usize>(k);
    let r = input.vec::<usize>(k);
    let q = input.usize();
    for _ in 0..q {
        let x = input.usize();
        let i = binary_search_first_true(0..k, |i| r[i] >= x);
        assert!(r[i] >= x);
        let a = min(x, l[i] + r[i] - x) - 1;
        // let b = max(x, l[i] + r[i] - x) - 1;
        rev[a] ^= true;
    }
    for i in 0..k {
        let mut li = l[i] - 1;
        let mut ri = r[i] - 1;
        while li < ri {
            if rev[li] {
                s.swap(li, ri);
                if li + 1 < ri {
                    rev[li + 1] ^= true;
                }
            }
            li += 1;
            ri -= 1;
        }
    }
    let s = String::from_utf8(s).unwrap();
    out_line!(s);
}

pub(crate) fn run(mut input: Input) -> bool {
    let t = input.read();
    for i in 0usize..t {
        solve(&mut input, i + 1);
    }
    output().flush();
    true
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
    // tester::run_locally();
    // tester::run_with_last_downloaded_file();
}
//END MAIN
