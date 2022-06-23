//{"name":"F. Я могу ошибаться","group":"Codeforces - Codeforces Round #800 (Div. 1)","url":"https://codeforces.com/contest/1693/problem/F","interactive":false,"timeLimit":2000,"tests":[{"input":"7\n1\n1\n2\n10\n3\n101\n4\n1000\n5\n11010\n6\n110000\n20\n01000010001010011000\n","output":"0\n1\n1\n3\n2\n2\n5\n"}],"testType":"multiNumber","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"FYaMoguOshibatsya"}}}

use algo_lib::collections::last_exn::LastExn;
use algo_lib::io::output::output;
use algo_lib::io::task_io_settings::TaskIoType;
use algo_lib::io::task_runner::run_task;
use algo_lib::io::{input::Input, task_io_settings::TaskIoSettings};
#[allow(unused)]
use algo_lib::{dbg, out, out_line};

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct Pref {
    balance: i32,
    pos: usize,
}

fn solve_case(a: &mut [i32]) -> usize {
    let cnt1 = a.iter().filter(|&x| *x == 1).count();
    if cnt1 * 2 > a.len() {
        let mut b: Vec<_> = a.iter().map(|x| -*x).collect();
        b.reverse();
        return solve_case(&mut b);
    }
    if cnt1 == 0 {
        return 0;
    }
    let mut stack: Vec<Pref> = vec![];
    let n = a.len();
    let mut cur = 0;
    for pos in 0..n {
        cur += a[pos];
        while !stack.is_empty() && stack.last_exn().balance <= cur {
            stack.pop();
        }
        stack.push(Pref { pos, balance: cur });
    }

    let mut res = 0;
    let mut first_one = 0;
    let mut cur_balance = 0;
    let mut it = 0;
    loop {
        while a[first_one] == -1 {
            cur_balance += a[first_one];
            first_one += 1;
        }
        if first_one + cnt1 as usize == a.len() {
            return res;
        }
        res += 1;

        while it + 1 < stack.len() && stack[it + 1].balance >= cur_balance {
            it += 1;
        }

        let till = stack[it].pos + 1;
        assert!(till > first_one);
        a[first_one..till].sort();
    }
}

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.usize();
    let s = input.string();
    let mut a: Vec<_> = s
        .into_iter()
        .map(|x| if x == b'1' { 1 } else { -1 })
        .collect();
    let res = solve_case(&mut a);
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
    // tester::run_stress(stress);
}
//END MAIN
