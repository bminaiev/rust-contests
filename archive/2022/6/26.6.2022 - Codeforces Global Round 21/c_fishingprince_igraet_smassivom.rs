//{"name":"C. Fishingprince играет с массивом","group":"Codeforces - Codeforces Global Round 21","url":"https://codeforces.com/contest/1696/problem/C","interactive":false,"timeLimit":2000,"tests":[{"input":"5\n5 2\n1 2 2 4 2\n4\n1 4 4 2\n6 2\n1 2 2 8 2 2\n2\n1 16\n8 3\n3 3 3 3 3 3 3 3\n4\n6 6 6 6\n8 3\n3 9 6 3 12 12 36 12\n16\n9 3 2 2 2 3 4 12 4 12 4 12 4 12 4 4\n8 3\n3 9 6 3 12 12 36 12\n7\n12 2 4 3 4 12 56\n","output":"Yes\nYes\nNo\nYes\nNo\n"}],"testType":"multiNumber","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"CFishingprinceIgraetSMassivom"}}}

use algo_lib::collections::last_exn::LastExn;
use algo_lib::io::output::output;
use algo_lib::io::task_io_settings::TaskIoType;
use algo_lib::io::task_runner::run_task;
use algo_lib::io::{input::Input, task_io_settings::TaskIoSettings};
#[allow(unused)]
use algo_lib::{dbg, out, out_line};

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Debug)]
struct Elem {
    value: i64,
    cnt: i64,
}

fn canon(a: &[i64], m: i64) -> Vec<Elem> {
    let mut res = vec![Elem { value: 0, cnt: 0 }];
    for &x in a.iter() {
        let mut x = x;
        let mut cnt = 1;
        while x % m == 0 {
            x /= m;
            cnt *= m;
        }
        if res[res.len() - 1].value == x {
            let pos = res.len() - 1;
            res[pos].cnt += cnt;
        } else {
            res.push(Elem { value: x, cnt });
        }
    }
    res
}

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.usize();
    let m = input.i64();
    let a = input.vec::<i64>(n);
    let k = input.usize();
    let b = input.vec::<i64>(k);
    if canon(&a, m) == canon(&b, m) {
        out_line!("Yes");
    } else {
        out_line!("No");
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
    // tester::run_single_test("2");
    // tester::run_stress(stress);
}
//END MAIN
