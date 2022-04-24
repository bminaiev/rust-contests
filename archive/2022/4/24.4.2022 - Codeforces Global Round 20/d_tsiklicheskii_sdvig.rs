//{"name":"D. Циклический сдвиг","group":"Codeforces - Codeforces Global Round 20","url":"https://codeforces.com/contest/1672/problem/D","interactive":false,"timeLimit":1000,"tests":[{"input":"5\n5\n1 2 3 3 2\n1 3 3 2 2\n5\n1 2 4 2 1\n4 2 2 1 1\n5\n2 4 5 5 2\n2 2 4 5 5\n3\n1 2 3\n1 2 3\n3\n1 1 2\n2 1 1\n","output":"YES\nYES\nNO\nYES\nNO\n"}],"testType":"multiNumber","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"DTsiklicheskiiSdvig"}}}

use std::collections::HashMap;

use algo_lib::io::output::output;
use algo_lib::io::task_io_settings::TaskIoType;
use algo_lib::io::task_runner::run_task;
use algo_lib::io::{input::Input, task_io_settings::TaskIoSettings};
use algo_lib::misc::gen_vector::gen_vec;
use algo_lib::misc::rand::Random;
#[allow(unused)]
use algo_lib::{dbg, out, out_line};

fn can(a: &[usize], b: &[usize]) -> bool {
    let n = a.len();
    let mut balance = HashMap::<_, i32>::new();
    let mut iter = n - 1;
    for pos in (0..n).rev() {
        if a[iter] == b[pos] {
            if iter != 0 {
                iter -= 1;
            }
        } else {
            if pos + 1 < n && b[pos + 1] == b[pos] {
                *balance.entry(b[pos]).or_default() += 1;
            } else {
                while a[iter] != b[pos] {
                    let entry = balance.entry(a[iter]).or_default();
                    if *entry <= 0 {
                        return false;
                    }
                    *entry -= 1;
                    iter -= 1;
                }
                if iter != 0 {
                    iter -= 1;
                }
            }
        }
    }
    true
}

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.usize();
    let a = input.vec::<usize>(n);
    let b = input.vec::<usize>(n);
    if can(&a, &b) {
        out_line!("YES");
    } else {
        out_line!("NO");
    }
}

fn stress() {
    for it in 11.. {
        let mut rnd = Random::new(it);
        dbg!(it);
        const MAX: usize = 10;
        let n = rnd.gen_in_range(2..MAX);
        let a = gen_vec(n, |_| rnd.gen_in_range(0..MAX));
        let mut b = a.clone();
        for _ in 0..10 {
            let idx = rnd.gen_in_range(0..n - 1);
            let idx2 = rnd.gen_in_range(idx + 1..n);
            if b[idx] == b[idx2] {
                b[idx..idx2].rotate_left(1);
            }
            if !(can(&a, &b)) {
                dbg!(a);
                dbg!(b);
                assert!(false);
            }
        }
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
    // tester::run_stress(stress);
}
//END MAIN
