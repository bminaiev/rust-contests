//{"name":"C. Тоня и Бурёнка-179","group":"Codeforces - Codeforces Round #814 (Div. 1)","url":"https://codeforces.com/contest/1718/problem/C","interactive":false,"timeLimit":3000,"tests":[{"input":"4\n2 1\n1 2\n1 3\n4 4\n4 1 3 2\n2 6\n4 6\n1 1\n3 11\n9 3\n1 7 9 4 5 2 3 6 8\n3 1\n2 1\n9 1\n6 3\n1 1 1 1 1 1\n1 5\n4 4\n3 8\n","output":"3\n5\n14\n16\n24\n24\n24\n57\n54\n36\n36\n6\n18\n27\n28\n"}],"testType":"multiNumber","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"CTonyaIBuryonka179"}}}

use algo_lib::collections::multiset::MultiSet;
use algo_lib::io::output::output;
use algo_lib::io::task_io_settings::TaskIoType;
use algo_lib::io::task_runner::run_task;
use algo_lib::io::{input::Input, task_io_settings::TaskIoSettings};
use algo_lib::math::primes::gen_primes_table;
#[allow(unused)]
use algo_lib::{dbg, out, out_line};

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.usize();
    let q = input.usize();
    let mut a = input.vec::<i64>(n);
    let mut is_prime = gen_primes_table(n + 1);
    let mut steps = vec![];
    for s in 1..n {
        if n % s == 0 && (is_prime[n / s] || s == 1) {
            steps.push(s);
        }
    }
    let mut results = vec![];
    for i in 0..steps.len() {
        results.push(vec![0; steps[i]]);
    }
    for pos in 0..n {
        for i in 0..steps.len() {
            let id = pos % steps[i];
            results[i][id] += a[pos];
        }
    }
    let mut multiset = MultiSet::new();
    for (idx, it) in results.iter().enumerate() {
        let mult = (steps[idx]) as i64;
        for x in it.iter() {
            multiset.insert(*x * mult);
        }
    }
    out_line!(*multiset.last().unwrap());
    for _ in 0..q {
        let pos = input.usize() - 1;
        let value = input.i64();
        let delta = value - a[pos];
        for i in 0..steps.len() {
            let id = pos % steps[i];
            let mult = (steps[i]) as i64;
            multiset.remove(&(results[i][id] * mult));
            results[i][id] += delta;
            multiset.insert(results[i][id] * mult);
        }
        a[pos] = value;
        out_line!(*multiset.last().unwrap());
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
